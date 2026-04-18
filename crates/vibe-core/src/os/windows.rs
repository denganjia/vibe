use crate::error::{Result, VibeError};
use std::os::windows::io::AsRawHandle;
use std::os::windows::process::CommandExt;
use std::process::Child;
use std::sync::Mutex;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::System::JobObjects::*;
use windows_sys::Win32::System::Threading::{CREATE_NO_WINDOW, DETACHED_PROCESS};

struct SafeHandle(HANDLE);
unsafe impl Send for SafeHandle {}
unsafe impl Sync for SafeHandle {}

static JOB_OBJECT: Mutex<Option<SafeHandle>> = Mutex::new(None);

/// Spawns the current process as a detached daemon.
/// On Windows, this re-executes the current executable with a VIBE_DAEMON environment variable.
pub fn spawn_daemon() -> Result<()> {
    if std::env::var("VIBE_DAEMON").is_ok() {
        return Ok(());
    }

    let exe = std::env::current_exe()?;
    let args: Vec<String> = std::env::args().skip(1).collect();

    std::process::Command::new(exe)
        .args(args)
        .env("VIBE_DAEMON", "1")
        .creation_flags(DETACHED_PROCESS | CREATE_NO_WINDOW)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| VibeError::Internal(format!("Failed to spawn daemon: {}", e)))?;

    std::process::exit(0);
}

/// Assigns a child process to a global job object that will kill the process tree when the handle is closed.
pub fn assign_to_job(child: &Child) -> Result<()> {
    let job_handle = get_job_object()?;
    let child_handle = child.as_raw_handle() as HANDLE;

    unsafe {
        if AssignProcessToJobObject(job_handle, child_handle) == 0 {
            return Err(VibeError::Internal(format!(
                "Failed to assign process to job object: error code {}",
                GetLastError()
            )));
        }
    }
    Ok(())
}

fn get_job_object() -> Result<HANDLE> {
    let mut job_obj = JOB_OBJECT.lock().map_err(|e| VibeError::Internal(format!("Failed to lock job object mutex: {}", e)))?;
    if let Some(ref safe_handle) = *job_obj {
        return Ok(safe_handle.0);
    }

    unsafe {
        let handle = CreateJobObjectW(std::ptr::null(), std::ptr::null());
        if handle.is_null() {
            return Err(VibeError::Internal(format!(
                "Failed to create job object: error code {}",
                GetLastError()
            )));
        }

        let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = std::mem::zeroed();
        info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;

        if SetInformationJobObject(
            handle,
            JobObjectExtendedLimitInformation,
            &info as *const _ as *const _,
            std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
        ) == 0
        {
            CloseHandle(handle);
            return Err(VibeError::Internal(format!(
                "Failed to set job object information: error code {}",
                GetLastError()
            )));
        }

        *job_obj = Some(SafeHandle(handle));
        Ok(handle)
    }
}
