pub struct AppSetting {
    pub config_path: String,
    pub known_host_path: String
}

impl AppSetting {
    pub fn new() -> AppSetting {
        AppSetting {
            config_path: String::new(),
            known_host_path: String::new()
        }
    }

    pub fn is_set(&self) -> bool {
        if self.config_path == "" && self.known_host_path == "" {
            return false;
        }
        return true;
    }

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