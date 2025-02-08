use strum::{Display, EnumString};

/// CPU architecture the binary is compiled as
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display)]
pub enum PlatformArch {
    #[strum(serialize = "x86")]
    X86,
    #[strum(serialize = "x64")]
    X64,
}

impl Default for PlatformArch {
    fn default() -> Self {
        platform_arch()
    }
}

#[cfg(all(
    target_pointer_width = "64",
    not(any(target_arch = "arm", target_arch = "aarch64"))
))]
pub fn platform_arch() -> PlatformArch {
    PlatformArch::X64
}

#[cfg(all(
    target_pointer_width = "32",
    not(any(target_arch = "arm", target_arch = "aarch64"))
))]
pub fn platform_arch() -> PlatformArch {
    PlatformArch::X86
}
