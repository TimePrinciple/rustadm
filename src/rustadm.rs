use clap::{Parser, Subcommand};
use crate::init;
use crate::join;
use crate::config::Config;
use crate::config;
use crate::install;
use std::env;
use std::path::Path;
use std::fs;

#[derive(Parser)]
#[command(name = "Rustadm")]
#[command(author = "Ruoqing He <timeprinciple@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "A rust implementation of Kubeadm, easily bootstrap a secure Kubernetes cluster", long_about = "
    
Introduction:

    ┌──────────────────────────────────────────────────────────┐
    │ RUSTADM                                                  │
    │ A kubeadm implementation written in Rust                 │ 
    │ Easily bootstrap a secure Kubernetes cluster             │
    │                                                          │
    │ Please give us feedback at:                              │
    │                                                          │  
    └──────────────────────────────────────────────────────────┘

Example usage:

    Create a two-machine cluster with one control-plane node
    (which controls the cluster), and one worker node
    (where your workloads, like Pods and Deployments run).

    ┌──────────────────────────────────────────────────────────┐
    │ On the first machine:                                    │
    ├──────────────────────────────────────────────────────────┤
    │ control-plane# kubeadm init                              │
    └──────────────────────────────────────────────────────────┘

    ┌──────────────────────────────────────────────────────────┐
    │ On the second machine:                                   │
    ├──────────────────────────────────────────────────────────┤
    │ worker# kubeadm join <arguments-returned-from-init>      │
    └──────────────────────────────────────────────────────────┘

    You can then repeat the second step on as many other machines as you like.
")]
struct Cli {
    #[arg(long)]
    flag1: Option<String>,
    #[arg(long)]
    flag2: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Join,
    Install {
        target: String,
    },
    Generate {
        target: String,
    },
}

pub fn run_command() {
    // Change working directory
    let work_root = Path::new("/rustadm");
    if !work_root.is_dir() {
        fs::create_dir("/rustadm").expect("Error happened when trying to create `/rustadm` directory");
        fs::create_dir("/rustadm/cfg").expect("Error happened when trying to create `/rustadm/cfg` directory");
        fs::create_dir("/rustadm/etcd").expect("Error happened when trying to create `/rustadm/etcd` directory");
    }
    env::set_current_dir(&work_root).expect("Error happened when trying to change working directory");

    let cli = Cli::parse();

    

    match &cli.command {
        Commands::Init => {
            // Read configuration file.
            let adm_config = Config::init();
            tracing::info!("Init subcommand invoked.");
            init::pre_check::start(&adm_config);
            init::etcd::start(&adm_config);
        },
        Commands::Join => {
            // Read configuration file.
            let adm_config = Config::init();
            tracing::info!("Join subcommand invoked.");
            init::pre_check::start(&adm_config);
            join::etcd::start(&adm_config);
        },
        Commands::Install { target } => {
            // Read configuration file.
            let adm_config = Config::init();
            match target.as_str() {
                "etcd" => {
                    tracing::info!("Installing etcd...");
                    install::etcd::start(&adm_config);
                },
                "cfssl" => {
                    tracing::info!("Installing cfssl...");
                    install::cfssl::start(&adm_config);
                    tracing::info!("cfssl installation complete");
                },
                _ => {
                    tracing::info!("Unknown target");
                },
            }           
        },
        Commands::Generate { target } => {
            // Generate `config_template` do not require reading configuration.
            match target.as_str() {
                "config" => {
                    tracing::info!("Generating `config-template.yaml`...");
                    config::generate_config_template();
                    tracing::info!("`config_template.yaml` generated under `rustadm/cfg`");
                },
                _ => {
                    tracing::info!("Unknown target");
                },
            }           
        },
    }
}
