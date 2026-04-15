use std::path::PathBuf;
use tokio::net::UnixStream;
use tokio_util::codec::{Framed, LinesCodec};
use futures::{StreamExt, SinkExt};
use std::time::Duration;
use tokio::time;
use crate::error::{Result, VibeError};
use crate::ipc::protocol::{Message, RegisterInfo, HeartbeatInfo, ExitStatusInfo, ExecuteIntentInfo, ReportInfo};
use dialoguer::Confirm;

#[derive(Clone)]
pub struct WorkerClient {
    socket_path: PathBuf,
    vibe_id: String,
    physical_id: String,
    terminal_type: String,
    role: Option<String>,
    trusted: bool,
}

impl WorkerClient {
    pub fn new(
        socket_path: PathBuf,
        vibe_id: String,
        physical_id: String,
        terminal_type: String,
        role: Option<String>,
    ) -> Self {
        Self {
            socket_path,
            vibe_id,
            physical_id,
            terminal_type,
            role,
            trusted: false,
        }
    }

    pub fn set_trusted(&mut self, trusted: bool) {
        self.trusted = trusted;
    }

    pub async fn run_worker(self) -> Result<()> {
        let mut interval = time::interval(Duration::from_secs(5));
        
        loop {
            let stream_res = UnixStream::connect(&self.socket_path).await;
            
            let stream = match stream_res {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to connect to master at {:?}: {}. Retrying in 5s...", self.socket_path, e);
                    interval.tick().await;
                    continue;
                }
            };

            let mut framed = Framed::new(stream, LinesCodec::new());

            // Register
            let reg = Message::Register(RegisterInfo {
                vibe_id: self.vibe_id.clone(),
                physical_id: self.physical_id.clone(),
                terminal_type: self.terminal_type.clone(),
                role: self.role.clone(),
                pid: std::process::id(),
            });

            self.send_msg(&mut framed, &reg).await?;

            // Wait for Ack
            match framed.next().await {
                Some(Ok(line)) => {
                    let msg = Message::from_str(&line)?;
                    if msg != Message::Ack {
                        eprintln!("Expected Ack, got {:?}. Retrying...", msg);
                        interval.tick().await;
                        continue;
                    }
                }
                _ => {
                    eprintln!("Disconnected before registration Ack. Retrying...");
                    interval.tick().await;
                    continue;
                }
            }

            println!("Worker {} registered and listening.", self.vibe_id);

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        let hb = Message::Heartbeat(HeartbeatInfo {
                            vibe_id: self.vibe_id.clone(),
                            status: "idle".to_string(),
                        });
                        if let Err(e) = self.send_msg(&mut framed, &hb).await {
                            eprintln!("Failed to send heartbeat: {}. Reconnecting...", e);
                            break;
                        }
                    }
                    msg_res = framed.next() => {
                        match msg_res {
                            Some(Ok(line)) => {
                                let msg = Message::from_str(&line)?;
                                match msg {
                                    Message::ExecuteIntent(intent) => {
                                        self.handle_intent(&mut framed, intent).await?;
                                    }
                                    Message::Ack => {}
                                    _ => {
                                        eprintln!("Warning: Unexpected message: {:?}", msg);
                                    }
                                }
                            }
                            Some(Err(e)) => {
                                eprintln!("Codec error: {}. Reconnecting...", e);
                                break;
                            }
                            None => {
                                eprintln!("Master disconnected. Reconnecting...");
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    async fn send_msg(&self, framed: &mut Framed<UnixStream, LinesCodec>, msg: &Message) -> Result<()> {
        let json = serde_json::to_string(msg).map_err(|e| VibeError::Internal(e.to_string()))?;
        framed.send(json).await.map_err(|e| VibeError::Internal(e.to_string()))
    }

    async fn handle_intent(&self, framed: &mut Framed<UnixStream, LinesCodec>, intent: ExecuteIntentInfo) -> Result<()> {
        println!("\n[VIBE] Intent received: {}", intent.cmd);
        
        let approved = if self.trusted || intent.trusted {
            true
        } else {
            self.show_gate(&intent.cmd)
        };

        if approved {
            println!("[VIBE] Executing command...");
            let mut cmd = std::process::Command::new("sh");
            #[cfg(windows)]
            let mut cmd = std::process::Command::new("cmd");
            
            #[cfg(not(windows))]
            cmd.arg("-c").arg(&intent.cmd);
            #[cfg(windows)]
            cmd.arg("/c").arg(&intent.cmd);

            if let Some(cwd) = intent.cwd {
                cmd.current_dir(cwd);
            }

            match cmd.status() {
                Ok(status) => {
                    println!("[VIBE] Command exited with status: {}", status);
                }
                Err(e) => {
                    eprintln!("[VIBE] Failed to execute command: {}", e);
                }
            }
        } else {
            println!("[VIBE] Command rejected by user.");
        }

        self.send_msg(framed, &Message::Ack).await?;
        Ok(())
    }

    fn show_gate(&self, cmd: &str) -> bool {
        println!("\n--------------------------------------------------");
        println!("  [VIBE GATE] Incoming Command Validation");
        println!("--------------------------------------------------");
        println!("  Command: {}", cmd);
        println!("--------------------------------------------------");
        
        Confirm::new()
            .with_prompt("Do you want to execute this command?")
            .default(false)
            .interact()
            .unwrap_or(false)
    }

    pub async fn run_heartbeat(self) -> Result<()> {
        self.run_worker().await
    }

    pub async fn send_exit_status(&self, code: i32) -> Result<()> {
        let stream = UnixStream::connect(&self.socket_path).await
            .map_err(|e| VibeError::Internal(format!("Failed to connect to master to send exit status: {}", e)))?;
        
        let mut framed = Framed::new(stream, LinesCodec::new());

        let msg = Message::ExitStatus(ExitStatusInfo {
            vibe_id: self.vibe_id.clone(),
            code,
        });

        self.send_msg(&mut framed, &msg).await?;

        if let Some(Ok(line)) = framed.next().await {
            let msg = Message::from_str(&line)?;
            if msg != Message::Ack {
                return Err(VibeError::Internal(format!("Expected Ack, got {:?}", msg)));
            }
        }
        Ok(())
    }

    pub async fn send_report(&self, status: String, summary: String) -> Result<()> {
        let stream = UnixStream::connect(&self.socket_path).await
            .map_err(|e| VibeError::Internal(format!("Failed to connect to master to send report: {}", e)))?;
        
        let mut framed = Framed::new(stream, LinesCodec::new());

        let msg = Message::Report(ReportInfo {
            vibe_id: self.vibe_id.clone(),
            status,
            summary,
        });

        self.send_msg(&mut framed, &msg).await?;

        if let Some(Ok(line)) = framed.next().await {
            let msg = Message::from_str(&line)?;
            if msg != Message::Ack {
                return Err(VibeError::Internal(format!("Expected Ack, got {:?}", msg)));
            }
        }
        Ok(())
    }
}
