use core::ops::Deref;
use core::fmt;
use crate::sync::RawSpinlock;
use lock_api::Mutex;

/// Console write trait
pub trait Write {

   fn write(&self, args: fmt::Arguments) -> fmt::Result;

   fn write_str(&self, s: &str) -> fmt::Result {
       self.write(format_args!("{s}"))
   }

   fn flush(&self);

}


/// Console read trait
pub trait Read {

    fn read_char(&self) -> char;

}


/// Trait representing a user interface to the kernel
pub trait Console: Write + Read { }


impl<C> Console for C where C: Write + Read { }

struct NullConsole;

impl Write for NullConsole {
   
    fn write(&self, args: fmt::Arguments) -> fmt::Result {
        Ok(())
    }

    fn flush(&self) {

    }

}

impl Read for NullConsole {

    fn read_char(&self) -> char {
        ' '
    }

}

/// An empty console place holder
static NULL_CONSOLE: NullConsole = NullConsole;

/// The current configured console
static CONSOLE: Mutex<RawSpinlock, &'static (dyn Console + Sync)> = Mutex::new(&NULL_CONSOLE); 

/// Set the current console
pub fn set(console: &'static (dyn Console + Sync)) {
    *CONSOLE.lock() = console;
}

pub fn get() -> &'static dyn Console {
    *CONSOLE.lock().deref()
}
