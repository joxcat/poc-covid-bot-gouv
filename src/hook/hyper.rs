use hyper::{Client,Response,Body,Request};
use hyper::header::{HeaderName,HeaderValue};
use hyper_tls::HttpsConnector;

use std::collections::HashMap;

type Resp<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn get_uri(uri: &str, headers: HashMap<String,String>) -> Resp<Response<Body>> {
    let mut req = Request::get(uri);
    append_headers(req.headers_mut().unwrap(), headers)?;
    let req = req.body(Body::empty())?;

    let resp = if uri.starts_with("http://") {
        let client = Client::new();
        client.request(req).await?

    } else if uri.starts_with("https://") {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_,hyper::Body>(https);
        client.request(req).await?

    } else {
        eprintln!("Unsupported protocol");
        std::process::exit(1);
    };
    Ok(resp)
}

pub async fn post_uri(uri: &str, headers: HashMap<String,String>, body: &str) -> Resp<Response<Body>> {
    let mut req = Request::post(uri);

    append_headers(req.headers_mut().unwrap(), headers)?;
    let req = req.body(
        Body::from(hyper::body::Bytes::copy_from_slice(body.as_bytes()))
    )?;

    let resp = if uri.starts_with("http://") {
        let client = Client::new();
        client.request(req).await?

    } else if uri.starts_with("https://") {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_,hyper::Body>(https);
        client.request(req).await?

    } else {
        eprintln!("Unsupported protocol");
        std::process::exit(1);
    };

    Ok(resp)
}

fn append_headers(builder: &mut hyper::HeaderMap, headers: HashMap<String,String>) -> Resp<()> {
    for (k,v) in headers {
        builder.insert(HeaderName::from_lowercase(k.to_lowercase().as_bytes())?, HeaderValue::from_str(v.as_str())?);
    }

    Ok(())
}