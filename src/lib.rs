mod error;
mod model;
mod output_kind;
mod voice;
mod wave_sample;

pub use error::{FliteError, FliteResult};
pub use model::Flite;
pub use output_kind::FliteOutputKind;
pub use voice::FliteVoice;
pub use wave_sample::FliteWaveSample;
