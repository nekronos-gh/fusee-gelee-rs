mod error;

use crate::rcm::error::RcmError;
use regex::Regex;
use std::{fs, path::PathBuf};
use walkdir::WalkDir;

const NVIDIA_TEGRA_VID: u16 = 0x0955; // Vendor ID for NVIDIA Corporation
const NVIDIA_TEGRA_PID: u16 = 0x7321; // Product ID for NVIDIA Tegra RCM mode

#[derive(Debug)]
pub struct TegraRCMDevice {
    sysfs_path: PathBuf,
}

fn read_hex_from_file(path: &PathBuf) -> Option<u16> {
    let content = fs::read_to_string(path).ok()?;
    u16::from_str_radix(content.trim(), 16).ok()
}

impl TegraRCMDevice {
    fn find_device() -> Result<PathBuf, RcmError> {
        // Iterate over all devices using the xhci_hcd interface
        let devices_re = Regex::new(r"/.*(\d{4}:\d{2}:\w{2}.\d)/usb\d/\d+-\d+$").unwrap();
        for entry in WalkDir::new("/sys/bus/pci/drivers/xhci_hcd/")
            .max_depth(3)
            .follow_links(true)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| devices_re.is_match(&entry.path().to_string_lossy()))
        {
            let Some(vid) = read_hex_from_file(&entry.path().join("idVendor")) else {
                continue;
            };
            let Some(pid) = read_hex_from_file(&entry.path().join("idProduct")) else {
                continue;
            };

            if vid == NVIDIA_TEGRA_VID && pid == NVIDIA_TEGRA_PID {
                return Ok(std::path::PathBuf::from(entry.path()));
            }
        }
        Err(RcmError::DeviceNotFound)
    }
    pub fn read(&self) {}
    pub fn write(&self) {}
    pub fn new() -> Result<Self, RcmError> {
        let sysfs_path = Self::find_device()?;
        Ok(Self { sysfs_path })
    }
}
