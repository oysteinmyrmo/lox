use log::{error, Metadata, Record};

pub struct LoxLogger {}

impl LoxLogger {
    pub fn scanner_error(line: usize, message: String) {
        Self::scanner_report(line, "".into(), message)
    }

    fn scanner_report(line: usize, location: String, message: String) {
        error!("[line {}] Error{}: {}", line, location, message);
    }
}

impl log::Log for LoxLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {
        todo!()
    }
}
