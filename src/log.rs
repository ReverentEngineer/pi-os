use log::Log;
use crate::{uart, console::Write};

struct Logger;

static LOGGER: Logger = Logger;

impl Log for Logger {

    fn enabled(&self, _metadata: &::log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &::log::Record) {
        if self.enabled(record.metadata()) {
            let uart = uart::get();
            uart.fmt(record.level()).unwrap();
            uart.write_str(" | ").unwrap();
            uart.fmt(record.args()).unwrap();
            uart.write_str("\r\n").unwrap();
        }
    }

    fn flush(&self) {
    }
}

pub fn init() {
    unsafe { log::set_logger_racy(&LOGGER).unwrap() };
    log::set_max_level(log::LevelFilter::Debug);
}
