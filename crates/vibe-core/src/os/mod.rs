#[cfg(windows)]
pub mod windows;

#[cfg(not(windows))]
pub mod unix;

pub mod shell;

#[cfg(windows)]
pub use windows::assign_to_job;

pub use shell::ShellAdapter;

#[cfg(not(windows))]
pub fn assign_to_job(_child_handle: &std::process::Child) -> crate::error::Result<()> {
    // No-op for non-Windows platforms
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
