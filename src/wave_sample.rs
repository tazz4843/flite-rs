pub struct FliteWaveSample {
    ptr: *mut flite_sys::cst_wave,
}

impl Drop for FliteWaveSample {
    fn drop(&mut self) {
        unsafe { flite_sys::delete_wave(self.ptr) };
    }
}

impl FliteWaveSample {
    /// # Safety
    /// * `ptr` must be a valid pointer to a flite wave sample
    pub unsafe fn new(ptr: *mut flite_sys::cst_wave) -> Self {
        Self { ptr }
    }

    pub unsafe fn into_inner(self) -> *mut flite_sys::cst_wave {
        let ptr = self.ptr;
        std::mem::forget(self);
        ptr
    }

    pub fn kind(&self) -> &str {
        let cst_wave = unsafe { *self.ptr };
        let kind = unsafe { std::ffi::CStr::from_ptr(cst_wave.type_) };
        kind.to_str().expect("Kind is not valid UTF-8")
    }

    pub fn sample_rate(&self) -> i32 {
        unsafe { *self.ptr }.sample_rate
    }

    pub fn num_samples(&self) -> i32 {
        unsafe { *self.ptr }.num_samples
    }

    pub fn num_channels(&self) -> i32 {
        unsafe { *self.ptr }.num_channels
    }

    pub fn samples(&self) -> &[i16] {
        unsafe { std::slice::from_raw_parts((*self.ptr).samples, self.num_samples() as usize) }
    }
}
