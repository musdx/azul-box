use std::error::Error;

use rodio::{Decoder, OutputStream, Sink};

pub fn done_sound() {
    let source = include_bytes!("../../../assets/sounds/completion-success.oga");
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let source = Decoder::new(std::io::Cursor::new(source)).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}
pub fn fail_sound() {
    let source = include_bytes!("../../../assets/sounds/completion-fail.oga");
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
use notify_rust::Notification;
pub fn notification_done(info: &str) -> Result<(), Box<dyn Error>> {
    Notification::new()
        .summary("Azul Box")
        .body(format!("Your {} is done!", info).as_str())
        .icon("azul_box")
        .show()?;
    Ok(())
}
pub fn notification_fail(info: &str) -> Result<(), Box<dyn Error>> {
    Notification::new()
        .summary("Azul Box")
        .body(format!("Your {} FAIL!!", info).as_str())
        .icon("azul_box")
        .show()?;
    Ok(())
}
