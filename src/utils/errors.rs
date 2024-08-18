#[derive(Debug)]
pub enum MyError {
    FileOrFolderNotFound,
    AccessDeniedBySystem,
    NotAValidPath,
    UNKNOWN,
}
pub type MyResult<T> = core::result::Result<T, MyError>;

