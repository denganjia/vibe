use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{self, BufRead, Write};
use vibe_core::env::{detect_current_terminal, resolve_socket_path};
use vibe_core::state::StateStore;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
    pub id: Value,
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    pub jsonrpc: String,
    pub result: Option<Value>,
    pub error: Option<Value>,
    pub id: Value,
}

pub async fn run_mcp_server() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut line = String::new();

    while handle.read_line(&mut line)? > 0 {
        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(req) => req,
            Err(_) => {
                line.clear();
                continue;
            }
        };

        let response = handle_request(request).await?;
        let json_res = serde_json::to_string(&response)?;
        println!("{}", json_res);
        io::stdout().flush()?;

        line.clear();
    }

    Ok(())
}

async fn handle_request(req: JsonRpcRequest) -> anyhow::Result<JsonRpcResponse> {
    match req.method.as_str() {
        "initialize" => {
            Ok(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: Some(serde_json::json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "serverInfo": {
                        "name": "vibe-cli",
                        "version": "0.1.0"
                    }
                })),
                error: None,
            })
        }
        "tools/list" => {
            Ok(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: Some(serde_json::json!({
                    "tools": [
                        {
                            "name": "vibe_check",
                            "description": "Check if the current terminal environment supports physical orchestration (split/focus).",
                            "inputSchema": {
                                "type": "object",
                                "properties": {}
                            }
                        },
                        {
                            "name": "vibe_list",
                            "description": "List all active vibe agents and their current status.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {}
                            }
                        },
                        {
                            "name": "vibe_run",
                            "description": "Run a command in a tracked vibe agent. If current environment is not supported, it will spawn an external window.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "command": { "type": "string", "description": "The command to execute" },
                                    "role": { "type": "string", "description": "Optional role for the agent" }
                                },
                                "required": ["command"]
                            }
                        },
                        {
                            "name": "vibe_split",
                            "description": "Split the current pane or create a new one externally if local orchestration is not available.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "vertical": { "type": "boolean", "description": "Split vertically instead of horizontally" }
                                }
                            }
                        },
                        {
                            "name": "vibe_focus",
                            "description": "Switch terminal focus to a specific vibe agent's pane.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "vibeId": { "type": "string", "description": "Target vibe ID" }
                                },
                                "required": ["vibeId"]
                            }
                        },
                        {
                            "name": "vibe_inject",
                            "description": "Inject a command into a running worker agent.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "vibeId": { "type": "string", "description": "Target vibe ID" },
                                    "command": { "type": "string", "description": "The command to inject" }
                                },
                                "required": ["vibeId", "command"]
                            }
                        }
                    ]
                })),
                error: None,
            })
        }
        "tools/call" => {
            let params = req.params.unwrap_or_default();
            let name = params.get("name").and_then(|v| v.as_str()).unwrap_or_default();
            let tool_params = params.get("arguments").cloned().unwrap_or(serde_json::json!({}));
            
            let result = call_tool(name, tool_params).await?;
            
            Ok(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: Some(serde_json::json!({
                    "content": [
                        {
                            "type": "text",
                            "text": serde_json::to_string_pretty(&result)?
                        }
                    ]
                })),
                error: None,
            })
        }
        _ => {
            Ok(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: None,
                error: Some(serde_json::json!({
                    "code": -32601,
                    "message": "Method not found"
                })),
            })
        }
    }
}

async fn call_tool(name: &str, params: Value) -> anyhow::Result<Value> {
    match name {
        "vibe_check" => {
            let term_opt = detect_current_terminal();
            let term_name = vibe_core::env::get_terminal_name();
            let socket_path = resolve_socket_path()?;
            let master_alive = tokio::net::UnixStream::connect(&socket_path).await.is_ok();
            let store = StateStore::new()?;
            let active_workers = store.list_active_panes()?.len();

            Ok(serde_json::json!({
                "supported": term_opt.is_some(),
                "terminal": term_name,
                "master_status": if master_alive { "running" } else { "stopped" },
                "active_workers": active_workers,
                "recommendation": if term_opt.is_some() {
                    "Environment supported. You can use split and focus."
                } else {
                    "Current terminal does not support local orchestration. Use vibe_run to spawn tasks in an external window."
                }
            }))
        }
        "vibe_list" => {
            let store = StateStore::new()?;
            let panes = store.list_active_panes()?;
            let json_panes: Vec<_> = panes.into_iter().map(|(v_id, p_id, t_type, role, status, summary, cwd)| {
                serde_json::json!({
                    "vibe_id": v_id,
                    "physical_id": p_id,
                    "terminal": t_type,
                    "role": role,
                    "status": status,
                    "summary": summary,
                    "cwd": cwd
                })
            }).collect();
            Ok(serde_json::json!(json_panes))
        }
        "vibe_run" => {
            let command = params.get("command").and_then(|v| v.as_str()).ok_or_else(|| anyhow::anyhow!("Missing command"))?;
            
            let exe = std::env::current_exe()?;
            let mut cmd = std::process::Command::new(exe);
            cmd.arg("run");
            cmd.arg("--yes");
            cmd.arg("--");
            cmd.arg(command);

            let child = cmd.spawn()?;
            
            Ok(serde_json::json!({
                "status": "started",
                "pid": child.id(),
                "note": "The task has been spawned. Check vibe_list for its registration status."
            }))
        }
        "vibe_split" => {
            let vertical = params.get("vertical").and_then(|v| v.as_bool()).unwrap_or(false);
            
            let exe = std::env::current_exe()?;
            let mut cmd = std::process::Command::new(exe);
            cmd.arg("split");
            if vertical {
                cmd.arg("--vertical");
            }

            let output = cmd.output()?;
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            
            if output.status.success() {
                Ok(serde_json::json!({ "status": "success", "output": stdout }))
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                anyhow::bail!("Split failed: {}", stderr);
            }
        }
        "vibe_focus" => {
            let vibe_id = params.get("vibeId").and_then(|v| v.as_str()).ok_or_else(|| anyhow::anyhow!("Missing vibeId"))?;
            
            let exe = std::env::current_exe()?;
            let mut cmd = std::process::Command::new(exe);
            cmd.arg("focus");
            cmd.arg(vibe_id);

            let output = cmd.output()?;
            if output.status.success() {
                Ok(serde_json::json!({ "status": "success", "message": format!("Focused on {}", vibe_id) }))
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                anyhow::bail!("Focus failed: {}", stderr);
            }
        }
        "vibe_inject" => {
            let vibe_id = params.get("vibeId").and_then(|v| v.as_str()).ok_or_else(|| anyhow::anyhow!("Missing vibeId"))?;
            let command = params.get("command").and_then(|v| v.as_str()).ok_or_else(|| anyhow::anyhow!("Missing command"))?;
            
            let exe = std::env::current_exe()?;
            let mut cmd = std::process::Command::new(exe);
            cmd.arg("inject");
            cmd.arg(vibe_id);
            cmd.arg(command);
            cmd.arg("--yes");

            let output = cmd.output()?;
            if output.status.success() {
                Ok(serde_json::json!({ "status": "success", "message": format!("Injected command into {}", vibe_id) }))
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                anyhow::bail!("Injection failed: {}", stderr);
            }
        }
        _ => anyhow::bail!("Tool not found"),
    }
}
