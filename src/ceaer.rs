use std;
use libc::c_char;
use libc::execvpe;
use libc::fork;
use libc::pid_t;
use std::result::Result;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;

use std::ffi::CString;

use redirect_map;
use redirect_map_factory;
use const_api;
use process;

#[derive(Debug)]
pub struct Ceaer {
    pub executable: String,
    pub argv: Vec<String>,
    pub envp: Vec<String>,
    pub return_code: i32,
    red: redirect_map_factory::RedirectMapFactory,
}

impl Ceaer {
    pub fn new() -> Result<Ceaer, String> {
        Ok(Ceaer {
            executable: String::new(),
            argv: Vec::new(),
            envp: Vec::new(),
            return_code: -1,
            red: redirect_map_factory::RedirectMapFactory::new().unwrap(),
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
    fn _wrapped_execvpe(&mut self) -> Result<process::Process, ()> {
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
        unsafe {
            execvpe(child_path, child_argv, child_envp);
        }
        panic!("execvpe failed.");
    }

    pub fn launch(&mut self) -> Result<process::Process, const_api::LauncherError> {
        let mut bill = redirect_map::RedirectMapContainer::new();
        match self.red.update_map_container(&mut bill) {
            Ok(_) => {}
            Err(_) => {
                return Err(const_api::LauncherError::LaunchPrepError);
            }
        };
        match bill.prep_launch() {
            Ok(_) => {}
            Err(_) => {
                return Err(const_api::LauncherError::LaunchPrepError);
            }
        }
        let child_id: pid_t;
        unsafe {
            let launched_process_id = fork();

            if launched_process_id < 0 {
                return Err(const_api::LauncherError::ForkError);
            }
            child_id = launched_process_id;
        };
        if child_id == 0 {
            // is child process
            let _ = bill.post_launch_child();
            let _ = self._wrapped_execvpe();
        } else {
            let _ = bill.post_launch_pairent();
        }
        let output = process::Process {
            executable: self.executable.clone(),
            argv: self.argv.clone(),
            envp: self.envp.clone(),
            red: bill,
            return_code: 0,
            launched_process_id: child_id,
        };
        Ok(output)
    }
}
