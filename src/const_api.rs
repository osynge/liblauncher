
#[derive(Debug)]
pub enum LauncherError {
    ExecutableNotFound,
    ProcessNotFound,
    WaitpidError,
    ForkError
}

pub type LaunchResult = Result<(), LauncherError>;

#[derive(Debug)]
pub enum Direction {
    RedirectRead = 0,
    RedirectWrite,
    RedirectIgnore
}
