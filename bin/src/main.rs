extern crate pretty_env_logger;
extern crate log;

use aigis_lib;
use aigis_lib::LCU::LockFile;
use std::env;
use log::{debug};

fn main() {
    setup();

    let mut client: aigis_lib::LCU::LCUClient = aigis_lib::LCU::LCUClient::new();
    let lockfile : LockFile = LockFile::get_connection_info().unwrap();
    println!("{} || {}", lockfile.password, lockfile.port);
    client.load_lockfile(lockfile);
    client.lcu_status();
    client.listen_to_websocket();
}

#[cfg(debug_assertions)]
fn setup() {
    env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();
}

#[cfg(not(debug_assertions))]
fn setup() {

}
