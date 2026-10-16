mod error;
use crate::rcm::error::RcmError;

const NVIDIA_TEGRA_VID: u16 = 0x0955; // Vendor ID for NVIDIA Corporation
const NVIDIA_TEGRA_PID: u16 = 0x7321; // Product ID for NVIDIA Tegra RCM mode

#[derive(Debug)]
pub struct TegraRCMDevice {
    device: std::path::PathBuf,
}

impl TegraRCMDevice {
    fn find() -> Option<std::path::PathBuf> {
        Some(std::path::PathBuf::from("/sys/bus/pci/drivers/xhci_hcd/"))
    }
    pub fn read(&self) {}
    pub fn write(&self) {}
    pub fn new() -> Result<Self, RcmError> {
        if let Some(device) = Self::find() {
            Ok(Self { device })
        } else {
            Err(RcmError::DeviceNotFound)
        }
    }
}
