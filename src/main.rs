/// Manipulate .ssh settings.
///
/// created 2020/07/15
/// updated 2020/07/20
use ssh_config::structs::config::SshConfig;
// use ssh_config::structs::app_setting::AppSetting;
use ssh_config::functions::file_io;
use ssh_config::functions::custom::{ io_custom, regex_cutom };

/// Main function
fn main() {
    // Read file
    let mut app_setting = file_io::read_setting_file();
    if !app_setting.is_set() {
        println!("Setting file is not found.");
        let input: &str = &io_custom::input("Create new setting file? [y/n] > ").to_lowercase();
        app_setting.config_path = io_custom::input("Input config file full path > ");
        match input {
            "y" | "yes" | "" => {
                if file_io::write_app_setting_file(app_setting.to_string()) {
                    println!("Create succeed.");
                }
            }
            _ => {},
        }
    }

    let mut ssh_configs = file_io::read_config_file(&app_setting.config_path);
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
                if file_io::write_config_file(&app_setting.config_path.clone(), out_string) {
                    println!("Succeed.");
                } else {
                    println!("Failed.");
                }
            }
            "path" => {
                println!("\nSSH config path : {}", &app_setting.config_path.clone());
                println!("Change path or output to setting file.");
                println!("'edit', 'save', 'exit'.");
                loop {
                    let word: &str = &io_custom::input("conf-path > ");
                    match word {
                        "edit" => {
                            let path: &str = &io_custom::input("Input config file path > ");
                            if path != "" {
                                app_setting.config_path = path.to_string();
                            }
                        }
                        "save" => {
                            if file_io::write_app_setting_file(app_setting.to_string()) {
                                println!("succeed.");
                            }
                        }
                        "exit" => break,
                        _ => continue,
                    }
                    println!("");
                }
            }
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

    return;
}
