mod config;
mod rcm;

fn main() {
    let device = rcm::TegraRCMDevice::new().expect("Could not connect NVIDIA Tegra RCM Device");
    println!("Connected to Nvidia Tegra RCM Device: {}", device);

    device
        .send_payload()
        .expect("Could not send payload to device");

    let data = device
        .trigger_vulnerability()
        .expect("Could not trigger the vulneravility in the device");
    hexdump::hexdump(&data);
}
