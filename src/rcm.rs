mod error;
mod usb;

use error::RcmError;
use usb::USBDevice;

const NVIDIA_TEGRA_VID: u16 = 0x0955; // Vendor ID for NVIDIA Corporation
const NVIDIA_TEGRA_PID: u16 = 0x7321; // Product ID for NVIDIA Tegra RCM mode

// USB Endpoints for RCM Device
const BULK_EP_OUT: u32 = 0x1;
const BULK_EP_IN: u32 = 0x81;

// DMA buffer size
const DMA_BUFFER_SIZE: usize = 0x1000;

#[derive(Debug)]
pub struct TegraRCMDevice {
    device: USBDevice,
    pub id: [u8; 16],
}

impl std::fmt::Display for TegraRCMDevice {
    // Display the device ID when printing the struct
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x")?;
        for byte in self.id.iter().rev() {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

impl TegraRCMDevice {
    // Send user defined payload to the nintendo
    pub fn send_payload(&self) -> Result<(), RcmError> {
        // RCM command size 680, max payload size 0x3000
        const RCM_SIZE: usize = 0x2A8;
        // Header
        let mut rcm_command: Vec<u8> = Vec::new();
        // The RCM protocol checks the first 4 bytes of the comand, which is going to be the command size
        // RCM CHECK: Less than 0x30000 + 0x2A8 && is multiple of 8 && is greater than 0x400
        let header: u32 = 0x30298; // Highest number that passes the test with flying colors
        rcm_command.extend_from_slice(&header.to_le_bytes());
        rcm_command.resize(RCM_SIZE, 0u8); // Fill the command until payload

        // Set our payload: Nekronos is inside the switch
        let payload: [u8; 30] = [
            0x4E, 0x65, 0x6B, 0x72, 0x6F, 0x6E, 0x6F, 0x73, 0x20, 0x69, 0x73, 0x20, 0x69, 0x6E,
            0x73, 0x69, 0x64, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x77, 0x69, 0x74, 0x63,
            0x68, 0x00,
        ];
        // Write a full buffer size
        while rcm_command.len() < DMA_BUFFER_SIZE {
            rcm_command.extend_from_slice(&payload);
        }
        // Trim it
        rcm_command.resize(DMA_BUFFER_SIZE, 0u8);

        // Issue the write, since the data is smaller than the size specified inthe header, the RCM
        // protocol waits until more data is to come, while still serving control requests.
        self.device
            .issue_bulk_request(BULK_EP_OUT, &mut rcm_command)?;

        Ok(())
    }

    pub fn trigger_vulnerability(&self) -> Result<[u8; 2048], RcmError> {
        let mut buffer = [0u8; 2048];
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
