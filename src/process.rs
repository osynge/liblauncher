use wrap_posix;
use libc::pid_t;

use std::os::unix::io::FromRawFd;
use std::fs::File;

use redirect_map;

#[derive(Debug)]
pub struct Process {
    pub(crate) launched_process_id: pid_t,
    pub(crate) executable: String,
    pub(crate) argv: Vec<String>,
    pub(crate) envp: Vec<String>,
    pub(crate) return_code: i32,
    pub(crate) red: redirect_map::RedirectMapContainer,
}

impl Process {
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
                println!("redirect_file failed!{:?}", child_fd);
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

    pub fn wait(&mut self) -> Result<(), i32> {
        if self.launched_process_id == -1 {
            return Err(-1);
        }
        match wrap_posix::wait(self.launched_process_id) {
            Ok(j) => {
                self.return_code = j;
                return Ok(());
            }
            Err(j) => {
                if j == -4 {
                    return Ok(());
                } else {
                    return Err(j);
                }
            }
        };
    }

    pub fn signal(&mut self, signal: u32) -> Result<(), i32> {
        if self.launched_process_id == -1 {
            return Err(-1);
        };
        let rc = wrap_posix::kill_process(self.launched_process_id, signal);
        match rc {
            Ok(_) => {
                return Ok(());
            }
            Err(j) => {
                return Err(j);
            }
        };
    }
}
