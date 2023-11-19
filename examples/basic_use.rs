use std::path::Path;

fn main() {
    // Load the flite library
    let st = std::time::Instant::now();
    let mut flite = flite::Flite::new().expect("Failed to initialize flite");
    println!("Initialized flite in {:?}", st.elapsed());

    // Load the English language
    let st = std::time::Instant::now();
    flite
        .load_english()
        .expect("should have loaded english successfully");
    println!("Loaded english in {:?}", st.elapsed());

    // Load the voice
    let st = std::time::Instant::now();
    let voice = flite
        .load_model_from_file(Path::new(
            "/home/niko/data/flite-voices/cmu_us_rms.flitevox",
        ))
        .expect("Failed to load voice");
    println!("Loaded voice in {:?}", st.elapsed());

    // Synthesize audio
    let st = std::time::Instant::now();
    let sample = flite
        .text_to_wave("Hello, world!", &voice)
        .expect("Failed to synthesize audio");
    let et = st.elapsed();
    let sample_seconds = sample.num_samples() as f64 / sample.sample_rate() as f64;
    println!(
        "Synthesized {} seconds of audio in {:?} (rtf = {})",
        sample_seconds,
        et,
        et.as_secs_f64() / sample_seconds
    );
}
