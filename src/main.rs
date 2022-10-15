use std::borrow::Borrow;
use std::fs::File;
use std::io::Write;
use std::thread;
use reqwest::{Error, Response};
use reqwest::header::HeaderValue;

struct FilePart {
    name: String,
    start: i64,
    end: i64,
}

pub trait HeaderValueExtension {
    fn to_string(&self) -> String;
}

impl HeaderValueExtension for HeaderValue {
    fn to_string(&self) -> String {
        self.to_str().unwrap_or_default().to_string()
    }
}

async fn downloader(url: &str) -> Result<(), &str> {
    if url == "" {
        return Err("invalid url");
    }
    let client = reqwest::Client::new();
    let res = client
        .head(url)
        .send()
        .await;

    let mut accept_range: bool = false;
    let mut content_length: i64 = 0;
    match res {
        Ok(f) => {
            content_length = f.headers().get("content-length").unwrap().to_string().parse::<i64>().unwrap();
            accept_range = f.headers().get("accept-ranges").unwrap().to_string() == "bytes";
            println!("{:?} - {:?}", content_length, accept_range)
        },
        Err(e) => println!("{}", e)
    };

    if !accept_range {
        return Err("unable to download file with multithreading");
    }

    let nb_part = 3;
    let offset = content_length/nb_part;

    for i in 0..nb_part {
        let name = format!("part{}", i);
        let start = i * offset;
        let end  = (i + 1) * offset;
        let mut file = File::create(name).unwrap();

        || async {
            let body = client
                .get(url)
                .header("Range", format!("bytes={}-{}", start, end))
                .send()
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap();

            file.write_all(body.borrow());
        };
    }

    Ok(())
}

fn worker() {

}

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    downloader("https://agritrop.cirad.fr/584726/1/Rapport.pdf").await
}
