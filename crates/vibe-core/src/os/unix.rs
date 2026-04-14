use daemonize::Daemonize;
use std::fs::File;
use crate::error::Result;

pub fn spawn_daemon() -> Result<()> {
    let daemonize = Daemonize::new()
        .stdout(File::create("/dev/null").unwrap())
        .stderr(File::create("/dev/null").unwrap());

    match daemonize.start() {
        Ok(_) => Ok(()),
        Err(e) => Err(crate::error::VibeError::Internal(e.to_string())),
    }
}
