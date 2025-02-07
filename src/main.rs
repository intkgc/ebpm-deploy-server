use rocket::fs::TempFile;
use rocket::launch;
use rocket::{post, routes};

#[post("/upload", data = "<file>")]
async fn upload(mut file: TempFile<'_>) -> std::io::Result<()> {
    file.persist_to("uploads/file.txt").await?;
    Ok(())
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
