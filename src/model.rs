use crate::error::{FliteError, FliteResult};
use crate::output_kind::FliteOutputKind;
use crate::voice::FliteVoice;
use crate::wave_sample::FliteWaveSample;
use std::ffi::{CStr, CString};
use std::path::Path;
use std::sync::Once;

static FLITE_INITIALIZED: Once = Once::new();
const ENGLISH_CSTR: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"eng\0") };

#[derive(Copy, Clone)]
pub struct Flite {
    _hidden: (),
}

impl Flite {
    pub fn new() -> FliteResult<Self> {
        if FLITE_INITIALIZED.is_completed() {
            Err(FliteError::AlreadyInitialized)
        } else {
            let ret = unsafe { flite_sys::flite_init() };
            if ret != 0 {
                Err(FliteError::InitFailed(ret))
            } else {
                Ok(Flite { _hidden: () })
            }
        }
    }

    /// Load the English language.
    ///
    /// Specifically, finds `usenglish_init` and `cmu_lex_init`,
    /// then calls `flite_add_lang` with those callbacks.
    ///
    /// # Returns
    /// `Ok(())` on success, `Err` on failure.
    pub fn load_english(&mut self) -> FliteResult<()> {
        let retval = unsafe {
            flite_sys::flite_add_lang(
                ENGLISH_CSTR.as_ptr(),
                Some(flite_sys::usenglish_init),
                Some(flite_sys::cmu_lex_init),
            )
        };
        if retval != 1 {
            Err(FliteError::InitFailed(retval))
        } else {
            Ok(())
        }
    }

    /// Load a voice from the given path.
    ///
    /// # Arguments
    /// * `path` - The path to the voice file
    ///
    /// # Returns
    /// The loaded voice on success
    pub fn load_model_from_file(&self, path: &Path) -> FliteResult<FliteVoice> {
        let path = CString::new(format!("file://{}", path.to_str().unwrap())).unwrap();
        let ret = unsafe { flite_sys::flite_voice_select(path.as_ptr()) };
        if ret.is_null() {
            Err(FliteError::NullPtr)
        } else {
            Ok(unsafe { FliteVoice::new(ret) })
        }
    }

    /// Load a voice from the given url.
    ///
    /// # Arguments
    /// * `url` - The url to the voice file
    ///
    /// # Returns
    /// The loaded voice on success
    #[cfg(feature = "url-support")]
    pub fn load_model_from_url(&self, url: url::Url) -> FliteResult<FliteVoice> {
        if !(url.scheme() == "http" || url.scheme() == "https") {
            return Err(FliteError::InvalidUrlScheme);
        }
        let url = CString::new(url.as_str()).unwrap();
        let ptr = unsafe { flite_sys::flite_voice_select(url.as_ptr()) };
        if ptr.is_null() {
            Err(FliteError::NullPtr)
        } else {
            Ok(unsafe { FliteVoice::new(ptr) })
        }
    }

    /// Synthesize and play the given file using the given voice.
    ///
    /// # Arguments
    /// * `filename` - The path to the file to synthesize
    /// * `voice` - The voice to use for synthesis
    /// * `ssml` - Should the file be treated as SSML?
    ///
    /// # Returns
    /// The length of the synthesized audio in seconds.
    pub fn file_to_speech(&self, filename: &Path, voice: &FliteVoice, ssml: bool) -> f32 {
        const OUTPUT_KIND: &str = "play";
        let output_kind = CString::new(OUTPUT_KIND).unwrap();
        let filename = CString::new(filename.to_str().unwrap()).unwrap();
        if ssml {
            unsafe {
                flite_sys::flite_ssml_file_to_speech(
                    filename.as_ptr(),
                    voice.ptr,
                    output_kind.as_ptr(),
                )
            }
        } else {
            unsafe {
                flite_sys::flite_file_to_speech(filename.as_ptr(), voice.ptr, output_kind.as_ptr())
            }
        }
    }

    /// Synthesize and output the given text using the given voice.
    ///
    /// # Arguments
    /// * `text` - The text to synthesize
    /// * `voice` - The voice to use for synthesis
    /// * `output_kind` - The kind of output to use
    /// * `ssml` - Should the text be treated as SSML?
    ///
    /// # Returns
    /// The length of the synthesized audio in seconds.
    pub fn text_to_speech(
        &self,
        text: &str,
        voice: &FliteVoice,
        output_kind: FliteOutputKind,
        ssml: bool,
    ) -> f32 {
        let text = CString::new(text).unwrap();
        let output_kind = output_kind.to_c_string();
        if ssml {
            unsafe {
                flite_sys::flite_ssml_text_to_speech(text.as_ptr(), voice.ptr, output_kind.as_ptr())
            }
        } else {
            unsafe {
                flite_sys::flite_text_to_speech(text.as_ptr(), voice.ptr, output_kind.as_ptr())
            }
        }
    }

    /// Synthesize and return an audio sample of the given text using the given voice.
    ///
    /// # Arguments
    /// * `text` - The text to synthesize
    /// * `voice` - The voice to use for synthesis
    ///
    /// # Returns
    /// The synthesized audio sample on success
    pub fn text_to_wave(&self, text: &str, voice: &FliteVoice) -> FliteResult<FliteWaveSample> {
        let text = CString::new(text).unwrap();
        let ptr = unsafe { flite_sys::flite_text_to_wave(text.as_ptr(), voice.ptr) };
        if ptr.is_null() {
            Err(FliteError::NullPtr)
        } else {
            Ok(unsafe { FliteWaveSample::new(ptr) })
        }
    }
}
