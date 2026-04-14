#[cfg(windows)]
pub mod windows;

#[cfg(not(windows))]
pub mod unix;

#[cfg(windows)]
pub use windows::{assign_to_job, spawn_daemon};

#[cfg(not(windows))]
pub use unix::spawn_daemon;

#[cfg(not(windows))]
pub fn assign_to_job(_child_handle: &std::process::Child) -> crate::error::Result<()> {
    // No-op for non-Windows platforms
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_daemon_spawn() {
        // This test is hard to automate because it exits the process.
        // It's meant to be run manually or via a special test runner.
        spawn_daemon().expect("Failed to spawn daemon");
    }
}
