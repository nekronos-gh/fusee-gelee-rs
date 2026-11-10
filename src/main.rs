mod config;
mod rcm;

fn main() {
    let device = rcm::TegraRCMDevice::new().expect("Could not connect NVIDIA Tegra RCM Device");
    println!("Connected to Nvidia Tegra RCM Device: {}", device);

    // Set our payload: Nekronos is inside the switch
    let payload: [u8; 30] = [
        0x4E, 0x65, 0x6B, 0x72, 0x6F, 0x6E, 0x6F, 0x73, 0x20, 0x69, 0x73, 0x20, 0x69, 0x6E, 0x73,
        0x69, 0x64, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x77, 0x69, 0x74, 0x63, 0x68, 0x00,
    ];
    device
        .send_payload(&payload)
        .expect("Could not send payload to device");

    let data = device
        .trigger_vulnerability()
        .expect("Could not trigger the vulneravility in the device");
    hexdump::hexdump(&data);
}
