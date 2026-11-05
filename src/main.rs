mod config;
mod rcm;

fn main() {
    match rcm::TegraRCMDevice::new() {
        Ok(device) => {
            println!("Found Nvidia Tegra RCM Device: {}", device);
            // device
            //     .send_payload()
            //     .expect("Could not send payload to RCM device");
            let data = device
                .trigger_vulnerability()
                .expect("Could not trigger the buffer overflow on the RCM device");
            println!("Data read from memory \n {:02X?}", data);
        }
        Err(e) => eprintln!("Tegra Device could not be instantiated: {e}"),
    }
}
