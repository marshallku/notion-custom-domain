use core::panic;
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct Env {
    pub address: Cow<'static, str>,
    pub port: u16,
    pub host: Cow<'static, str>,
    pub public_domain: Cow<'static, str>,
    pub notion_page_url: Cow<'static, str>,
}

impl Env {
    pub fn new() -> Self {
        let address = match std::env::var("BIND_ADDRESS") {
            Ok(address) => Cow::Owned(address),
            Err(_) => Cow::Owned("127.0.0.1".to_string()),
        };
        let port = match std::env::var("PORT") {
            Ok(port) => port.parse().unwrap_or(41890),
            Err(_) => 41890,
        };
        let host = match std::env::var("HOST") {
            Ok(host) => Cow::Owned(host),
            Err(_) => Cow::Owned("http://localhost/".to_string()),
        };
        let public_domain = match std::env::var("PUBLIC_DOMAIN") {
            Ok(public_domain) => Cow::Owned(public_domain),
            Err(_) => panic!("PUBLIC_DOMAIN is required"),
        };
        let notion_page_url = match std::env::var("NOTION_PAGE_URL") {
            Ok(notion_page_url) => Cow::Owned(notion_page_url),
            Err(_) => panic!("NOTION_PAGE_URL is required"),
        };

        Self {
            address,
            port,
            host,
            public_domain,
            notion_page_url,
        }
    }
}
