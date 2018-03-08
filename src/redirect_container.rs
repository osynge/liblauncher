use const_api;
use redirect::LauncherStructPipe;
use libc::c_int;
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
        let foo: &mut LauncherStructPipe;
        match self.redirect.get_mut(usize_child_fd) {
            Some(expr) => {
                foo = expr;
            }
            None => {
                return None;
            }
        }
        return foo.redirect_fd();
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
        let bing = rc.redirect[0].file_descriptor_read;
        match bing {
            Some(v) => {
                assert!(false);
            }
            None => {}
        }
        rc.prep_launch();
        let bing = rc.redirect[0].file_descriptor_read;
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
        rc.post_launch_pairent();
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
