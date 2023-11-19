use std::ffi::{CString, OsStr};
use std::path::Path;

pub enum FliteOutputKind<'a> {
    Play,
    File(&'a Path),
    Discard,
}

impl FliteOutputKind<'_> {
    pub(crate) fn to_c_string(&self) -> CString {
        match self {
            FliteOutputKind::Play => CString::new("play").unwrap(),
            FliteOutputKind::File(path) => {
                CString::new(OsStr::as_encoded_bytes(path.as_ref())).unwrap()
            }
            FliteOutputKind::Discard => CString::new("none").unwrap(),
        }
    }
}
