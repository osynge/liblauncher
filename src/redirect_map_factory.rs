use const_api;
use std::collections::HashMap;
use redirect_factory;

use redirect_map;

#[derive(Debug)]
pub struct RedirectMapFactory {
    pub(crate) redirect: HashMap<u32, redirect_factory::RedirectFactory>,
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

    pub(crate) fn update_map_container(
        &mut self,
        boo: &mut redirect_map::RedirectMapContainer,
    ) -> Result<(), String> {
        for (mut sd, mut fd) in self.redirect.iter_mut() {
            let fd_mapping: u32 = *sd;
            match fd.generate_process() {
                Ok(factory) => {
                    boo.redirect.insert(fd_mapping, factory);
                }
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
        let mut bill = redirect_map::RedirectMapContainer::new();
        match rc.update_map_container(&mut bill) {
            Ok(v) => {}
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
        let mut bill = redirect_map::RedirectMapContainer::new();
        match rc.update_map_container(&mut bill) {
            Ok(v) => {}
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
