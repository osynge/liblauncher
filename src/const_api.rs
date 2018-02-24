#[derive(Debug)]
pub enum LauncherError {
    ExecutableNotFound,
    ProcessNotFound,
    WaitpidError,
    ForkError,
    InvalidChildFd,
    InvalidRedirectPairentFd,
}

pub type LaunchResult = Result<(), LauncherError>;

#[derive(Debug, Copy, Clone)]
pub enum RedirectType {
    RedirectRead = 0,
    RedirectWrite,
    RedirectIgnore,
    RedirectMirror,
}
