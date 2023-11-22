use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

async fn read_urls(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path).await?;
    let reader = BufReader::new(file);
    let mut urls = Vec::new();

    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        urls.push(line.trim().to_string());
    }

    Ok(urls)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls = read_urls("./urls.csv").await?;
    let dest = "./files/";
    for url in urls {
        let path = format!("{}{}", dest, url.split('/').last().unwrap());
        download_file(&url, path.as_str()).await?;
    }
    Ok(())
}

async fn download_file(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    let mut file = File::create(path).await?;
    file.write_all(&bytes).await?;

    Ok(())
}
