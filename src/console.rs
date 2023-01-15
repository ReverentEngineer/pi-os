use core::fmt;

/// Console write trait
pub trait Write {

   fn write(&self, args: fmt::Arguments) -> fmt::Result;

   fn write_str(&self, s: &str) -> fmt::Result {
       self.write(format_args!("{s}"))
   }

   fn fmt<D: fmt::Display>(&self, display: D) -> fmt::Result {
        self.write(format_args!("{display}"))
   }

}


/// Console read trait
pub trait Read {

    fn read_char(&self) -> char;

}


/// Trait representing a user interface to the kernel
pub trait Console: Write + Read { }
