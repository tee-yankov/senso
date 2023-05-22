use std::{
    fmt::Display,
    fs::OpenOptions,
    io::Write,
    sync::{mpsc, Mutex},
    thread,
};

use lazy_static::lazy_static;

struct Logger<T> {
    rx: Mutex<mpsc::Receiver<T>>,
    #[allow(unused)]
    tx: Mutex<mpsc::Sender<T>>,
}

impl<T: Display> Logger<T> {
    fn start(&self) {
        let mut out = OpenOptions::new()
            .write(true)
            .append(true)
            .open("err.log")
            .unwrap();
        out.set_len(0).unwrap();

        while let Ok(msg) = self.rx.lock().unwrap().recv() {
            writeln!(&mut out, "{}", msg).unwrap();
        }
    }
}

impl<T> Default for Logger<T> {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            tx: Mutex::new(tx),
            rx: Mutex::new(rx),
        }
    }
}

lazy_static! {
    static ref LOGGER: Logger<String> = Logger::default();
}

#[allow(unused)]
pub fn log_message(message: &str) {
    LOGGER
        .tx
        .lock()
        .unwrap()
        .send(String::from(message))
        .unwrap();
}

pub fn start_logger() {
    thread::spawn(|| {
        LOGGER.start();
    });
}
