use std::ffi::{c_ulong, c_void};
use std::mem::size_of;

// USB Control Transfer Request definition
// include/uapi/linux/usbdevice_fs.h:40
#[repr(C)]
struct USBControlTransfer {
    req_type: u8,
    req: u8,
    value: u16,
    index: u16,
    length: u16,
    timeout: u32,
    data: *mut c_void,
}

// USB Bulk Transfer Request definition
// include/uapi/linux/usbdevice_fs.h:50
#[repr(C)]
struct USBBulkTransfer {
    ep: u32,
    len: u32,
    timeout: u32,
    data: *mut c_void,
}

struct USBDevice {}

// IOCTL Operation number
// Defined as a recursive macro in the kernel
// include/uapi/linux/usbdevice_fs.h:187
// #define USBDEVFS_CONTROL _IOWR('U', 0, struct usbdevfs_ctrltransfer)
// #define USBDEVFS_BULK		_IOWR('U', 2, struct usbdevfs_bulktransfer)

// include/uapi/asm-generic/ioctl.h:88
// #define _IOWR(type,nr,argtype)	_IOC(_IOC_READ|_IOC_WRITE,(type),(nr),(_IOC_TYPECHECK(argtype)))

// include/uapi/asm-generic/ioctl.h:69
// #define _IOC(dir,type,nr,size) \
// 	(((dir)  << _IOC_DIRSHIFT) | \
// 	 ((type) << _IOC_TYPESHIFT) | \
// 	 ((nr)   << _IOC_NRSHIFT) | \
// 	 ((size) << _IOC_SIZESHIFT))

#[inline(always)]
const fn iocwr_usbdev(request_size: u32, request_number: u32) -> u32 {
    const USBDEV_REQ_TYPE: u32 = b'U' as u32; // USB ioctl identifier
    const IOC_WRITE: u32 = 0x01;
    const IOC_READ: u32 = 0x02;
    const DIRECTION: u32 = IOC_READ | IOC_WRITE;
    (DIRECTION << 30/*DIRSHIFT*/)
        | (request_size << 16/*TYPESHIFT*/)
        | (USBDEV_REQ_TYPE << 8/*NRSHIFT*/)
        | request_number /*SIZESHIFT*/
}

const IOCTL_USB_CTRL: u32 = iocwr_usbdev(size_of::<USBControlTransfer>() as u32, 0);
const IOCTL_USB_BULK: u32 = iocwr_usbdev(size_of::<USBBulkTransfer>() as u32, 2);

impl USBDevice {
    fn write(/*fd: i32*/) {
        // Dummy workflow
        // let endpoint: u32 = 0x1;
        // let mut data: u32 = 0xDEADBEEF;
        // let mut bulk_request = USBBulkTransfer {
        //     ep: endpoint,
        //     len: size_of::<u32>() as u32,
        //     timeout: 0,
        //     data: (&mut data as *mut u32).cast::<c_void>(),
        // };
        // unsafe {
        //     libc::ioctl(
        //         fd,
        //         IOCTL_USB_BULK as c_ulong,
        //         &mut bulk_request as *mut USBBulkTransfer,
        //     )
        // };
    }
}
