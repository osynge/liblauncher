#[derive(Debug)]
pub enum LauncherError {
    ExecutableNotFound,
    ProcessNotFound,
    WaitpidError,
    ForkError,
    InvalidChildFd,
    InvalidRedirectPairentFd,
    LaunchPrepError,
    Unknown,
}

pub type LaunchResult = Result<(), LauncherError>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RedirectType {
    RedirectRead,
    RedirectWrite,
    RedirectIgnore,
    RedirectMirror,
}
