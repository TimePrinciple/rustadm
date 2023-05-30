use clap::{Parser, Subcommand};

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
    command: Commnads,
}

#[derive(Subcommand)]
enum Commnads {
    Init,
    Join,
}

pub fn run_command() {
    let cli = Cli::parse();

    match &cli.command {
        Commnads::Init => {
            tracing::info!("Init subcommand invoked.");
        },
        Commnads::Join => {
            tracing::info!("Join subcommand invoked.");
        },
    }
}
