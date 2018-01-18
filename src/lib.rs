extern crate libc;

use libc::pid_t;
use libc::c_int;
use libc::c_char;
use libc::fork;
use libc::execvpe;
use libc::waitpid;
use libc::WNOHANG;
use std::result::Result;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use std::ffi::CString;

#[derive(Debug)]
pub enum LauncherError {
    ExecutableNotFound,
    ProcessNotFound,
    WaitpidError,
    ForkError
}

type LaunchResult = Result<(), LauncherError>;

#[derive(Debug)]
pub enum Direction {
    RedirectRead = 0,
    RedirectWrite,
    RedirectIgnore
}

#[derive(Debug)]
pub struct LauncherStructPipe {
	pub file_descriptor_child: i32,
	file_descriptor_pairent: i32,
	file_descriptor_id_child: i32,
	pipe_type: Direction
}



#[derive(Debug)]
pub struct Launcher {
    pub launched_process_id: pid_t,
    pub executable: String,
    pub argv: Vec<String>,
    pub envp: Vec<String>,
    pub redirects: Vec<LauncherStructPipe>,
    pub return_code: i32,
}


impl Launcher {
    pub fn new() -> Result<Launcher, String> {
        Ok(Launcher {
            launched_process_id: -1,
            executable: String::new(),
            argv: Vec::new(),
            envp: Vec::new(),
            redirects: Vec::new(),
            return_code: -1
        })
    }

    pub fn executable_set(&mut self, path: &str) -> LaunchResult {
        self.executable =  String::from(path);
        Ok(())
    }

    pub fn executable_get(&self) -> Result<String, ()> {
        return Ok(String::clone(&self.executable));
    }

    pub fn launch(&self) -> LaunchResult {
        let path = Path::new(&self.executable);
        if false == path.exists() {
            return Err(LauncherError::ExecutableNotFound);
        } 
        let md = path.metadata().unwrap();
        let perms = md.permissions();
        println!("permissions: {}", perms.mode());
        let child_id;
        unsafe {

            let launched_process_id = fork();
            if launched_process_id < 0 {
                return Err(LauncherError::ForkError);
            }
            child_id = launched_process_id;
        }
        if child_id == 0 {
            // is child process
            let child_path : *const c_char;
            let child_argv : *const *const c_char;
            let child_envp : *const *const c_char;

            let exec_str = self.executable.clone();
            let ex1 = CString::new(exec_str).unwrap();
            

            child_path = ex1.as_ptr();


            let cstr_argv: Vec<_> = self.argv.iter()
                .map(|arg| CString::new(arg.as_str()).unwrap())
                .collect();
            let mut p_argv: Vec<_> = cstr_argv.iter()
                .map(|arg| arg.as_ptr())
                .collect();
            p_argv.push(std::ptr::null());
            child_argv = p_argv.as_ptr();

            let cstr_envp: Vec<_> = self.envp.iter()
                .map(|env| CString::new(env.as_str()).unwrap())
                .collect();
            let mut p_envp: Vec<_> = cstr_envp.iter()
                .map(|env| env.as_ptr())
                .collect();
            p_envp.push(std::ptr::null());
            child_envp = p_envp.as_ptr();



            unsafe {
                execvpe(child_path, child_argv, child_envp);    
            }
            
            // pub unsafe extern "C" fn execvpe(
            // file: *const c_char, 
            // argv: *const *const c_char, 
            // envp: *const *const c_char
            // ) -> c_int
        }
        else {
            
        }
        
        
        Ok(())
    }

    pub fn wait(&mut self) -> Result<(), i32> {
	    let rc : pid_t;
        let mut status = 0 as c_int;
        let options = WNOHANG as c_int;
        if self.launched_process_id == -1
        {
            return Err(-1);
        }
        unsafe {
            rc = waitpid(self.launched_process_id, &mut status as *mut c_int,options);
        }
        if rc == -1 {
            println!("waitpid failed!");
            return Err(-1);
        }
        if rc == self.launched_process_id {
            self.return_code = status;
            return Ok(());
        }
        return Ok(());
    }  
}


