use reqwest::{multipart, Client};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Открываем файл
    let file = File::open("example.txt").await?;

    // Создаем поток данных из файла
    let stream = FramedRead::new(file, BytesCodec::new());
    let file_body = reqwest::Body::wrap_stream(stream);

    // Создаем multipart форму
    let form = multipart::Form::new()
        .part(
            "file",
            multipart::Part::stream(file_body)
                .file_name("example.txt")
                .mime_str("text/plain")?,
        )
        .text("description", "Описание файла")
        .text("user", "username");

    // Отправляем запрос
    let client = Client::new();
    let res = client
        .post("http://localhost:8814/upload")
        .multipart(form)
        .send()
        .await?;

    println!("Ответ: {}", res.status());

    Ok(())
}
