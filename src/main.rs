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
    let content_length: i64;
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

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    downloader("https://agritrop.cirad.fr/584726/1/Rapport.pdf").await
}
