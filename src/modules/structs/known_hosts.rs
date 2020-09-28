/// known_hosts file struct
///
/// id: Management No used in this application.
/// host: Host item in the known_hosts file.
/// port: Port No item in the known_hosts file.
pub struct KnownHosts {
    pub id: u32,
    pub host: String,
    pub port: String,
    pub key_type: String,
    pub key: String
}

impl KnownHosts {
    /// Initialization
    /// Set id to 0, else String::new()
    pub fn new() -> KnownHosts {
        KnownHosts {
            id: 0,
            host: String::new(),
            port: String::new(),
            key_type: String::new(),
            key: String::new()
        }
    }

    /// Print current settings
    /// Output in file description format
    pub fn print(&self) {
        if self.port != "" {
            println!("[{}]:{} {} {}", self.host, self.port, self.key_type, self.key)
        } else {
            println!("{} {} {}", self.host, self.key_type, self.key)
        }
    }

    /// Convert settings to string
    /// Convert to file description format
    pub fn to_string(&self) -> String {
        let mut settings_str = String::new();
        let space = " ".to_string();

        if self.port != "" {
            settings_str = [settings_str, "[".to_string(), self.host.clone(), "]".to_string(), ":".to_string(), self.port.clone()].concat();
        } else {
            settings_str = [settings_str, self.host.clone()].concat();
        }
        settings_str = [settings_str, space.clone(), self.key_type.clone(), space.clone(), self.key.clone()].concat();

        return settings_str;
    }
}