#[macro_use]
extern crate rocket;

use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

#[post("/upload", data = "<data>")]
async fn upload(content_type: &ContentType, data: Data<'_>) -> std::io::Result<String> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::file("file"),
        MultipartFormDataField::text("token"),
    ]);

    let mut form = MultipartFormData::parse(content_type, data, options)
        .await
        .unwrap();

    if let Some(file_fields) = form.files.get("file") {
        let file_field = &file_fields[0];
        let file_path = format!(
            "uploads/{}",
            file_field
                .file_name
                .as_ref()
                .unwrap_or(&"uploaded_file".to_string())
        );
        std::fs::rename(&file_field.path, &file_path)?;
    }

    let token = form
        .texts
        .remove("token")
        .map(|v| v[0].text.clone())
        .unwrap_or_default();

    Ok(format!("Файл загружен пользователем {}", token))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![upload])
        .configure(rocket::Config {
            address: "0.0.0.0".parse().unwrap(),
            port: 8814,
            ..Default::default()
        })
}
