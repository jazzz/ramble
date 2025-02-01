use std::ffi::OsString;

pub struct FileObject {
    pub filename: OsString,
    pub contents: String,
}
