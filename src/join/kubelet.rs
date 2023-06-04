use crate::config::Config;
use std::path::PathBuf;
use std::fs;



pub fn start(config: &Config) {
    let kubernetes_bin = PathBuf::from("/opt/kubernetes/bin");
    check_dir_exist_or_create(kubernetes_bin);
    let kubernetes_cfg= PathBuf::from("/opt/kubernetes/cfg");
    check_dir_exist_or_create(kubernetes_cfg);
    let kubernetes_ssl= PathBuf::from("/opt/kubernetes/ssl");
    check_dir_exist_or_create(kubernetes_ssl);
    let kubernetes_logs= PathBuf::from("/opt/kubernetes/logs");
    check_dir_exist_or_create(kubernetes_logs);

    
}

fn check_dir_exist_or_create(path: PathBuf) {
    if !path.is_dir() {
        fs::create_dir_all(path).expect("Error happened when trying to create path");
    }
}
