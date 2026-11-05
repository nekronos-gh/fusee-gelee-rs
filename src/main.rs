mod config;
mod rcm;

fn main() {
    match rcm::TegraRCMDevice::new() {
        Ok(device) => {
            println!("Found Nvidia Tegra RCM Device: {}", device);
            device
                .send_payload()
                .expect("Could not send payload to RCM device");
            let data = device
                .trigger_vulnerability()
                .expect("Could not trigger the buffer overflow on the RCM device");
            let _ = dbg!("{:?}", hexdump::hexdump(&data));
        }
        Err(e) => eprintln!("Tegra Device could not be instantiated: {e}"),
    }
}
