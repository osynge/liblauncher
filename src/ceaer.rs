use std;
use wrap_posix;
use libc::pid_t;
use std::result::Result;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use std::ffi::OsStr;

use redirect_map;
use redirect_map_factory;
use const_api;
use process;

#[derive(Debug)]
pub struct Ceaer {
    executable: String,
    argv: Vec<String>,
    envp: Vec<String>,
    return_code: i32,
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

    pub fn executable_set(&mut self, path: &std::path::Path) -> const_api::LaunchResult {
        self.executable = String::from(path.to_str().unwrap());
        Ok(())
    }

    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Ceaer {
        let bill = arg.as_ref();
        match bill.to_str() {
            Some(j) => {
                self.argv.push(String::from(j));
            }
            None => {}
        }
        self
    }

    pub fn arg_clear(&mut self) -> &mut Ceaer {
        self.argv.clear();
        self
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
        match wrap_posix::fork_process() {
            Ok(j) => child_id = j,
            Err(_) => {
                return Err(const_api::LauncherError::ForkError);
            }
        }
        if child_id == 0 {
            // is child process
            let _ = bill.post_launch_child();
        } else {
            let _ = bill.post_launch_pairent();
        }
        let mut output = process::Process {
            executable: self.executable.clone(),
            argv: self.argv.clone(),
            envp: self.envp.clone(),
            red: bill,
            return_code: 0,
            launched_process_id: child_id,
        };
        if child_id == 0 {
            // is child process

            let _ = output._wrapped_execvpe();
        } else {

        }
        Ok(output)
    }
}
