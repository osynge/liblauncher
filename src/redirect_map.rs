use const_api;
use std::collections::HashMap;

use redirect_factory;

#[derive(Debug)]
pub struct RedirectMapFactory {
    pub(crate) redirect: HashMap<u32, redirect_factory::RedirectFactory>,
}

#[derive(Debug)]
pub struct RedirectMapContainer {
    pub(crate) redirect: HashMap<u32, redirect_factory::RedirectProcess>,
}

impl RedirectMapFactory {
    pub fn new() -> Result<RedirectMapFactory, String> {
        Ok(RedirectMapFactory {
            redirect: HashMap::new(),
        })
    }
    pub(crate) fn redirect_set(
        &mut self,
        child_fd: u32,
        pairent_fd: Option<u32>,
        redirect_type: Option<const_api::RedirectType>,
    ) -> const_api::LaunchResult {
        let mut bill = redirect_factory::RedirectFactory::new().unwrap();
        bill.redirect = redirect_type;
        bill.file_descriptor_child = Some(child_fd);
        bill.file_descriptor_pairent = pairent_fd;
        self.redirect.insert(child_fd, bill);
        Ok(())
    }

    pub(crate) fn generate_container(&mut self) -> Result<RedirectMapContainer, String> {
        let mut output = RedirectMapContainer {
            redirect: HashMap::new(),
        };
        for (mut sd, mut fd) in self.redirect.iter_mut() {
            let fd_mapping: u32 = *sd;
            match fd.generate_process() {
                Ok(factory) => {
                    output.redirect.insert(fd_mapping, factory);
                }
                Err(_) => {
                    return Err(String::new());
                }
            };
        }
        Ok(output)
    }
}

impl RedirectMapContainer {
    pub fn new() -> Result<RedirectMapContainer, String> {
        Ok(RedirectMapContainer {
            redirect: HashMap::new(),
        })
    }

    pub(crate) fn redirect_fd(&mut self, child_fd: u32) -> Option<u32> {
        let foo: &mut redirect_factory::RedirectProcess;
        match self.redirect.get_mut(&child_fd) {
            Some(expr) => {
                foo = expr;
            }
            None => {
                return None;
            }
        }
        let bill = foo.redirect_fd();
        return bill;
    }
    pub(crate) fn post_launch_child(&mut self) -> const_api::LaunchResult {
        for (mut sd, mut fd) in self.redirect.iter_mut() {
            let bill = fd.post_launch_child();
        }
        Ok(())
    }
    pub(crate) fn post_launch_pairent(&mut self) -> const_api::LaunchResult {
        for (mut sd, mut fd) in self.redirect.iter_mut() {
            let bill = fd.post_launch_pairent();
        }
        Ok(())
    }

    pub(crate) fn prep_launch(&mut self) -> Result<(), String> {
        for (mut sd, mut fd) in self.redirect.iter_mut() {
            let fd_mapping: u32 = *sd;
            match fd.prep_launch() {
                Ok(factory) => {}
                Err(_) => {
                    return Err(String::new());
                }
            };
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc::c_int;

    #[test]
    fn has_stdio_precreated() {
        let rc = RedirectMapFactory::new().unwrap();
        let redirects = rc.redirect.len();
        if redirects != 0 {
            assert!(false);
        }
    }

    #[test]
    fn redirect_map_factory_can_create_redirects() {
        let mut rc = RedirectMapFactory::new().unwrap();
        match rc.redirect.get(&0) {
            Some(v) => {
                assert!(false);
            }
            None => {}
        }
        let rd1 = rc.redirect_set(0, None, Some(const_api::RedirectType::RedirectRead));
        match rd1 {
            Ok(_) => {}
            Err(_) => {
                assert!(false);
            }
        }
        match rc.redirect.get(&0) {
            Some(v) => {}
            None => {
                assert!(false);
            }
        }
    }
    #[test]
    fn redirect_map_factory_can_generate_container() {
        let mut rc = RedirectMapFactory::new().unwrap();
        match rc.redirect.get(&0) {
            Some(v) => {
                assert!(false);
            }
            None => {}
        }
        let rd1 = rc.redirect_set(0, None, Some(const_api::RedirectType::RedirectRead));
        match rd1 {
            Ok(_) => {}
            Err(_) => {
                assert!(false);
            }
        }
        match rc.redirect.get(&0) {
            Some(v) => {}
            None => {
                assert!(false);
            }
        }
        let mut bill: RedirectMapContainer;
        match rc.generate_container() {
            Ok(v) => {
                bill = v;
            }
            Err(_) => {
                assert!(false);
                return;
            }
        };
        match bill.redirect.get(&0) {
            Some(v) => {}
            None => {
                assert!(false);
            }
        }
    }
    #[test]
    fn RedirectMapContainer_can_prep_launch() {
        let mut rc = RedirectMapFactory::new().unwrap();
        match rc.redirect.get(&0) {
            Some(v) => {
                assert!(false);
            }
            None => {}
        }
        let rd1 = rc.redirect_set(0, None, Some(const_api::RedirectType::RedirectRead));
        match rd1 {
            Ok(_) => {}
            Err(_) => {
                assert!(false);
            }
        }
        match rc.redirect.get(&0) {
            Some(v) => {}
            None => {
                assert!(false);
            }
        }
        let mut bill: RedirectMapContainer;
        match rc.generate_container() {
            Ok(v) => {
                bill = v;
            }
            Err(_) => {
                assert!(false);
                return;
            }
        };
        match bill.redirect.get(&0) {
            Some(v) => {}
            None => {
                assert!(false);
            }
        };
        match bill.prep_launch() {
            Ok(v) => {}
            Err(_) => {
                assert!(false);
            }
        };
        let mut redirect_type = None;

        match bill.redirect.get(&0) {
            Some(v) => {
                redirect_type = v.redirect;
            }
            None => {
                assert!(false);
            }
        };
        let mut p: const_api::RedirectType;
        match redirect_type {
            Some(v) => {
                p = v;
            }
            None => {
                assert!(false);
                return;
            }
        };
        if p != const_api::RedirectType::RedirectRead {
            assert!(false);
        }
    }
}
