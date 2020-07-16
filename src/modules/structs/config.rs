use crate::modules::functions::custom::io_custom;

/// config file struct
///
/// id: Management No used in this application.
/// host: Host item in the config file.
/// hostname: HostName item in the config file.
/// port: Port item in the config file.
/// user: User item in the config file.
/// identity_file: IdentityFile item in the config file.
/// server_alive_interval: ServerAliveInterval item in the config file.
pub struct SshConfig {
    pub id: u32,
    pub host: String,
    pub hostname: String,
    pub port: String,
    pub user: String,
    pub identity_file: String,
    pub server_alive_interval: String
}

impl SshConfig {
    /// Initialization
    /// Set id to 0, else String::new()
    pub fn new() -> SshConfig {
        SshConfig {
            id: 0,
            host: String::new(),
            hostname: String::new(),
            port: String::new(),
            user: String::new(),
            identity_file: String::new(),
            server_alive_interval: String::new()
        }
    }

    /// Create a new setting
    /// Interactively enter only Host item as a required item.
    /// Id is an argument.
    pub fn create(id: u32) -> SshConfig {
        // Input Host
        let host;
        loop {
            let word = io_custom::input("Host: ");
            if word != "" {
                host = word.to_string();
                break;
            }
        }

        // Create SshConfig
        SshConfig {
            id: id,
            host: host,
            hostname: io_custom::input("HostName: "),
            port: io_custom::input("Port: "),
            user: io_custom::input("User: "),
            identity_file: io_custom::input("IdentityFile: "),
            server_alive_interval: io_custom::input("ServerAliveInterval: "),
        }
    }

    /// Edit existing setting
    /// Content is not saved until type 'save'
    pub fn edit (&mut self) {
        loop {
            // Print current settings
            println!("");
            println!("Please select the number of the item you want to edit.");
            println!("Enter 'save' to quit");
            println!("[1] Host: {}", self.host);
            println!("[2] HostName: {}", self.hostname);
            println!("[3] Port: {}", self.port);
            println!("[4] User: {}", self.user);
            println!("[5] IdentifyFile: {}", self.identity_file);
            println!("[6] ServerAliveinterval: {}", self.server_alive_interval);

            let word: &str = &io_custom::input("> ");
            match word {
                "save" => break,
                "1" => loop {
                    let host = io_custom::input("Host: ");
                    if host != "" {
                        self.host = host;
                        break;
                    }
                }
                "2" => self.hostname = io_custom::input("HostName: "),
                "3" => self.port = io_custom::input("Port: "),
                "4" => self.user = io_custom::input("User: "),
                "5" => self.identity_file = io_custom::input("IdentifyFile: "),
                "6" => self.server_alive_interval = io_custom::input("ServerAliveinterval: "),
                _ => println!("not implemented."),
            }
        }
    }

    /// Print current settings
    /// Output in file description format
    pub fn print(&self) {
        if self.host != "" { println!("Host {}", self.host) };
        if self.hostname != "" { println!("    HostName {}", self.hostname) };
        if self.port != "" { println!("    Port {}", self.port) };
        if self.user != "" { println!("    User {}", self.user) };
        if self.identity_file != "" { println!("    IdentityFile {}", self.identity_file) };
        if self.server_alive_interval != "" { println!("    ServerAliveInterval {}", self.server_alive_interval) };
    }

    /// Convert settings to string
    /// Convert to file description format
    pub fn to_string(&self) -> String {
        let mut setting_str = String::new();
        let lf = "\n".to_string();

        if self.host != "" { setting_str = [setting_str, "Host ".to_string(), self.host.clone(), lf.clone()].concat(); }
        if self.hostname != "" { setting_str = [setting_str, "    HostName ".to_string(), self.hostname.clone(), lf.clone()].concat(); }
        if self.port != "" { setting_str = [setting_str, "    Port ".to_string(), self.port.clone(), lf.clone()].concat(); }
        if self.user != "" { setting_str = [setting_str, "    User ".to_string(), self.user.clone(), lf.clone()].concat(); }
        if self.identity_file != "" { setting_str = [setting_str, "    IdentityFile ".to_string(), self.identity_file.clone(), lf.clone()].concat(); }
        if self.server_alive_interval != "" { setting_str = [setting_str, "    ServerAliveInterval ".to_string(), self.server_alive_interval.clone(), lf.clone()].concat(); }
        
        return setting_str
    }
}