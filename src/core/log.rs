use log::Log;
use crate::core::console;

struct Logger;

static LOGGER: Logger = Logger;

impl Log for Logger {

    fn enabled(&self, _metadata: &::log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &::log::Record) {
        if self.enabled(record.metadata()) {
            let console = console::get();
            console.write(format_args!("{}", record.level())).unwrap();
            console.write_str(" | ").unwrap();
            console.write(format_args!("{}", record.args())).unwrap();
            console.write_str("\r\n").unwrap();
        }
    }

    fn flush(&self) {
        let console = console::get();
        console.flush();
    }
}

pub fn init() {
    unsafe { log::set_logger_racy(&LOGGER).unwrap() };
    log::set_max_level(log::LevelFilter::Debug);
}
