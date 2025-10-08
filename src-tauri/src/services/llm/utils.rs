// MIT License Copyright (c) 2024-present Frank Zhang
use entity::entities::settings::ProxySetting;
use reqwest::{
    Client,
    Proxy,
};

pub fn build_http_client(proxy_setting: Option<ProxySetting>) -> Client {
    let mut client_builder = reqwest::Client::builder();
    if let Some(proxy) = proxy_setting {
        if proxy.enabled {
            let proxy_url = format!("{}://{}:{}", proxy.protocol, proxy.host, proxy.port);
            if let Ok(proxy) = Proxy::all(&proxy_url) {
                client_builder = client_builder.proxy(proxy);
            }
        }
    }
    client_builder.build().unwrap()
}

pub fn sum_option(a: Option<u32>, b: Option<u32>) -> Option<u32> {
    a.zip(b).map(|(x, y)| x + y)
}