use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::NamedFile;
use rocket::get;
use rocket::http::ContentType;
use rocket::http::Header;
use rocket::http::Status;
use rocket::launch;
use rocket::post;
use rocket::response::content::RawHtml;
use rocket::routes;
use rocket::Build;
use rocket::Data;
use rocket::Rocket;
use rocket::{Request, Response};
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};
use std::path::Path;
use std::process::Command;

const WEBPAGE: &str = "./static/index.html";

// CORS Fairing
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(WEBPAGE).await.ok()
}

#[post("/classify", data = "<data>")]
async fn classify(content_type: &ContentType, data: Data<'_>) -> Result<RawHtml<String>, Status> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::file("image"),
    ]);

    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options)
        .await
        .map_err(|e| {
            eprintln!("Error parsing multipart form data: {:?}", e);
            Status::BadRequest
        })?;

    if let Some(file_fields) = multipart_form_data.files.get_mut("image") {
        if !file_fields.is_empty() {
            let file = &file_fields[0];
            let path = Path::new(&file.path);

            // Log the file path
            println!("Attempting to classify image at path: {:?}", path);

            // Check if the file exists
            if !path.exists() {
                eprintln!("File does not exist at path: {:?}", path);
                return Err(Status::InternalServerError);
            }

            // Run the classify command
            let output = Command::new("classify").arg(path).output().map_err(|e| {
                eprintln!("Failed to execute command: {}", e);
                Status::InternalServerError
            })?;

            let mut result = String::new();

            // Capture stdout
            if !output.stdout.is_empty() {
                result.push_str(&String::from_utf8_lossy(&output.stdout));
            } else {
                println!("Command produced no stdout output");
            }

            // Capture stderr
            if !output.stderr.is_empty() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Command produced stderr: {}", stderr);
                result.push_str("\nErrors:\n");
                result.push_str(&stderr);
            }

            // Check if the command was successful
            if !output.status.success() {
                eprintln!("Command failed with exit code: {:?}", output.status.code());
                return Err(Status::InternalServerError);
            }

            // Wrap the result in pre tags to preserve formatting
            return Ok(RawHtml(format!("<pre>{}</pre>", result)));
        }
    }

    eprintln!("No image file found in the request");
    Err(Status::BadRequest)
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, classify])
        .configure(rocket::Config::figment().merge(("address", "0.0.0.0")))
}
