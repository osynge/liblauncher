use libc::c_int;
use libc::pipe;
use libc::dup;
use libc::dup2;
use libc::close;
use std::io::Error;
use libc::pid_t;
use libc::waitpid;
use libc::kill;
use libc::WNOHANG;

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

pub(crate) fn wait(launched_process_id: pid_t) -> Result<i32, i32> {
    let rc: pid_t;
    let mut status = 0 as c_int;
    let options = WNOHANG as c_int;
    if launched_process_id == -1 {
        return Err(-1);
    }
    unsafe {
        rc = waitpid(launched_process_id, &mut status as *mut c_int, options);
    }
    if rc == -1 {
        println!("waitpid failed!");
        return Err(-3);
    }
    if rc == launched_process_id {
        return Ok(status);
    } else {
        return Err(-4);
    }
}

pub(crate) fn kill_process(launched_process_id: pid_t, signal: u32) -> Result<i32, i32> {
    let rc: c_int;
    let signal_as = signal as c_int;
    if signal_as < 0 {
        return Err(-5);
    }

    unsafe {
        rc = kill(launched_process_id, 1);
    }
    if rc == -1 {
        println!("waitpid failed!");
        return Err(-3);
    }
    if rc == 0 {
        return Ok(0);
    } else {
        return Err(-4);
    }
}
