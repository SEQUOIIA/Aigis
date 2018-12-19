use ws::{Handler, Sender, Handshake, Result, Message, Request};
use url;
use base64;

pub struct Client {
    pub out: Sender,
    pub auth_pass : String
}

impl Handler for Client {
    fn on_open(&mut self, _shake: Handshake) -> Result<()> {
        debug!("Socket opened");
        self.out.send("[5, \"OnJsonApiEvent\"]")
    }


    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Got message: {}", msg);
        Ok(())
    }

    fn build_request(&mut self, url: &url::Url) -> Result<Request> {
        let mut req : Request = Request::from_url(url).unwrap();
        let auth = format!("riot:{}", self.auth_pass);
        let auth_encoded = base64::encode(&auth);
        println!("{}", auth_encoded);
        req.headers_mut().push(("Authorization".into(), format!("Basic {}", auth_encoded).into()));
        req.headers_mut().push(("Sec-WebSocket-Protocol".into(), format!("wamp").into()));
        println!("Request built");
        Ok(req)
    }
}