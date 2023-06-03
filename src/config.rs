use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    // The machine running this program.
    pub instance_name: String,
    // The ip address of running server.
    pub instance_ip: String,
    pub instance_hosts: HashMap<String, String>,

    // Fields needed by `install cfssl` command.
    pub cfssl_url: String,
    pub cfssljson_url: String,
    pub cfsslcertinfo_url: String,
    // Fields needed by `install etcd` command.
    pub etcd_url: String,
    
    // Fields needed by `etcd` phase.
    pub etcd_ca_CN: String,
    pub etcd_CN: String,
    pub etcd_key_algo: String,
    pub etcd_key_size: i64,
    pub etcd_expiry: String,
    pub etcd_usages: Vec<String>,
    pub etcd_names_C: String,
    pub etcd_names_L: String,
    pub etcd_names_ST: String,
}

impl Config {
    pub fn init() -> Config {
        tracing::info!("Reading config file...");
        let mut file = File::open("cfg/config.yaml").expect("File `config.yaml` does not exist!");
        let mut content = vec![];
        file.read_to_end(&mut content).expect("Error happened when trying to read content of `config.yaml`");
        let config = serde_yaml::from_slice(&content).expect("Something went wrong while parsing config.yaml");
        tracing::info!("Config read: {:?}", config);
        config
    }
}

pub fn generate_config_template() {
    let config = Config {
        instance_name: "master01".to_owned(),
        instance_ip: "192.168.221.135".to_owned(),
        instance_hosts: {
            let mut map = HashMap::new();
            map.insert(
                "192.168.221.135".to_owned(),
                "master01".to_owned(),
            );
            map.insert(
                "192.168.221.136".to_owned(),
                "worker01".to_owned(),
            );
            map.insert(
                "192.168.221.137".to_owned(),
                "worker02".to_owned(),
            );
            map
        },

        cfssl_url: "https://pkg.cfssl.org/R1.2/cfssl_linux-amd64".to_owned(),
        cfssljson_url: "https://pkg.cfssl.org/R1.2/cfssljson_linux-amd64".to_owned(),
        cfsslcertinfo_url: "https://pkg.cfssl.org/R1.2/cfssl-certinfo_linux-amd64".to_owned(),
        etcd_url: "https://github.com/etcd-io/etcd/releases/download/v3.4.9/etcd-v3.4.9-linux-amd64.tar.gz".to_owned(),

        etcd_ca_CN: "etcd CA".to_owned(),
        etcd_CN: "etcd".to_owned(),
        etcd_key_algo: "rsa".to_owned(),
        etcd_key_size: 2048,
        etcd_expiry: "87600h".to_owned(),
        etcd_usages: vec![
            "signing".to_owned(),
            "key encipherment".to_owned(),
            "server auth".to_owned(),
            "client auth".to_owned(),
        ],
        etcd_names_C: "CN".to_owned(),
        etcd_names_L: "Beijing".to_owned(),
        etcd_names_ST: "Beijing".to_owned(),
    };

    let yaml = serde_yaml::to_string(&config).unwrap();
    let mut file = File::create("cfg/config_template.yaml").unwrap();
    file.write_all(yaml.as_bytes()).unwrap();
}

