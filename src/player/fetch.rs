use std::io::prelude::*;
use futures::channel::oneshot::Sender;
use reqwest::header::{HOST, CACHE_CONTROL, PRAGMA, HeaderMap, UPGRADE_INSECURE_REQUESTS, ACCEPT, ACCEPT_ENCODING, USER_AGENT};
use reqwest::Method;
use tempfile::NamedTempFile;

#[tokio::main]
pub async fn fetch_data(url: &str, buffer: NamedTempFile, tx: Sender<String>) -> Result<(), failure::Error> {

    // debug!("start fetch_data");
    let mut buffer = buffer;

    let mut headers = HeaderMap::new();
    headers.insert(CACHE_CONTROL, "no-cache".parse().unwrap());
    headers.insert(PRAGMA, "no-cache".parse().unwrap());
    headers.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());
    headers.insert(HOST, "m7.music.126.net".parse().unwrap());
    headers.insert(ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "gzip,deflate".parse().unwrap());
    headers.insert(
        USER_AGENT,
        "User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:65.0) Gecko/20100101 Firefox/65.0".parse().unwrap(),
    );
    let client = reqwest::Client::builder()
        // no need proxy but can add it in config
        // .proxy(reqwest::Proxy::all("socks5://127.0.0.1:3333").expect("proxy error"))
        .build().expect("builder error");
    let builder = client.request(Method::GET, url).headers(headers);
    let mut res = builder.send().await?;

    debug!("start download");
    if let Some(chunk) = res.chunk().await? {
        debug!("first chunk");
        buffer.write(&chunk[..]).unwrap();
        send_msg(tx);
    }

    while let Some(chunk) = res.chunk().await? {
        // bytes
        buffer.write(&chunk[..]).unwrap();
    }
    debug!("finish downloa");
    Ok(())
}

fn send_msg(tx: Sender<String>) {
    tx.send("ok".to_owned()).expect("send error");
}
