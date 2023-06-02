
use eframe::egui;

mod sound_types;
mod ui;
mod voice_algorithms;




fn main() -> eframe::Result<()> {

    /* audio initialization */
    // define the audio stream and its handle
    let (
        _stream,
        stream_handle
    ) = rodio::OutputStream::try_default().unwrap();

    // get a sink for the output stream
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    sink.set_volume(0.15); // reduce volume

    let (sound_gen, tx) = sound_types::SoundGenerator::new(
        voice_algorithms::_electric_piano1,
    );

    sink.append(sound_gen);


    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default() // not sure why this works but it does
    };

    return eframe::run_native( // here graphics takes over the main thread
        "AudioKeyboard",
        options,
        Box::new(|_cc| Box::<ui::AppState>::new(ui::AppState::new(tx))),
    );



}