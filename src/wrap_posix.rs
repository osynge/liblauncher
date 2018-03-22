use std;
use libc::c_int;
use libc::pipe;
use libc::dup;
use libc::dup2;
use libc::close;
use std::io::Error;
use libc::pid_t;
use libc::waitpid;
use libc::kill;
use libc::fork;
use libc::WNOHANG;
use libc::c_char;
use libc::execvpe;
use const_api;

use std::ffi::CString;
use ceaer::Ceaer;

use process;
use redirect_map;

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

pub(crate) fn fork_process() -> Result<pid_t, ()> {
    let child_id: pid_t;
    unsafe {
        let launched_process_id = fork();

        if launched_process_id < 0 {
            return Err(());
        } else {
            child_id = launched_process_id;
        }
    }
    Ok(child_id)
}

pub(crate) fn wrapped_execvpe(
    executable: &String,
    argv: &Vec<String>,
    envp: &Vec<String>,
) -> Result<process::Process, ()> {
    let child_path: *const c_char;
    let child_argv: *const *const c_char;
    let child_envp: *const *const c_char;
    let exec_str = executable.clone();
    let ex1 = CString::new(exec_str).unwrap();
    child_path = ex1.as_ptr();
    let cstr_argv: Vec<_> = argv.iter()
        .map(|arg| CString::new(arg.as_str()).unwrap())
        .collect();
    let mut p_argv: Vec<_> = cstr_argv.iter().map(|arg| arg.as_ptr()).collect();
    p_argv.push(std::ptr::null());
    child_argv = p_argv.as_ptr();
    let cstr_envp: Vec<_> = envp.iter()
        .map(|env| CString::new(env.as_str()).unwrap())
        .collect();
    let mut p_envp: Vec<_> = cstr_envp.iter().map(|env| env.as_ptr()).collect();
    p_envp.push(std::ptr::null());
    child_envp = p_envp.as_ptr();
    unsafe {
        execvpe(child_path, child_argv, child_envp);
    }
    panic!("execvpe failed.");
}
