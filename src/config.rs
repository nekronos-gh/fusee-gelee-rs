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

impl Default for Config {
    fn default() -> Self {
        Self {
            payload_path: std::path::PathBuf::from("./payload"),
            wait_for_rcm: false,
            vid: None,
            pid: None,
            skip_check: false,
            permissive_id: false,
        }
    }
}
