use const_api;

use redirect_process;

#[derive(Debug)]
pub struct RedirectFactory {
    pub file_descriptor_child: Option<u32>,   // Child ID wanted
    pub file_descriptor_pairent: Option<u32>, // Pairent ID to mirror
    pub redirect: Option<const_api::RedirectType>,
}

impl RedirectFactory {
    pub fn new() -> Result<RedirectFactory, String> {
        Ok(RedirectFactory {
            file_descriptor_child: None,
            file_descriptor_pairent: None,
            redirect: None,
        })
    }

    pub fn generate_process(
        &mut self,
    ) -> Result<redirect_process::RedirectProcess, const_api::LauncherError> {
        let bill = redirect_process::RedirectProcess {
            file_descriptor_read: None,
            file_descriptor_write: None,
            file_descriptor_child: self.file_descriptor_child.clone(),
            file_descriptor_pairent: self.file_descriptor_pairent.clone(),
            redirect: self.redirect.clone(),
        };
        return Ok(bill);
    }
}
