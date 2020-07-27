/// application settings file struct
///
/// config_path: .ssh/config file path
/// known_host: known_host file path
pub struct AppSetting {
    pub config_path: String,
    pub known_host_path: String
}

impl AppSetting {
    /// Initialization
    /// Set String::new()
    pub fn new() -> AppSetting {
        AppSetting {
            config_path: String::new(),
            known_host_path: String::new()
        }
    }

    /// The settings must be defined
    pub fn is_set(&self) -> bool {
        if self.config_path == "" && self.known_host_path == "" {
            return false;
        }
        return true;
    }
    
    /// Print current settings
    pub fn print(&self) {
        if self.config_path != "" { println!("ssh config file path: {}", self.config_path) };
        if self.known_host_path != "" { println!("ssh known_host file path: {}", self.known_host_path) };
    }

    /// Convert settings to string
    /// Convert to file description format
    pub fn to_string(&self) -> String {
        let mut setting_str = String::new();
        let lf = "\n".to_string();

        if self.config_path != "" { 
            setting_str = [setting_str, "# .ssh/config Path". to_string(), lf.clone()].concat();
            setting_str = [setting_str, "config_path: ".to_string(), self.config_path.clone(), lf.clone()].concat();
            setting_str = [setting_str, lf.clone()].concat();
            setting_str = [setting_str, "# .ssh/known_host Path". to_string(), lf.clone()].concat();
            setting_str = [setting_str, "known_host_path: ".to_string(), self.known_host_path.clone(), lf.clone()].concat();
        }

        return setting_str;
    }

}