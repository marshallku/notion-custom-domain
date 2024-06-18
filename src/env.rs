use core::panic;
use std::{borrow::Cow, env};

#[derive(Clone, Debug)]
pub struct Env {
    pub address: Cow<'static, str>,
    pub port: u16,
    pub host: Cow<'static, str>,
    pub notion_page_id: Cow<'static, str>,
    pub external_address: Cow<'static, str>,
}

impl Env {
    pub fn new() -> Self {
        let address = match env::var("BIND_ADDRESS") {
            Ok(address) => Cow::Owned(address),
            Err(_) => Cow::Owned("127.0.0.1".to_string()),
        };
        let port = match env::var("PORT") {
            Ok(port) => port.parse().unwrap_or(48099),
            Err(_) => 48099,
        };
        let host = match env::var("HOST") {
            Ok(host) => Cow::Owned(host),
            Err(_) => Cow::Owned("http://localhost/".to_string()),
        };
        let notion_page_id = match env::var("NOTION_PAGE_ID") {
            Ok(notion_page_id) => Cow::Owned(notion_page_id),
            Err(_) => panic!("NOTION_PAGE_ID is required"),
        };
        let external_address = match env::var("EXTERNAL_ADDRESS") {
            Ok(external_address) => Cow::Owned(external_address),
            Err(_) => panic!("EXTERNAL_ADDRESS is required"),
        };

        Self {
            address,
            port,
            host,
            notion_page_id,
            external_address,
        }
    }
}
