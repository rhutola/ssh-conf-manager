
use std::fs::{ self, read_to_string };
use std::io::{ BufWriter, Write};
use crate::modules::structs::config::SshConfig;
use crate::modules::structs::app_setting::AppSetting;

/// Read setting.txt
/// Get set value and store in Vec
pub fn read_setting_file() -> AppSetting {
    let file_data = read_to_string("setting.txt");
    let read_str = match file_data {
        Ok(content) => content,
        Err(_error) => return AppSetting::new(),
    };

    // Check the line by line and get the set value
    let read_lines: Vec<&str> = read_str.trim().split("\n").collect();
    let mut app_setting = AppSetting::new();
    for line in read_lines {
        // Get config file path
        if line.starts_with("config_path:") {
            app_setting.config_path = (&line[11..]).trim().to_string();
        }
    }

    return app_setting;
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

/// Write application setting file
/// Output string to confisetting file
pub fn write_app_setting_file(write_string: String) -> bool {
    let file_name = String::from("setting.txt");
    let mut file;

    // Create file if it does not exist
    if fs::metadata(file_name.clone()).is_ok() {
        file = BufWriter::new(fs::OpenOptions::new().write(true).open(file_name.clone()).unwrap());
    } else {
        file = BufWriter::new(fs::File::create(file_name.clone()).unwrap());
    }

    let byte_string = write_string.as_bytes();
    file.write(byte_string).unwrap();

    return true;
}