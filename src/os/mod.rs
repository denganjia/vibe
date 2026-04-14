#[cfg(windows)]
pub mod windows;

#[cfg(windows)]
pub use windows::assign_to_job;

#[cfg(not(windows))]
pub fn assign_to_job(_child_handle: &std::process::Child) -> crate::error::Result<()> {
    // No-op for non-Windows platforms
    Ok(())
}
