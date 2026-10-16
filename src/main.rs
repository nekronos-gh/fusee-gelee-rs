mod config;
mod rcm;

fn main() {
    // TODO: Send payload
    if let Ok(vulnerable_device) = rcm::TegraRCMDevice::new() {
        dbg!(vulnerable_device);
    }
}
