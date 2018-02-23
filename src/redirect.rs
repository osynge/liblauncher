use libc::c_int;
use libc::pipe;
use libc::dup;
use libc::dup2;
use libc::close;
use std::io::Error;

use const_api;

#[derive(Debug)]
pub struct LauncherStructPipe {
    pub file_descriptor_read: Option<u32>, // FIFO read
    pub file_descriptor_write: Option<u32>, // FIFO write
    pub file_descriptor_child: Option<u32>, // Child ID wanted
    pub file_descriptor_pairent: Option<u32>, // Pairent ID to mirror
    pub redirect: Option<const_api::RedirectType>,
}


#[derive(Debug)]
pub enum LauncherStructPipeError {
    Unknown,
    RedirectRead,
    RedirectWrite,
    RedirectIgnore,
    RedirectMirror,
}


fn posix_close(file_num: c_int) -> Result<(), const_api::LauncherError> {

    unsafe {
        let rc: c_int;
        println!("closing={:?}", file_num);
        if file_num == 1 {
            assert!(false);
        }
        rc = close(file_num);
        if rc == 0 {
            return Ok(());
        }
        let freed = Error::last_os_error();
        println!("raw_os_error={:?}={:?}", file_num, freed);
        return Ok(());
    }
}

fn mkpipe() -> Result<(u32, u32), const_api::LauncherError> {
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


fn posix_dup(oldfd: c_int) -> Result<c_int, const_api::LauncherError> {
    let dup2_rc: c_int;
    unsafe {
        dup2_rc = dup(oldfd);
        if dup2_rc == -1 {
            return Err(const_api::LauncherError::ExecutableNotFound);
        }
    }
    Ok(dup2_rc)
}

fn posix_dup2(oldfd: c_int, newfd: c_int) -> Result<(), const_api::LauncherError> {
    let dup2_rc: c_int;
    unsafe {
        dup2_rc = dup2(oldfd, newfd);
        if dup2_rc == -1 {
            return Err(const_api::LauncherError::ExecutableNotFound);
        }
    }
    Ok(())
}

impl LauncherStructPipe {
    pub fn new() -> Result<LauncherStructPipe, String> {
        Ok(LauncherStructPipe {
            file_descriptor_read: None,
            file_descriptor_write: None,
            file_descriptor_child: None,
            file_descriptor_pairent: None,
            redirect: None,
        })
    }

    fn prep_launch_fifo(&mut self) -> Result<(), LauncherStructPipeError> {
        let pipe_rc = mkpipe();
        let (file_descriptor_read, file_descriptor_write) = pipe_rc.unwrap();
        self.file_descriptor_read = Some(file_descriptor_read);
        self.file_descriptor_write = Some(file_descriptor_write);
        self.file_descriptor_pairent = Some(file_descriptor_read);
        return Ok(());
    }

    fn prep_launch_mirror(&mut self) -> Result<(), LauncherStructPipeError> {
        match self.file_descriptor_write {
            Some(fd_int) => {
                posix_close(fd_int as i32);
                self.file_descriptor_write = None;
            }
            None => {}
        }
        let filedes_c: c_int;
        match self.file_descriptor_child {
            Some(fd_int) => filedes_c = fd_int as c_int,
            None => {
                return Err(LauncherStructPipeError::Unknown);
            }

        }
        let filedes_w: c_int;
        match self.file_descriptor_pairent {
            Some(fd_int) => filedes_w = fd_int as c_int,
            None => {
                filedes_w = filedes_c;
            }

        }
        if filedes_w == filedes_c {
            ()
        }
        let posix_dup_rc = posix_dup(filedes_w);
        match posix_dup_rc {
            Ok(n) => {
                self.file_descriptor_write = Some(n as u32);
            }
            Err(_) => {}
        }
        Ok(())
    }


    pub fn prep_launch(&mut self) -> Result<(), LauncherStructPipeError> {
        let action: const_api::RedirectType;
        {
            let ref mut bill = self.redirect;
            let jam = bill.as_mut();
            match jam {
                Some(redirect_type) => {
                    action = redirect_type.clone();
                }
                None => {
                    return Err(LauncherStructPipeError::Unknown);
                }
            }
        }

        match action {
            const_api::RedirectType::RedirectRead => self.prep_launch_fifo(),
            const_api::RedirectType::RedirectWrite => self.prep_launch_fifo(),
            const_api::RedirectType::RedirectMirror => self.prep_launch_mirror(),
            const_api::RedirectType::RedirectIgnore => {
                return Ok(());
            }
        }
    }


    fn post_launch_child_close(&mut self) -> Result<(), LauncherStructPipeError> {
        let filedes_w = self.file_descriptor_pairent.unwrap() as c_int;
        match posix_close(filedes_w) {
            Ok(()) => {}
            Err(_) => {
                return Err(LauncherStructPipeError::Unknown);
            }
        }
        Ok(())
    }

    fn post_launch_child_redirect_read(&mut self) -> Result<(), LauncherStructPipeError> {
        let filedes_w = self.file_descriptor_write.unwrap() as c_int;
        let filedes_r = self.file_descriptor_child.unwrap() as c_int;
        let foo = posix_dup2(filedes_w, filedes_r);
        match foo {
            Ok(()) =>
            {
            }
            Err(_) => {
                return Err(LauncherStructPipeError::Unknown);
            }
        }
        match posix_close(filedes_w) {
            Ok(()) => {
                self.file_descriptor_write = None;
            }
            Err(_) => {
                return Err(LauncherStructPipeError::Unknown);
            }
        }
        self.post_launch_child_close()
    }

    fn post_launch_child_redirect_write(&mut self) -> Result<(), LauncherStructPipeError> {
        let filedes_w = self.file_descriptor_read.unwrap() as c_int;
        let filedes_r = self.file_descriptor_child.unwrap() as c_int;
        let foo = posix_dup2(filedes_w, filedes_r);
        match foo {
            Ok(()) => {}
            Err(_) => {
                return Err(LauncherStructPipeError::Unknown);
            }
        }
        match posix_close(filedes_r) {
            Ok(()) => {
                self.file_descriptor_read = None;
            }
            Err(_) => {
                return Err(LauncherStructPipeError::Unknown);
            }
        }
        self.post_launch_child_close()

    }
    fn post_launch_child_mirror(&mut self) -> Result<(), LauncherStructPipeError> {
        let filedes_c: c_int;
        match self.file_descriptor_child {
            Some(fd_int) => filedes_c = fd_int as c_int,
            None => {
                return Err(LauncherStructPipeError::Unknown);
            }
        }
        let filedes_w: c_int;
        match self.file_descriptor_pairent {
            Some(fd_int) => filedes_w = fd_int as c_int,
            None => {
                filedes_w = filedes_c;
            }
        }
        if filedes_w == filedes_c {
            ()
        }
        let filedes_j = self.file_descriptor_write.unwrap() as c_int;

        let posix_dup_rc = posix_dup2(filedes_j, filedes_c);
        match posix_dup_rc {
            Ok(()) => {
            }
            Err(_) => {
                return Err(LauncherStructPipeError::Unknown);
            }
        }
        let close_ok = posix_close(filedes_j);
        match close_ok {
            Ok(()) => {
                self.file_descriptor_write = None;
            }
            Err(_) => {
                return Err(LauncherStructPipeError::Unknown);
            }
        }
        Ok(())
    }
    fn post_launch_child_ignore(&mut self) -> Result<(), LauncherStructPipeError> {
        let filedes_r = self.file_descriptor_child.unwrap() as c_int;
        let close_ok = posix_close(filedes_r);
        match close_ok {
            Ok(()) =>
            {
            }
            Err(_) => {
                return Err(LauncherStructPipeError::Unknown);
            }
        }
        Ok(())
    }


    pub(crate) fn post_launch_child(&mut self) -> Result<(), LauncherStructPipeError> {
        let action: const_api::RedirectType;
        {
            let ref mut bill = self.redirect;
            let jam = bill.as_mut();
            match jam {
                Some(redirect_type) => {
                    action = redirect_type.clone();
                }
                None => {
                    return Err(LauncherStructPipeError::Unknown);
                }
            }
        }

        match action {
            const_api::RedirectType::RedirectRead => self.post_launch_child_redirect_read(),
            const_api::RedirectType::RedirectWrite => self.post_launch_child_redirect_write(),
            const_api::RedirectType::RedirectMirror => self.post_launch_child_mirror(),
            const_api::RedirectType::RedirectIgnore => self.post_launch_child_ignore(),
        }
    }



    fn post_launch_pairent_close(&mut self) -> Result<(), LauncherStructPipeError> {
        let filedes_w = self.file_descriptor_child.unwrap() as c_int;
        match posix_close(filedes_w) {
            Ok(()) => {
                self.file_descriptor_child = None;
            }
            Err(_) => {
                return Err(LauncherStructPipeError::Unknown);
            }
        }
        Ok(())
    }


    fn post_launch_pairent_redirect_read(&mut self) -> Result<(), LauncherStructPipeError> {
        let filedes_w = self.file_descriptor_write.unwrap() as c_int;
        match posix_close(filedes_w) {
            Ok(()) => {
                self.file_descriptor_write = None;
            }
            Err(_) => {
                return Err(LauncherStructPipeError::Unknown);
            }
        }
        Ok(())
    }

    fn post_launch_pairent_redirect_write(&mut self) -> Result<(), LauncherStructPipeError> {
        let filedes_w = self.file_descriptor_read.unwrap() as c_int;
        match posix_close(filedes_w) {
            Ok(()) => {
                self.file_descriptor_read = None;
            }
            Err(_) => {
                return Err(LauncherStructPipeError::Unknown);
            }
        }
        Ok(())
    }

    fn post_launch_pairent_redirect_ignore(&mut self) -> Result<(), LauncherStructPipeError> {
        Ok(())
    }

    fn post_launch_pairent_redirect_mirror(&mut self) -> Result<(), LauncherStructPipeError> {
        match self.file_descriptor_write {
            Some(n) => {
                match posix_close(n as i32) {
                    Ok(()) => {}
                    Err(_) => {
                        return Err(LauncherStructPipeError::Unknown);
                    }
                }
                self.file_descriptor_write = None;
            }
            None => {}
        }
        Ok(())
    }

    pub(crate) fn post_launch_pairent(&mut self) -> Result<(), LauncherStructPipeError> {
        let action: const_api::RedirectType;
        {
            let ref mut bill = self.redirect;
            let jam = bill.as_mut();
            match jam {
                Some(redirect_type) => {
                    action = redirect_type.clone();
                }
                None => {
                    return Err(LauncherStructPipeError::Unknown);
                }
            }
        }


        match action {
            const_api::RedirectType::RedirectRead => {
                let dup2_rc = self.post_launch_pairent_redirect_read();
                match dup2_rc {
                    Ok(()) => {}
                    Err(_) => {
                        return Err(LauncherStructPipeError::Unknown);
                    }
                }
            }
            const_api::RedirectType::RedirectWrite => {
                let dup2_rc = self.post_launch_pairent_redirect_write();
                match dup2_rc {
                    Ok(()) => {}
                    Err(_) => {
                        return Err(LauncherStructPipeError::Unknown);
                    }
                }
            }
            const_api::RedirectType::RedirectMirror => {
                return self.post_launch_pairent_redirect_mirror();
            }
            const_api::RedirectType::RedirectIgnore => {
                //dup2_rc = Ok(());
                return self.post_launch_pairent_redirect_ignore();
            }
        }
        Ok(())
    }
}


#[derive(Debug)]
pub struct RedirectContainer {
    pub(crate) redirect: Vec<LauncherStructPipe>,
}










impl RedirectContainer {
    pub fn new() -> RedirectContainer {
        let mut new_list = Vec::new();
        for counter in 0..3 {
            let mut bill = LauncherStructPipe::new().unwrap();
            bill.redirect = Some(const_api::RedirectType::RedirectMirror);
            bill.file_descriptor_child = Some(counter);
            bill.file_descriptor_pairent = Some(counter);
            new_list.push(bill);
        }
        RedirectContainer { redirect: new_list }
    }
    fn set_size(&mut self, newsize: usize) -> const_api::LaunchResult {
        let mut len = self.redirect.len();
        while len < newsize {
            let mut bill = LauncherStructPipe::new().unwrap();
            let child_index = (len as u32) - 1;
            bill.redirect = Some(const_api::RedirectType::RedirectMirror);
            bill.file_descriptor_child = Some(child_index);
            bill.file_descriptor_pairent = Some(child_index);
            self.redirect.push(bill);
            len = self.redirect.len();
        }
        Ok(())
    }


    pub(crate) fn prep_launch(&mut self) -> const_api::LaunchResult {
        for mut fd in self.redirect.iter_mut() {
            let bill = fd.prep_launch();


        }
        Ok(())
    }
    pub(crate) fn post_launch_child(&mut self) -> const_api::LaunchResult {
        for mut fd in self.redirect.iter_mut() {
            let bill = fd.post_launch_child();
        }
        Ok(())
    }
    pub(crate) fn post_launch_pairent(&mut self) -> const_api::LaunchResult {
        for mut fd in self.redirect.iter_mut() {
            let bill = fd.post_launch_pairent();
        }
        Ok(())
    }
    pub(crate) fn redirect_set(
        &mut self,
        child_fd: u32,
        pairent_fd: Option<u32>,
        redirect_type: Option<const_api::RedirectType>,
    ) -> const_api::LaunchResult {
        let current_size = self.redirect.len();
        let child_index = child_fd as usize;
        let min_size = (child_index) + 1;
        if current_size < min_size {
            let set_size_rc = self.set_size(min_size);
            if set_size_rc.is_err() {
                return set_size_rc;
            }
        }
        let mut bill = LauncherStructPipe::new().unwrap();
        bill.redirect = redirect_type;
        bill.file_descriptor_child = Some(child_fd);
        bill.file_descriptor_pairent = pairent_fd;
        if let Some(elem) = self.redirect.get_mut(child_index) {
            *elem = bill;
        }
        Ok(())
    }
    pub(crate) fn redirect_fd(&mut self, child_fd: u32) -> Option<u32> {
        let usize_child_fd = child_fd as usize;
        if usize_child_fd >= self.redirect.len() {
            return None;
        }
        match self.redirect.get(usize_child_fd) {
            Some(expr) => {
                return expr.file_descriptor_pairent;
            }
            None => {
                return None;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_stdio_precreated() {
        let rc = RedirectContainer::new();
        let redirects = rc.redirect.len();
        if redirects != 3 {
            assert!(false);
        }
    }

    #[test]
    fn can_create_redirects() {
        let mut rc = RedirectContainer::new();
        let rd1 = rc.redirect_set(0, None, Some(const_api::RedirectType::RedirectRead));
        match rd1 {
            Ok(_) => {}
            Err(_) => {
                assert!(false);
            }
        }
        let bing = rc.redirect[0].file_descriptor_pairent;
        match bing {
            Some(v) => {
                assert!(false);
            }
            None => {}
        }
        rc.prep_launch();
        let bing = rc.redirect[0].file_descriptor_pairent;
        let pairent_file_id: c_int;
        match bing {
            Some(v) => {
                pairent_file_id = v as c_int;
            }
            None => {
                assert!(false);
                return;
            }
        }
        let redirect: c_int;
        let jon = rc.redirect_fd(0);
        let redirect_file_id: c_int;
        match jon {
            Some(v) => {
                redirect_file_id = v as c_int;
            }
            None => {
                assert!(false);
                return;
            }
        }
        if redirect_file_id == pairent_file_id {

        } else {
            assert!(false);
            return;
        }
    }
}
