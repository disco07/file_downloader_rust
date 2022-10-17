use reqwest::header::HeaderValue;
use std::borrow::Borrow;
use std::fs;
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
    let res = client.head(url).send().await;

    let mut accept_range: bool = false;
    let mut content_length: i64 = 0;
    match res {
        Ok(f) => {
            content_length = f
                .headers()
                .get("content-length")
                .unwrap()
                .to_string()
                .parse::<i64>()
                .unwrap();
            accept_range = f.headers().get("accept-ranges").unwrap().to_string() == "bytes";
        }
        Err(e) => println!("{}", e),
    };

    if !accept_range {
        return Err("unable to download file with multithreading");
    }

    let url_split = url.split("/").collect::<Vec<_>>();
    let filename = url_split[url_split.len() - 1];
    let nb_part = 3;
    let offset = content_length / nb_part;
    let bar = Arc::new(Bar::default_bytes(content_length, "downloading"));

    let mut threads = vec![];
    for i in 0..nb_part {
        let mut bar = Arc::clone(&bar);
        let th = tokio::spawn(async move {
            let name = format!("part{}", i);
            let start = i * offset;
            let end = (i + 1) * offset;
            let mut file = File::create(name).unwrap();
            let body = reqwest::Client::new()
                .get(url)
                .header("Range", format!("bytes={}-{}", start, end))
                .send()
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap();
            // println!("{i}");
            file.write_all(body.borrow()).unwrap();
            //bar.writer(file).write_all(body.borrow()).unwrap();
            bar.inc();
        });
        threads.push(th);
    }

    for thread in threads {
        thread.await.unwrap();
    }

    let mut out = File::create(filename).unwrap();
    for i in 0..nb_part {
        let name = format!("part{}", i);

        let file = File::open(&name).unwrap();
        let mut file_copy = file.try_clone().unwrap();
        let mut contents = vec![];
        file_copy.read_to_end(&mut contents).unwrap();

        out.write_all(contents.as_ref()).unwrap();

        fs::remove_file(name.as_str()).unwrap();
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    let time = Instant::now();
    downloader("https://agritrop.cirad.fr/584726/1/Rapport.pdf").await;
    println!("");
    println!("{:?}", time.elapsed().as_secs());
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