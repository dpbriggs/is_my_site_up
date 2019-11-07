use reqwest::{Client, Response};

pub fn init_client() -> Client {
    Client::new()
}

pub fn head_req(client: &Client, url: &str) -> Result<Response, reqwest::Error> {
    client.head(url).send()
}
