struct FilePart {
    name: String,
    start: i64,
    end: i64,
}

async fn downloader(url: &str){
    // assert_eq!(url.trim(), "", "invalid url");
    let client = reqwest::Client::new();
    let res = client
        .head(url)
        .send()
        .await;

    match res {
        Ok(f) => println!("{:?}", f.headers().get("accept-ranges")),
        Err(e) => println!("{}", e)
    }

}

#[tokio::main]
async fn main() {
    downloader("https://agritrop.cirad.fr/584726/1/Rapport.pdf").await
}
