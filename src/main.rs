mod rustadm;
mod init;
mod join;
mod config;
mod install;

use rustadm::run_command;

fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Rustadm started");
    run_command();
}

