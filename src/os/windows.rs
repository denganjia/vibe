use crate::error::{Result, VibeError};
use std::os::windows::io::AsRawHandle;
use std::process::Child;
use std::sync::OnceLock;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::System::JobObjects::*;

static JOB_OBJECT: OnceLock<HANDLE> = OnceLock::new();

/// Assigns a child process to a global job object that will kill the process tree when the handle is closed.
pub fn assign_to_job(child: &Child) -> Result<()> {
    let job_handle = get_job_object()?;
    let child_handle = child.as_raw_handle() as HANDLE;

    unsafe {
        if AssignProcessToJobObject(job_handle, child_handle) == 0 {
            return Err(VibeError::Unexpected(format!(
                "Failed to assign process to job object: error code {}",
                GetLastError()
            )));
        }
    }
    Ok(())
}

fn get_job_object() -> Result<HANDLE> {
    JOB_OBJECT
        .get_or_try_init(|| unsafe {
            let handle = CreateJobObjectW(std::ptr::null(), std::ptr::null());
            if handle == 0 {
                return Err(VibeError::Unexpected(format!(
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
                return Err(VibeError::Unexpected(format!(
                    "Failed to set job object information: error code {}",
                    GetLastError()
                )));
            }

            Ok(handle)
        })
        .map(|h| *h)
}
