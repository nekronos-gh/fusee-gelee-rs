mod error;
mod usb;

use error::RcmError;
use usb::USBDevice;

const NVIDIA_TEGRA_VID: u16 = 0x0955; // Vendor ID for NVIDIA Corporation
const NVIDIA_TEGRA_PID: u16 = 0x7321; // Product ID for NVIDIA Tegra RCM mode

// USB Endpoints for RCM Device
const BULK_EP_OUT: u32 = 0x1;
const BULK_EP_IN: u32 = 0x81;

#[derive(Debug)]
pub struct TegraRCMDevice {
    device: USBDevice,
    pub id: [u8; 16],
}

impl std::fmt::Display for TegraRCMDevice {
    // Display the device ID when printing the struct
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x")?;
        for byte in self.id {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

impl TegraRCMDevice {
    pub fn send_payload(&self) -> Result<(), RcmError> {
        // RCM command size 680, payload size 0x3000
        const RCM_SIZE: usize = 680;
        // Header
        let mut rcm_command: Vec<u8> = vec![0x00, 0x03, 0x02, 0x98]; // Must end in 8, has to be less than 0x3000 + 680 and greater than 0x400
        rcm_command.resize(RCM_SIZE, 0u8);

        let mut payload: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
        rcm_command.extend_from_slice(&payload);

        self.device.issue_bulk_request(BULK_EP_OUT, &mut payload)?;
        Ok(())
    }

    pub fn trigger_vulnerability(&self) -> Result<[u8; 256], RcmError> {
        let mut buffer = [0u8; 256];
        self.device.issue_control_request(&mut buffer)?;
        Ok(buffer)
    }

    // Return a newly instantiated device ready for communication
    pub fn new() -> Result<Self, RcmError> {
        let device = USBDevice::find_device(NVIDIA_TEGRA_VID, NVIDIA_TEGRA_PID)?;
        device.claim_interface(0)?;
        // Read device ID: handshake to enable communication
        let mut id: [u8; 16] = [0u8; 16];
        device.issue_bulk_request(BULK_EP_IN, &mut id)?;
        Ok(Self { device, id })
    }
}
