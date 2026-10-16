mod config;
mod rcm;

fn main() {
    // TODO: Send payload
    match rcm::TegraRCMDevice::new() {
        Ok(device) => {
            dbg!(device);
        }
        Err(e) => eprintln!("Tegra Device could not be instantiated: {e}"),
    }
}
