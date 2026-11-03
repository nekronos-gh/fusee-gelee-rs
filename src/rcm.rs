mod error;
mod usb;

use crate::rcm::error::RcmError;
use usb::USBDevice;

const NVIDIA_TEGRA_VID: u16 = 0x0955; // Vendor ID for NVIDIA Corporation
const NVIDIA_TEGRA_PID: u16 = 0x7321; // Product ID for NVIDIA Tegra RCM mode

// USB Endpoints for RCM Device
const BULK_EP: u32 = 0x1;

#[derive(Debug)]
pub struct TegraRCMDevice {
    device: USBDevice,
}

impl TegraRCMDevice {
    pub fn send_payload(&self) -> Result<(), RcmError> {
        let mut payload = [0xDE, 0xAD, 0xBE, 0xEF];
        self.device
            .write_bulk(BULK_EP, &mut payload)
            .map_err(RcmError::IO)?;
        Ok(())
    }

    pub fn trigger_vulnerability(&self) -> Result<[u8; 256], RcmError> {
        let mut buffer = [0u8; 256];
        self.device
            .write_control(&mut buffer)
            .map_err(RcmError::IO)?;
        Ok(buffer)
    }

    pub fn new() -> Result<Self, RcmError> {
        if let Some(device) = USBDevice::find_device(NVIDIA_TEGRA_VID, NVIDIA_TEGRA_PID) {
            device.claim_interface(0).map_err(RcmError::IO)?;
            return Ok(Self { device });
        }
        Err(RcmError::DeviceNotFound)
    }
}
