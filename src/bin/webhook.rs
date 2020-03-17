#![allow(deprecated)]
use std::thread;

use gouv_rs::{util, hook, async_spawn};
use hyper::{Response,Body};

type Resp<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Resp<()> {
    let (tx,rx) = std::sync::mpsc::channel();
    let x = async_spawn(
        hook("http://127.0.0.1:3000/", None, Some(rx), process_body)
    );
    thread::sleep_ms(10000);
    tx.send(())?;
    x.await??;
    Ok(())
}

async fn process_body(body: Response<Body>) -> Resp<()> {
    let body = hyper::body::to_bytes(body.into_body()).await?;
    let body = String::from_utf8(body.to_vec()).unwrap();

    let hashbody = util::sha256(&body);
    println!("{}", hashbody);
    let oldhash = util::read_file("./log.txt");
    println!("{}", oldhash);

    if oldhash != hashbody {
        println!("Content updated!");
        util::write_file("./log.txt", &hashbody);
    } else {
        println!("Content unchanged!");
    }

    thread::sleep_ms(1000);
    Ok(())
} 