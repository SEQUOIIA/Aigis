extern crate reqwest;
extern crate ws;
extern crate tokio;
extern crate num_cpus;
extern crate url;
extern crate base64;
extern crate native_tls;
extern crate regex;
pub extern crate tungstenite;

#[macro_use]
extern crate log;

use self::futures::future::{ok, Future};

pub mod LCU;

pub mod futures {
    extern crate futures;

    pub use self::futures::*;
}