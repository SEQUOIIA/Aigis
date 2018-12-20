extern crate pretty_env_logger;
extern crate log;
extern crate clap;
extern crate serde_json;

use std::fs::File;
use std::path::Path;
use log::{debug};
use std::io::{Read, BufReader, BufRead};
use std::io::prelude::*;

use clap::{Arg, ArgMatches};

fn main() {
    let clap_args : ArgMatches = clap::App::new("analyser")
        .version("0.1.0")
        .about("Analyse JSON dumps from the dumper tool")
        .author("Emil H. Clausen (SEQUOIIA) <sequoiia@hummel.yt>")

        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .help("Path to the JSON dump")
            .takes_value(true)
            .index(1)
            .required(true))

        .arg(Arg::with_name("output-path")
            .short("o")
            .long("output-path")
            .help("Path to directory where Rust structs will be output")
            .takes_value(true)
            .default_value("output"))

        .arg(Arg::with_name("debug")
            .short("d")
            .long("debug"))

        .arg(Arg::with_name("legacy")
            .short("l")
            .long("legacy")
            .help("Include this arg for parsing legacy websocket dumps"))
        .get_matches();

    match clap_args.is_present("debug") {
        true => setup(),
        false => {},
    }

    let path_arg : &str = match clap_args.value_of("path") {
        Some(s) => s,
        None => {
            println!("No path was provided\n\n{}", clap_args.usage());
            std::process::exit(1);
        },
    };

    let path : &Path = Path::new(path_arg);

    if !path.exists() {
        println!("No file found at \"{}\"", path.to_str().unwrap());
        std::process::exit(1);
    } else {
        match clap_args.is_present("legacy") {
            true => {parse_file_legacy(path, clap_args.value_of("output-path").unwrap());},
            false => {parse_file_legacy(path, clap_args.value_of("output-path").unwrap());},
        }
    }
}

fn parse_file_legacy(path : &Path, output_path : &str) {
    let file : File = File::open(path).unwrap();

    let mut reader : BufReader<File> = BufReader::new(file);
    let mut line : String = String::new();
    reader.read_line(&mut line);
    let mut loop_check : bool = true;

    let mut files : std::collections::HashSet<String> = std::collections::HashSet::new();
    std::fs::create_dir(output_path).unwrap();

    let path_raw : String = format!("{}/AIGIS_uri-list", output_path);
    let path : &Path = Path::new(path_raw.as_str());
    let mut uri_file : File = File::create(path).unwrap();

    loop {
        let mut buf_line : String = String::new();
        if reader.read_line(&mut buf_line).unwrap() == 0 {
            loop_check = false;
        }

        if !loop_check {
            break;
        }

        if buf_line.contains("][") {
            line.push_str(buf_line.as_str());
            line.truncate(line.len() - 2);

            let new_line_char = line.pop();
            uri_gatherer(line.clone(), &mut files, output_path, &mut uri_file);
            line = String::new();
            line.push(new_line_char.unwrap());
            line.push('\n');
        } else {
            line.push_str(buf_line.as_str());
        }
    }
}

fn uri_gatherer(payload : String, files : &mut std::collections::HashSet<String>, output_path : &str, uri_file : &mut File) {
    match serde_json::from_str::<serde_json::Value>(payload.as_str()) {
        Ok(v) => {
            let v_array : &Vec<serde_json::Value> = v.as_array().unwrap();

            if let Some(v_object) = v_array[2].as_object() {
                let uri : &str = v_object.get("uri").unwrap().as_str().unwrap();

                let mut file_name : String = uri.clone().replace("/", "_");
                file_name = file_name.replace(":", "-");
                if !files.contains(file_name.as_str()) {
                    files.insert(file_name.clone());
                    let mut path_raw : String = format!("{}/{}.json", output_path, file_name.as_str());
                    let path : &Path = Path::new(path_raw.as_str());
                    debug!("{:?}", path);
                    let mut file : File = File::create(path).unwrap();
                    let payload : String = v_array[2].to_string();
                    file.write_all(payload.as_bytes());

                    // uri_file line
                    let uri_file_line : String = format!("{} - {}.json\n", uri, file_name);
                    uri_file.write_all(uri_file_line.as_bytes());
                }
            }
        },
        Err(_e) => {}
    };
}

#[cfg(debug_assertions)]
fn setup() {
    std::env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();
}

#[cfg(not(debug_assertions))]
fn setup() {

}
