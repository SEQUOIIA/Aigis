use super::futures::future::{ok, Future};
use http::request::Builder;
use futures::stream::Stream;
use std::io;
use std::io::Write;
use std::fs::File;
use std::io::prelude::Read;
use base64;

//Tungstenite
use tungstenite::{Message, connect_unsecure};
use tungstenite::handshake::client::Request;
use url::Url;
use std::borrow::Cow;

pub mod error;
pub mod socket;

pub struct LCUClient {
    state : LCUClientState
}

impl LCUClient {
    pub fn new() -> Self {
        LCUClient {
            state: LCUClientState {
                lockfile: LockFile::new()
            }
        }
    }

    pub fn lcu_status(&self) {
        let client = reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .build()
            .unwrap();

        let auth = format!("riot:{}", self.state.lockfile.password);
        let auth_encoded = base64::encode(&auth);
        let endpoint : String = format!("https://127.0.0.1:{}/lol-service-status/v1/lcu-status", self.state.lockfile.port);
        let mut res = client.get(endpoint.as_str())
            .header("Authorization", format!("Basic {}", auth_encoded))
            .send().unwrap();

        println!("{}", res.text().unwrap());
    }

    pub fn get_swagger(&self) {
        let client = reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .build()
            .unwrap();

        let auth = format!("riot:{}", self.state.lockfile.password);
        let auth_encoded = base64::encode(&auth);
        let endpoint : String = format!("https://127.0.0.1:{}/swagger/v2/swagger.json", self.state.lockfile.port);
        let mut res = client.get(endpoint.as_str())
            .header("Authorization", format!("Basic {}", auth_encoded))
            .send().unwrap();

        println!("{}", res.text().unwrap());
    }

    pub fn load_lockfile(&mut self, lockfile : LockFile) {
        self.state.lockfile = lockfile;
    }

    pub fn listen_to_websocket(&self) {
        //let endpoint = format!("wss://riot:{}@127.0.0.1:{}/", self.state.lockfile.password, self.state.lockfile.port);
        let endpoint = format!("wss://localhost:{}/", self.state.lockfile.port);
        //let endpoint = format!("wss://echo.websocket.org");
        println!("{}", endpoint);

        let auth = format!("riot:{}", self.state.lockfile.password);
        let auth_encoded = base64::encode(&auth);
        let mut headers : Vec<(Cow<str>, Cow<str>)> = Vec::new();
        let auth_val = format!("Basic {}", auth_encoded);
        println!("{}", auth_val);
        headers.push((Cow::Borrowed("Authorization"), Cow::Borrowed(auth_val.as_str())));
        headers.push((Cow::Borrowed("Sec-WebSocket-Protocol"), Cow::Borrowed("wamp")));

        let req = Request {url: Url::parse(endpoint.as_str()).unwrap(), extra_headers: Some(headers)};
        let (mut socket, _response) = connect_unsecure(req)
            .expect("Can't connect");

        socket.write_message(Message::Text("[5, \"OnJsonApiEvent\"]".to_owned())).unwrap();
        loop {
            let msg = socket.read_message().expect("Err reading MSG");
            println!("{}", msg);
        }
    }

    pub fn get_socket(&self) -> tungstenite::WebSocket<tungstenite::AutoStream> {
        let endpoint = format!("wss://localhost:{}/", self.state.lockfile.port);

        let auth = format!("riot:{}", self.state.lockfile.password);
        let auth_encoded = base64::encode(&auth);
        let mut headers : Vec<(Cow<str>, Cow<str>)> = Vec::new();
        let auth_val = format!("Basic {}", auth_encoded);
        headers.push((Cow::Borrowed("Authorization"), Cow::Borrowed(auth_val.as_str())));
        headers.push((Cow::Borrowed("Sec-WebSocket-Protocol"), Cow::Borrowed("wamp")));

        let req = Request {url: Url::parse(endpoint.as_str()).unwrap(), extra_headers: Some(headers)};
        let (socket, _response) = connect_unsecure(req)
            .expect("Can't connect");
        return socket;
    }
}

struct LCUClientState {
    lockfile : LockFile
}

pub struct LockFile {
    unknown_0 : String,   // 0
    unknown_1 : String,   // 1
    pub port : i32,           // 2
    pub password : String,    // 3
    pub protocol : String,    // 4
}

impl LockFile {
    fn new() -> Self {
        LockFile {
            unknown_0: String::new(),
            unknown_1: String::new(),
            port: 0,
            password: String::new(),
            protocol: String::new()
        }
    }

    // Finds and reads a lockfile provided by the League of Legends client, afterwards parsing it.
    // Example of the content in a lockfile: LeagueClient:280:50969:o0ZibhVkG8Qsj98xbFFz2Q:https
    pub fn get_connection_info() -> Result<LockFile, String> {
        use std::process::{Command, Output, Stdio};
        use std::path::Path;
        use regex::{Regex, Captures};

        let mut path : String = String::new();

        let re_output : String = if cfg!(target_os = "windows") {
            "\"--install-directory=(.*?)\"".to_owned()
        } else {
            "--install-directory=(.*?/LoL)".to_owned()
        };

        let process_output : String = if cfg!(target_os = "windows") {
            let output = Command::new("WMIC")
                .args(&["PROCESS", "WHERE", "name='LeagueClientUx.exe'", "GET", "commandline"])
                .output()
                .unwrap();
            String::from_utf8(output.stdout).unwrap()
        } else {
            let cmd_ps = Command::new("ps")
                .args(&["x", "-o", "args"])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            let output = cmd_ps.wait_with_output().unwrap();
            String::from_utf8(output.stdout).unwrap()
        };

        let re : Regex = Regex::new(re_output.as_str()).unwrap();
        if re.is_match(process_output.as_str()) {
            for cap_grp  in re.captures_iter(process_output.as_str()) {
                path = cap_grp[1].to_owned();
            }
        } else {
            return Err("No League process found".to_owned());
        }

        let mut lockfile : File;
        path = format!("{}/lockfile", path);
        let path_p : &Path = Path::new(path.as_str());

        match File::open(path_p) {
            Ok(f) => lockfile = f,
            Err(e) => panic!("lockfile not found, is the League of Legends client open?\n{}", e)
        }

        let mut buffer = String::new();
        lockfile.read_to_string(&mut buffer).unwrap();

        let mut parser_buffer = String::new();
        let mut parser_count :i16 = 0;
        let mut lockfile : LockFile = LockFile::new();
        for c in buffer.chars() {
            if c == ':' {
                match parser_count {
                    0 => lockfile.unknown_0 = parser_buffer.clone(),
                    1 => lockfile.unknown_1 = parser_buffer.clone(),
                    2 => lockfile.port = parser_buffer.parse::<i32>().unwrap(),
                    3 => lockfile.password = parser_buffer.clone(),
                    4 => lockfile.protocol = parser_buffer.clone(),
                    _ => {}
                }
                parser_buffer = String::new();
                parser_count = parser_count + 1;
            } else {
                parser_buffer.push(c);
            }
        }

        Ok(lockfile)
    }
}