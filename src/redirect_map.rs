use const_api;
use std::collections::HashMap;

use redirect_factory;
use redirect_process;

#[derive(Debug)]
pub struct RedirectMapContainer {
    pub(crate) redirect: HashMap<u32, redirect_process::RedirectProcess>,
}

impl RedirectMapContainer {
    pub fn new() -> RedirectMapContainer {
        RedirectMapContainer {
            redirect: HashMap::new(),
        }
    }

    pub(crate) fn redirect_fd(&mut self, child_fd: u32) -> Option<u32> {
        let foo: &mut redirect_process::RedirectProcess;
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
}
