use libc::c_int;
use libc::pipe;

use const_api;

#[derive(Debug)]
pub struct LauncherStructPipe {
    pub file_descriptor_read: Option<u32>, // FIFO read
    pub file_descriptor_write: Option<u32>, // FIFO write
    pub file_descriptor_child: Option<u32>, // Child ID wanted
    pub file_descriptor_pairent: Option<u32>, // Pairent ID to mirror
    pub redirect: Option<const_api::RedirectType>,
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
}


#[derive(Debug)]
pub struct RedirectContainer {
    pub(crate) redirect: Vec<LauncherStructPipe>,
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


impl RedirectContainer {
    pub fn new() -> RedirectContainer {
        let mut new_list = Vec::new();
        for counter in 0..3 {
            let mut bill = LauncherStructPipe::new().unwrap();
            bill.redirect = Some(const_api::RedirectType::RedirectMirror);
            bill.file_descriptor_pairent = Some(counter);
            new_list.push(bill);
        }
        RedirectContainer { redirect: new_list }
    }
    fn set_size(&mut self, newsize: usize) -> const_api::LaunchResult {
        let mut len = self.redirect.len();
        while len < newsize {
            let mut bill = LauncherStructPipe::new().unwrap();
            bill.redirect = Some(const_api::RedirectType::RedirectMirror);
            self.redirect.push(bill);
            len = self.redirect.len();
        }
        Ok(())

    }


    pub(crate) fn prep_launch(&mut self) -> const_api::LaunchResult {
        for mut fd in self.redirect.iter_mut() {

            let ref mut bill = fd.redirect;
            let jam = bill.as_mut();
            match jam {
                Some(v) => {
                    //*v = 42,
                    match *v {
                        const_api::RedirectType::RedirectRead => {
                            let pipe_rc = mkpipe();
                            let (file_descriptor_read, file_descriptor_write) = pipe_rc.unwrap();
                            fd.file_descriptor_read = Some(file_descriptor_read);
                            fd.file_descriptor_write = Some(file_descriptor_write);
                        }
                        const_api::RedirectType::RedirectWrite => {
                            let pipe_rc = mkpipe();
                            let (file_descriptor_read, file_descriptor_write) = pipe_rc.unwrap();
                            fd.file_descriptor_read = Some(file_descriptor_read);
                            fd.file_descriptor_write = Some(file_descriptor_write);
                        }
                        const_api::RedirectType::RedirectMirror => {}
                        const_api::RedirectType::RedirectIgnore => {}
                    }
                }
                None => {}
            }
        }
        Ok(())
    }


    pub(crate) fn redirect_set(
        &mut self,
        child_fd: u32,
        pairent_fd: Option<u32>,
        redirect_type: Option<const_api::RedirectType>,
    ) -> const_api::LaunchResult {
        let min_size = child_fd as usize;
        let set_size_rc = self.set_size(min_size);
        if set_size_rc.is_err() {
            return set_size_rc;
        }
        let mut bill = LauncherStructPipe::new().unwrap();
        bill.redirect = redirect_type;
        bill.file_descriptor_child = Some(child_fd);
        bill.file_descriptor_pairent = pairent_fd;
        if let Some(elem) = self.redirect.get_mut(1) {
            *elem = bill;
        }
        Ok(())
    }
}
