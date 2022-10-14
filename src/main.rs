struct FilePart {
    name: String,
    start: i64,
    end: i64,
}

fn downloader(url: &str) -> Result<(), &str> {
    if url.trim() == "" {
        return Err("invalid url");
    }
}

fn main() {
    println!("Hello, world!");
}
