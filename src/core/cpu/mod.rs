use core::fmt;
mod arch;
pub use arch::*;

#[derive(Copy, Clone, Debug)]
pub struct Info {
    arch: Arch,
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    manufacturer_id: arch::arm::ManufacturerID,
}

impl Info {

    pub fn get() -> Info {
        Self {
            arch: Arch::get(),
            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
            manufacturer_id: arch::midr()
        }
    }

    pub fn arch(&self) -> Arch {
        self.arch
    }

    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    pub fn manufacturer_id(&self) -> arch::arm::ManufacturerID {
        self.manufacturer_id
    }
}


#[derive(Copy, Clone, Debug)]
pub enum Arch {
    Arm,
    AArch64
}

impl Arch {

    const fn get() -> Arch {
       #[cfg(target_arch = "arm")]
       return Arch::Arm;
       #[cfg(target_arch = "aarch64")]
       return Arch::AArch64;
    }

}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Arm => write!(f, "arm"),
            Self::AArch64 => write!(f, "aarch64")
        }
    }
}

