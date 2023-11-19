pub struct FliteVoice {
    pub(crate) ptr: *mut flite_sys::cst_voice,
}

impl FliteVoice {
    /// Create a new voice from the given pointer.
    ///
    /// # Safety
    /// * `ptr` must be a valid pointer to a flite voice
    pub unsafe fn new(ptr: *mut flite_sys::cst_voice) -> Self {
        Self { ptr }
    }

    /// Get the underlying pointer to the flite voice.
    ///
    /// # Safety
    /// * The caller must ensure that the pointer is not used after the voice is dropped
    /// * The caller must also synchronize mutation of the voice with any other threads
    pub unsafe fn into_inner(self) -> *mut flite_sys::cst_voice {
        let ptr = self.ptr;
        std::mem::forget(self);
        ptr
    }

    pub fn name(&self) -> &str {
        let cst_voice = unsafe { *self.ptr };
        let name = unsafe { std::ffi::CStr::from_ptr(cst_voice.name) };
        name.to_str().expect("Name is not valid UTF-8")
    }

    pub fn get_callback(
        &self,
    ) -> Option<
        unsafe extern "C" fn(
            _: *mut flite_sys::cst_utterance_struct,
            _: *mut flite_sys::cst_voice_struct,
        ) -> *mut flite_sys::cst_utterance_struct,
    > {
        unsafe { *self.ptr }.utt_init
    }

    pub unsafe fn set_callback(
        &mut self,
        callback: Option<
            unsafe extern "C" fn(
                _: *mut flite_sys::cst_utterance_struct,
                _: *mut flite_sys::cst_voice_struct,
            ) -> *mut flite_sys::cst_utterance_struct,
        >,
    ) {
        unsafe { *self.ptr }.utt_init = callback;
    }

    // none of the remaining getters are implemented due to them appearing to have no real use
}

impl Drop for FliteVoice {
    fn drop(&mut self) {
        unsafe { flite_sys::delete_voice(self.ptr) };
    }
}
