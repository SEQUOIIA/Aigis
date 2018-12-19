use std;

#[derive(Debug)]
pub enum LCUError {
    LCUApiCallRequest(String)
}

impl std::fmt::Display for LCUError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            LCUError::LCUApiCallRequest(ref _err) => f.write_str(format!("LCUError::LCUApiCallRequest - {}", _err).as_str())
        }
    }
}

impl std::error::Error for LCUError {
    fn description(&self) -> &str {
        match *self {
            LCUError::LCUApiCallRequest(ref _err) => "An error occured within the LCUClient::api_call() call"
        }
    }
}