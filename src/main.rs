use reqwest::header::HeaderValue;
use std::{fs, io};
use std::fs::File;
use std::io::{Read, Write};
use std::sync::Arc;
use std::time::Instant;
use rpb::bar::Bar;

pub trait HeaderValueExtension {
    fn to_string(&self) -> String;
}

impl HeaderValueExtension for HeaderValue {
    fn to_string(&self) -> String {
        self.to_str().unwrap_or_default().to_string()
    }
}

async fn downloader<'l>(url: &'static str) -> Result<(), &str> {
    if url == "" {
        return Err("invalid url");
    }
    let client = reqwest::Client::new();

    let url_split = url.split("/").collect::<Vec<_>>();
    let filename = url_split[url_split.len() - 1];

    let mut file = File::create(filename).unwrap();
    let body = client
        .get(url)
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    let bar = Bar::default_bytes(body.len() as i64, "downloading");
    io::copy(&mut bar.reader(body.as_ref()), &mut file).unwrap();




    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    downloader("https://desktop.docker.com/win/main/amd64/Docker Desktop Installer.exe").await;
    println!("");
    Ok(())
}

// fn main() {
//     let mut threads = vec![];
//     for i in 0..10 {
//         let th = spawn(move || {
//             println!("{i}");
//             sleep(Duration::from_secs(1));
//         });
//         threads.push(th);
//     }
//
//     for thread in threads {
//         thread.join().unwrap();
//     }
//     println!("main")
// }