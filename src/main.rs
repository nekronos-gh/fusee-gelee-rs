mod config;

const NVIDIA_TEGRA_VID: u16 = 0x0955; // Vendor ID for NVIDIA Corporation
const NVIDIA_TEGRA_PID: u16 = 0x7321; // Product ID for NVIDIA Tegra RCM mode

fn main() {
    let config = config::Config::default();
    dbg!(&config);
}
