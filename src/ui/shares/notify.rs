use rodio::{Decoder, OutputStream, Sink};

pub fn done_sound() {
    let source = include_bytes!("../../../assets/sounds/completion-success.oga");
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let source = Decoder::new(std::io::Cursor::new(source)).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}
pub fn button_sound() {
    let source = include_bytes!("../../../assets/sounds/button-pressed-modifier.oga");
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let source = Decoder::new(std::io::Cursor::new(source)).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}
