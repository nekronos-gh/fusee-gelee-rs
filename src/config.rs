use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub payload_path: PathBuf,
    pub wait_for_rcm: bool,
    pub vid: Option<u16>,
    pub pid: Option<u16>,
    pub skip_check: bool,
    pub permissive_id: bool,
}
