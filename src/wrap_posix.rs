use libc::c_int;
use libc::pipe;
use libc::dup;
use libc::dup2;
use libc::close;
use std::io::Error;

use const_api;

pub(crate) fn posix_close(file_num: c_int) -> Result<(), const_api::LauncherError> {
    unsafe {
        let rc: c_int;
        rc = close(file_num);
        if rc == 0 {
            return Ok(());
        }
        let freed = Error::last_os_error();
        println!("raw_os_error={:?}={:?}", file_num, freed);
        return Ok(());
    }
}

pub(crate) fn mkpipe() -> Result<(u32, u32), const_api::LauncherError> {
    let mut pipeparam: [c_int; 2] = [-1, -1];
    let pipe_rc: c_int;
    unsafe { pipe_rc = pipe(pipeparam.as_mut_ptr()) }
    if pipe_rc != 0 {
        return Err(const_api::LauncherError::ExecutableNotFound);
    }
    let file_descriptor_read = pipeparam[0] as u32;
    let file_descriptor_write = pipeparam[1] as u32;
    Ok((file_descriptor_read, file_descriptor_write))
}

pub(crate) fn posix_dup(oldfd: c_int) -> Result<c_int, const_api::LauncherError> {
    let dup2_rc: c_int;
    unsafe {
        dup2_rc = dup(oldfd);
        if dup2_rc == -1 {
            return Err(const_api::LauncherError::ExecutableNotFound);
        }
    }
    Ok(dup2_rc)
}

pub(crate) fn posix_dup2(oldfd: c_int, newfd: c_int) -> Result<(), const_api::LauncherError> {
    let dup2_rc: c_int;
    unsafe {
        dup2_rc = dup2(oldfd, newfd);
        if dup2_rc == -1 {
            return Err(const_api::LauncherError::ExecutableNotFound);
        }
    }
    Ok(())
}
