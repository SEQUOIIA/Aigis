extern crate pretty_env_logger;
extern crate log;
extern crate clap;

use aigis_lib;
use aigis_lib::tungstenite::Message;
use clap::{Arg, ArgMatches, SubCommand, App};


fn main() {
    let mut app : App = clap::App::new("dumper")
        .version("0.1.0")
        .about("Dump data straight from LCU")
        .author("Emil H. Clausen (SEQUOIIA) <sequoiia@hummel.yt>")

        .arg(Arg::with_name("league-path")
            .short("p")
            .long("league-path")
            .help("Path to the League of Legends directory")
            .takes_value(true))

        .arg(Arg::with_name("debug")
            .short("d")
            .long("debug"))

        .subcommand(SubCommand::with_name("websocket")
            .about("Dump data from LCU websocket"))

        .subcommand(SubCommand::with_name("swagger")
            .about("Dump swagger.json from LCU"));

    let clap_args : ArgMatches = app.clone().get_matches();

    match clap_args.is_present("debug") {
        true => setup(),
        false => {},
    }

    let mut client: aigis_lib::LCU::LCUClient = aigis_lib::LCU::LCUClient::new();
    
    match clap_args.subcommand() {
        ("websocket", Some(_data)) => {
            let lockfile : aigis_lib::LCU::LockFile = aigis_lib::LCU::LockFile::get_connection_info().unwrap();
            client.load_lockfile(lockfile);

            use std::fs::File;
            use std::path::Path;
            use std::io::prelude::*;

            let path = Path::new("aigis_websocket_dump.json");

            let mut file_dump = File::create(&path).unwrap();

            let mut socket = client.get_socket();

            socket.write_message(Message::Text("[5, \"OnJsonApiEvent\"]".to_owned())).unwrap();

            loop {
                let msg = socket.read_message().expect("Err reading MSG");
                println!("{}", msg);
                //file_dump.write_all(msg.into_text().unwrap().as_bytes());
                file_dump.write_all(msg.into_data().as_slice()).unwrap();
            }
        },
        ("swagger", Some(_data)) => {
            let lockfile : aigis_lib::LCU::LockFile = aigis_lib::LCU::LockFile::get_connection_info().unwrap();
            client.load_lockfile(lockfile);

            client.get_swagger();
        }
        ("", None) => {
            app.print_help().unwrap();
        },
        _ => {},
    }
}

#[cfg(debug_assertions)]
fn setup() {
    std::env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();
}

#[cfg(not(debug_assertions))]
fn setup() {

}
