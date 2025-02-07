use futures::stream::TryStreamExt;

use reqwest::{Body, Client};

use tokio::fs::File;

use tokio_util::codec::{BytesCodec, FramedRead};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("example.txt").await?;

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8814/upload")
        .body(file_to_body(file))
        .send()
        .await?;

    Ok(())
}

fn file_to_body(file: File) -> Body {
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);
    body
}
