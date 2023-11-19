pub type FliteResult<T> = std::result::Result<T, FliteError>;

#[derive(Debug, Clone, Copy)]
pub enum FliteError {
    InitFailed(i32),
    AlreadyInitialized,
    NullPtr,
    #[cfg(feature = "url-support")]
    InvalidUrlScheme,
    LanguageNotAdded,
}
