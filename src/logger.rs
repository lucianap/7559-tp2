use std::string::String;
use std::time::SystemTime;
use std::fs::File;
use crate::mapa;
extern crate chrono;

use chrono::Local;
use std::io::{Write, BufWriter};
use std::sync::Mutex;

pub struct Logger {
    pub debug: bool,
    pub file: Mutex<File>
}

impl Logger {

    pub fn new(debug: bool) -> Logger {
        let file = Mutex::new(File::create("log.txt").unwrap());
        Logger {
            debug,
            file
        }
    }

    pub fn debug(&self, msg: &str) {
        if self.debug{
            let mut mtx_file = &*self.file.lock().expect("No se pudo lockear el file");
            let mut writer = BufWriter::new(mtx_file);
            write!(&mut writer, "[DEBUG] {} {}\n", Local::now().format("%Y-%m-%d %H:%M:%S") ,msg).unwrap();
        }

    }

    pub fn info(&self, msg: &str) {
        let mut mtx_file = &*self.file.lock().expect("No se pudo lockear el file");
        let mut writer = BufWriter::new(mtx_file);
        write!(&mut writer, "[INFO] {} {}\n", Local::now().format("%Y-%m-%d %H:%M:%S") ,msg).unwrap();
    }

}













