/// Manipulate .ssh settings.
///
/// created 2020/07/15
/// updated 2020/09/28
use ssh_config::structs::config::SshConfig;
use ssh_config::structs::app_setting::AppSetting;
// use ssh_config::structs::known_hosts::KnownHosts;
use ssh_config::functions::file_io;
use ssh_config::functions::custom::{ io_custom, regex_cutom };

/// Main function
fn main() {
    // Read App setting file
    let mut app_setting = file_io::read_setting_file();
    if !app_setting.is_set() {
        println!("Application setting file is not found.");
        let input: &str = &io_custom::input("Create new setting file? [y/n] > ").to_lowercase();
        match input {
            "y" | "yes" | "" => {
                app_setting.config_path = io_custom::input("Input config file full path > ");
                if file_io::write_app_setting_file(app_setting.to_string()) {
                    println!("Create file is succeed.");
                }
            }
            _ => {},
        }
    }

    // select operation
    println!("\nPlease specify the file to edit.");
    loop {
        let input: &str = &io_custom::input("> ");
        match input {
            "config" => { 
                if app_setting.config_path == "" {
                    println!("ssh config file path is not definition.");
                } else {
                    operate_config(app_setting.config_path.clone());
                }
            },
            "known_host" => {
                if app_setting.known_host_path == "" {
                    println!("ssh known_hosts file path is not definition.");
                } else {
                    operate_known_hosts(app_setting.known_host_path.clone());
                }
            },
            "setting" => app_setting = operate_setting(app_setting),
            "exit" => return,
            "" => continue,
            "help" | _ => {
                if input != "help" {
                    println!("\n'{}' is not implements", input);
                }
                println!("\nUsage: How to this application.");
                println!("\tconfig: Edit to config file.");
                println!("\tknown_host: Edit to known_host file.");
                println!("\tsetting: Edit to this application setting file.");
                println!("\texit: Finish to this application.");
            }
        }
    }
}


/// config file operation
fn operate_config(config_file_path: String) {
    // get config data
    let mut ssh_configs = file_io::read_config_file(&config_file_path);
    let mut max_id: u32 = 0;

    println!("\nEnter 'new' or 'edit' to edit config, and 'out' to output a config file.");
    println!("Options are shown with 'help'.");

    loop {
        let mut config_ids = Vec::new();
        for config in &ssh_configs {
            if max_id < config.id {
                max_id = config.id;
            }
        }

        let input: &str = &io_custom::input("(CONFIG) > ");
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
                    let word = io_custom::input("(CONFIG)Select id to edit > ");
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
            },
            "delete" => {
                // Select delete target No.
                loop {
                    for config in &ssh_configs {
                        println!("[{}] {}", config.id, config.host);
                        config_ids.push(config.id);
                    }
                    println!("[q] Cancel.");
                    let word = io_custom::input("(CONFIG)Select id to delete > ");
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
            },
            "show" => {
                if ssh_configs.len() != 0 {
                    println!("Print current settings.");
                    println!("_/_/_/_/_/_/_/_/_/_/_/_/");
                    for config in &ssh_configs { 
                        config.print(); 
                    }
                    println!("_/_/_/_/_/_/_/_/_/_/_/_/");
                } else {
                    println!("Current setting is not definition.")
                }
            },
            "out" => {
                let mut out_string = String::new();
                for config in &ssh_configs {
                    out_string = [out_string, config.to_string(), "\n".to_string()].concat();
                }
                if file_io::write_config_file(&config_file_path.clone(), out_string) {
                    println!("Succeed.");
                } else {
                    println!("Failed.");
                }
            },
            "path" => println!("\nSSH config path : {}", &config_file_path.clone()),
            "exit" => break,
            "" => continue,
            "help" | _ => {
                if input != "help" {
                    println!("\n'{}' is not implements", input);
                }
                println!("");
                println!("Usage: How to ssh config file edit function.");
                println!("\tnew : Create new config");
                println!("\tedit : Edit to config.");
                println!("\tdelete : Delete to config.");
                println!("\tshow : Print all current settings.");
                println!("\tout : Output to config file");
                println!("\tpath : Print config file path.");
                println!("\texit : Exit this application.");
            }
        }
        println!("");
    }
}


/// known_hosts file operation
fn operate_known_hosts(config_file_path: String) {
    // get config data
    let mut known_hosts = file_io::read_known_hosts_file(&config_file_path);

    println!("\nEnter 'delete' to delete known_hosts and 'out' to output a file.");
    println!("Options are shown with 'help'.");

    loop {
        let mut config_ids = Vec::new();

        let input: &str = &io_custom::input("(KNOWN-HOSTS) > ");
        match input {
            "delete" => {
                // Select delete target No.
                loop {
                    for host in &known_hosts {
                        if host.port != "" {
                            println!("[{}] {}:{}", host.id, host.host, host.port);
                        } else {
                            println!("[{}] {}:22", host.id, host.host);
                        }
                        config_ids.push(host.id);
                    }
                    println!("[q] Cancel.");
                    let word = io_custom::input("(KNOWN-HOSTS)Select id to delete > ");
                    if regex_cutom::is_number_string(word.clone()) {
                        let number = word.parse().unwrap();
                        if config_ids.contains(&number) {
                            let tmp_config = known_hosts;
                            known_hosts = Vec::new();
                            for config in tmp_config {
                                if config.id != number { known_hosts.push(config); }
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
            },
            "show" => {
                if known_hosts.len() != 0 {
                    println!("Print current settings.");
                    println!("");
                    for config in &known_hosts { 
                        config.print(); 
                    }
                } else {
                    println!("Current setting is not definition.")
                }
            },
            "out" => {
                let mut out_string = String::new();
                for config in &known_hosts {
                    out_string = [out_string, config.to_string(), "\n".to_string()].concat();
                }
                if file_io::write_known_hosts_file(&config_file_path.clone(), out_string) {
                    println!("Succeed.");
                } else {
                    println!("Failed.");
                }
            },
            "path" => println!("\nSSH config path : {}", &config_file_path.clone()),
            "exit" => break,
            "" => continue,
            "help" | _ => {
                if input != "help" {
                    println!("\n'{}' is not implements", input);
                }
                println!("");
                println!("Usage: How to ssh known_hosts file edit function.");
                println!("\tdelete : Delete to config.");
                println!("\tshow : Print all current settings.");
                println!("\tout : Output to config file");
                println!("\tpath : Print config file path.");
                println!("\texit : Exit this application.");
            }
        }
        println!("");
    }
}


/// application setting file operation
fn operate_setting(mut app_setting: AppSetting) -> AppSetting{
    
    println!("\nEnter 'edit' to edit setting, and 'save' to save a settings file.");
    println!("Options are shown with 'help'.");

    loop {
        let input: &str = &io_custom::input("(SETTING) > ");
        match input {
            "edit" => {
                let path: &str = &io_custom::input("Input config file path > ");
                if path != "" {
                    app_setting.config_path = path.to_string();
                }
                let path: &str = &io_custom::input("Input known_hosts file path > ");
                if path != "" {
                    app_setting.known_host_path = path.to_string();
                }
            }
            "save" => {
                if file_io::write_app_setting_file(app_setting.to_string()) {
                    println!("succeed.");
                }
            }
            "show" => app_setting.print(),
            "exit" => break,
            "" => continue,
            "help" | _ => {
                if input != "help" {
                    println!("\n'{}' is not implements", input);
                }
                println!("");
                println!("Usage: How to ssh config file edit function.");
                println!("\tedit : Edit to setting file.");
                println!("\tsave : Print all current settings.");
                println!("\tshow : Print all current settings.");
                println!("\texit : Exit this application.");
            },
        }
        println!("");
    }

    return app_setting
}
