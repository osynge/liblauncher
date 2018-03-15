use wrap_posix;
use libc::pid_t;

use std::os::unix::io::FromRawFd;
use std::fs::File;

use redirect_map;

#[derive(Debug)]
pub struct Process {
    pub launched_process_id: pid_t,
    pub executable: String,
    pub argv: Vec<String>,
    pub envp: Vec<String>,
    pub return_code: i32,
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
}
