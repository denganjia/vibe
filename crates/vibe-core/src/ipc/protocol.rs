use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Message {
    Register(RegisterInfo),
    Heartbeat(HeartbeatInfo),
    ExitStatus(ExitStatusInfo),
    Ack,
    ExecuteIntent(ExecuteIntentInfo),
    GateRequest(GateRequestInfo),
    GateResponse(GateResponseInfo),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RegisterInfo {
    pub vibe_id: String,
    pub physical_id: String,
    pub terminal_type: String,
    pub role: Option<String>,
    pub pid: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct HeartbeatInfo {
    pub vibe_id: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ExitStatusInfo {
    pub vibe_id: String,
    pub code: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ExecuteIntentInfo {
    pub target_vibe_id: String,
    pub cmd: String,
    pub cwd: Option<String>,
    pub env: std::collections::HashMap<String, String>,
    pub trusted: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GateRequestInfo {
    pub cmd: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GateResponseInfo {
    pub approved: bool,
}

impl Message {
    pub fn to_ndjson(&self) -> serde_json::Result<String> {
        let mut json = serde_json::to_string(self)?;
        json.push('\n');
        Ok(json)
    }

    pub fn from_str(s: &str) -> serde_json::Result<Self> {
        serde_json::from_str(s.trim())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialization() {
        let msg = Message::Register(RegisterInfo {
            vibe_id: "vibe-1".to_string(),
            physical_id: "phys-1".to_string(),
            terminal_type: "wezterm".to_string(),
            role: Some("worker".to_string()),
            pid: 1234,
        });

        let json = msg.to_ndjson().unwrap();
        assert!(json.ends_with('\n'));
        
        let deserialized = Message::from_str(&json).unwrap();
        assert_eq!(msg, deserialized);
    }

    #[test]
    fn test_heartbeat_serialization() {
        let msg = Message::Heartbeat(HeartbeatInfo {
            vibe_id: "vibe-1".to_string(),
            status: "running".to_string(),
        });

        let json = msg.to_ndjson().unwrap();
        let deserialized = Message::from_str(&json).unwrap();
        assert_eq!(msg, deserialized);
    }
}
