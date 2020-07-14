/// Manipulate .ssh settings.
///
/// created 2020/07/15
/// updated 2020/07/15
use std::fs::{ self, read_to_string };
use std::io::{ BufWriter, Write} ;

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
    id: u32,
    host: String,
    hostname: String,
    port: String,
    user: String,
    identity_file: String,
    server_alive_interval: String
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

/// Main function
fn main() {
    // Read file
    let settings = read_setting_file();
    let filename = &settings[0];
    if filename == "" {
        println!("Config file path is not listed.");
        return;
    }
    let mut ssh_configs = read_config_file(&filename);
    let mut max_id: u32 = 0;

    // Operation
    loop {
        let mut config_ids = Vec::new();
        println!("Enter 'new' or 'edit' to edit config, and 'out' to output a config file.");
        println!("Options are shown with 'help'.");
        for config in &ssh_configs {
            if max_id < config.id {
                max_id = config.id;
            }
        }

        let input: &str = &io_custom::input("> ");
        match input {
            "new" => ssh_configs.push(SshConfig::create(max_id + 1)),
            "edit" => {
                // Select edit target No.
                loop {
                    for config in &ssh_configs {
                        println!("[{}] {}", config.id, config.host);
                        config_ids.push(config.id);
                    }
                    println!("[q] Cancel.");
                    let word = io_custom::input("Select id to edit > ");
                    if regex_cutom::is_number_string(word.clone()) {
                        let number = word.parse().unwrap();
                        if config_ids.contains(&number) {
                            for config in &mut ssh_configs {
                                if config.id == number {
                                    config.edit();
                                    break;
                                }
                            }
                            break;
                        } else {
                            println!("There is no '{}' in id.", &word);
                        }
                    } else if word == "q" {
                        break;
                    } else {
                        println!("'{}' is not number.", &word);
                    }
                    println!("");
                }
            }
            "delete" => {
                // Select delete target No.
                loop {
                    for config in &ssh_configs {
                        println!("[{}] {}", config.id, config.host);
                        config_ids.push(config.id);
                    }
                    println!("[q] Cancel.");
                    let word = io_custom::input("Select id to delete > ");
                    if regex_cutom::is_number_string(word.clone()) {
                        let number = word.parse().unwrap();
                        if config_ids.contains(&number) {
                            let tmp_config = ssh_configs;
                            ssh_configs = Vec::new();
                            for config in tmp_config {
                                if config.id != number { ssh_configs.push(config); }
                            }
                            break;
                        } else {
                            println!("There is no '{}' in id.", &word);
                        }
                    } else if word == "q" {
                        break;
                    } else {
                        println!("'{}' is not number.", &word);
                    }
                    println!("");
                }
            }
            "show" => {
                println!("Print current settings.");
                println!("_/_/_/_/_/_/_/_/_/_/_/_/");
                for config in &ssh_configs {
                    config.print();
                    println!("");
                }
                println!("_/_/_/_/_/_/_/_/_/_/_/_/");
            }
            "out" => {
                let mut out_string = String::new();
                for config in &ssh_configs {
                    out_string = [out_string, config.to_string(), "\n".to_string()].concat();
                }
                if write_config_file(&filename, out_string) {
                    println!("Succeed.");
                } else {
                    println!("Failed.");
                }
            }
            "path" => println!("SSH config path : {}", &filename),
            "exit" => break,
            "help" | _ => {
                if input != "help" {
                    println!("\n'{}' is not implements", input);
                }
                println!("");
                println!("Usage");
                println!("    new : Create new config");
                println!("    edit : Edit to config.");
                println!("    delete : Delete to config.");
                println!("    show : Print all current settings.");
                println!("    out : Output to config file");
                println!("    path : Print config file path.");
                println!("    exit : Exit this application.");
            }
        }
        println!("");
    }
}

/// Read setting.txt
/// Get set value and store in Vec
pub fn read_setting_file() -> Vec<String> {
    let file_data = read_to_string("setting.txt");
    let read_str = match file_data {
        Ok(content) => content,
        Err(error) => { panic!("Cloud not open file: {}", error) }
    };

    // Check the line by line and get the set value
    let mut setting_vec = Vec::new();
    let read_lines: Vec<&str> = read_str.trim().split("\n").collect();
    let mut ssh_conf_path = String::new();
    for line in read_lines {
        // Get config file path
        if line.starts_with("ssh_conf:") {
            ssh_conf_path = (&line[9..]).trim().to_string();
        }
    }

    // Set to Vec
    setting_vec.push(ssh_conf_path);

    return setting_vec;
}

/// Read config file
/// Get the contents of the Config file and set it to SshConfig struct
pub fn read_config_file(filename: &str) -> Vec<SshConfig> {
    let file_data = read_to_string(filename);
    let read_str = match file_data {
        Ok(content) => content,
        Err(error) => { panic!("Cloud not open file: {}", error) }
    };

    // Get setting value
    let mut configs = Vec::new();
    let mut ssh_config = SshConfig::new();
    let mut id: u32 = 1;

    let read_lines: Vec<&str> = read_str.trim().split("\n").collect();
    for line in read_lines {
        if line != "" {
            let word: Vec<&str> = line.trim().split_whitespace().collect();
            match word[0] {
                "Host" => {
                    // If you have a host, save it and create a new SshConfig
                    if ssh_config.host != "" {
                        ssh_config.id = id;
                        configs.push(ssh_config);
                        ssh_config = SshConfig::new();
                        id += 1;
                    }
                    ssh_config.host = word[1].to_string();
                }
                "HostName" => ssh_config.hostname = word[1].to_string(),
                "Port" => ssh_config.port = word[1].to_string(),
                "User" => ssh_config.user = word[1].to_string(),
                "IdentityFile" => ssh_config.identity_file = word[1].to_string(),
                "ServerAliveInterval" => ssh_config.server_alive_interval = word[1].to_string(),
                _ => continue,
            }
        }
    }
    // save 
    ssh_config.id = id;
    configs.push(ssh_config);

    return configs;
}

/// Write config file
/// Output string to config file
pub fn write_config_file(filename: &str, write_string: String) -> bool {
    // Fail if the file does not exist
    if !fs::metadata(filename.clone()).is_ok() {
        return false;
    }
    let mut file = BufWriter::new(fs::OpenOptions::new().write(true).open(filename.clone()).unwrap());
    let byte_string = write_string.as_bytes();

    file.write(byte_string).unwrap();

    return true;
}

/// Custom I/O modules
pub mod io_custom {
    use std::io::{ self, Write };

    /// Input function
    /// Type after Text
    pub fn input(text: &str) -> String {
        print!("{}", text);
        let mut input  = String::new();
        let __ = io::stdout().flush();
        std::io::stdin().read_line(&mut input).ok();

        return input.trim().to_string()
    }
}

/// Custom regular expression modules
pub mod regex_cutom {
    extern crate regex;
    use regex::Regex;

    /// Numerical character judgment
    /// True if only number
    pub fn is_number_string(target: String) -> bool {
        let number_regex = Regex::new(r"^[0-9]+$").unwrap();

        if number_regex.is_match(&target) {
            return true;
        }

        return false;
    }
}