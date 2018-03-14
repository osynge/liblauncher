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
