use std;
use libc::c_char;
use libc::c_int;
use libc::execvpe;
use libc::fork;
use libc::pid_t;
use libc::waitpid;
use libc::WNOHANG;
use std::result::Result;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;

use std::os::unix::io::FromRawFd;
use std::fs::File;
use std::ffi::CString;

use redirect_container;
use const_api;

#[derive(Debug)]
pub struct Launcher {
    pub launched_process_id: pid_t,
    pub executable: String,
    pub argv: Vec<String>,
    pub envp: Vec<String>,
    pub return_code: i32,
    red: redirect_container::RedirectContainer,
}

impl Launcher {
    pub fn new() -> Result<Launcher, String> {
        Ok(Launcher {
            launched_process_id: -1,
            executable: String::new(),
            argv: Vec::new(),
            envp: Vec::new(),
            return_code: -1,
            red: redirect_container::RedirectContainer::new(),
        })
    }

    pub fn executable_set(&mut self, path: &str) -> const_api::LaunchResult {
        self.executable = String::from(path);
        Ok(())
    }

    pub fn redirect_set(
        &mut self,
        child_fd: u32,
        pairent_fd: Option<u32>,
        redirect_type: Option<const_api::RedirectType>,
    ) -> const_api::LaunchResult {
        let redirect_set_rc = self.red.redirect_set(child_fd, pairent_fd, redirect_type);
        if redirect_set_rc.is_err() {
            return redirect_set_rc;
        }
        Ok(())
    }

    pub fn redirect_fd(&mut self, child_fd: u32) -> Option<u32> {
        self.red.redirect_fd(child_fd)
    }

    pub fn redirect_file(&mut self, child_fd: u32) -> Option<File> {
        let redirect_fd: u32;
        match self.red.redirect_fd(child_fd) {
            Some(redirect_fd_rc) => {
                redirect_fd = redirect_fd_rc;
            }
            None => {
                return None;
            }
        }
        let bill: File;
        unsafe {
            let make_file = File::from_raw_fd(redirect_fd as i32);
            bill = make_file;
        }
        return Some(bill);
    }

    pub fn executable_get(&self) -> Result<String, ()> {
        return Ok(String::clone(&self.executable));
    }

    fn launch_perms(&mut self) -> const_api::LaunchResult {
        let path = Path::new(&self.executable);
        if false == path.exists() {
            return Err(const_api::LauncherError::ExecutableNotFound);
        }
        let md = path.metadata().unwrap();
        let perms = md.permissions();
        Ok(())
    }

    fn launch_child(&mut self) -> const_api::LaunchResult {
        let child_path: *const c_char;
        let child_argv: *const *const c_char;
        let child_envp: *const *const c_char;
        let exec_str = self.executable.clone();
        let ex1 = CString::new(exec_str).unwrap();
        child_path = ex1.as_ptr();
        let cstr_argv: Vec<_> = self.argv
            .iter()
            .map(|arg| CString::new(arg.as_str()).unwrap())
            .collect();
        let mut p_argv: Vec<_> = cstr_argv.iter().map(|arg| arg.as_ptr()).collect();
        p_argv.push(std::ptr::null());
        child_argv = p_argv.as_ptr();
        let cstr_envp: Vec<_> = self.envp
            .iter()
            .map(|env| CString::new(env.as_str()).unwrap())
            .collect();
        let mut p_envp: Vec<_> = cstr_envp.iter().map(|env| env.as_ptr()).collect();
        p_envp.push(std::ptr::null());
        child_envp = p_envp.as_ptr();
        self.red.post_launch_child();
        unsafe {
            execvpe(child_path, child_argv, child_envp);
        }
        panic!("execvpe failed.");
    }

    pub fn launch(&mut self) -> const_api::LaunchResult {
        let pre_launch_rc = self.red.prep_launch();
        if pre_launch_rc.is_err() {
            return pre_launch_rc;
        }
        let child_id;
        unsafe {
            let launched_process_id = fork();
            if launched_process_id < 0 {
                return Err(const_api::LauncherError::ForkError);
            }
            child_id = launched_process_id;
        }
        if child_id == 0 {
            // is child process
            return self.launch_child();
        } else {
            self.launched_process_id = child_id;
            self.red.post_launch_pairent();
        }

        Ok(())
    }

    pub fn wait(&mut self) -> Result<(), i32> {
        let rc: pid_t;
        let mut status = 0 as c_int;
        let options = WNOHANG as c_int;
        if self.launched_process_id == -1 {
            return Err(-1);
        }
        unsafe {
            rc = waitpid(self.launched_process_id, &mut status as *mut c_int, options);
        }
        if rc == -1 {
            println!("waitpid failed!");
            return Err(-3);
        }
        if rc == self.launched_process_id {
            self.return_code = status;
            return Ok(());
        }
        return Ok(());
    }
}
