use std::fs;
use std::path::PathBuf;
use chrono::Local;
use crate::error::Result;
use crate::env::resolve_plans_dir;

pub fn save_plan(vibe_id: &str, plan: &str) -> Result<PathBuf> {
    let plans_dir = resolve_plans_dir()?;
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let filename = format!("plan_{}_{}.md", vibe_id, timestamp);
    let path = plans_dir.join(filename);
    
    fs::write(&path, plan)?;
    Ok(path)
}
