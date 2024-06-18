use core::panic;
use std::{borrow::Cow, collections::HashMap, env};

#[derive(Clone, Debug)]
pub struct Env {
    pub address: Cow<'static, str>,
    pub port: u16,
    pub host: Cow<'static, str>,
    pub external_address: Cow<'static, str>,
    pub route_paths: Cow<'static, Vec<String>>,
    pub notion_pages: Cow<'static, Vec<String>>,
    pub path_to_notion_map: Cow<'static, HashMap<String, String>>,
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
        let external_address = match env::var("EXTERNAL_ADDRESS") {
            Ok(external_address) => Cow::Owned(external_address),
            Err(_) => panic!("EXTERNAL_ADDRESS is required"),
        };
        let route_paths = match env::var("ROUTE_PATHS") {
            Ok(route_paths) => {
                Cow::Owned::<Vec<_>>(route_paths.split(',').map(String::from).collect())
            }
            Err(_) => Cow::Owned(vec!["/".to_string()]),
        };
        let notion_pages = match env::var("NOTION_PAGES") {
            Ok(notion_pages) => {
                Cow::Owned::<Vec<_>>(notion_pages.split(',').map(String::from).collect())
            }
            Err(_) => panic!("NOTION_PAGES is required"),
        };
        let path_to_notion_map = Cow::Owned(
            route_paths
                .iter()
                .cloned()
                .zip(notion_pages.iter().cloned())
                .collect::<HashMap<_, _>>(),
        );

        Self {
            address,
            port,
            host,
            external_address,
            route_paths,
            notion_pages,
            path_to_notion_map,
        }
    }
}
