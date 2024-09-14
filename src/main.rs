use base64::decode;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::get;
use rocket::http::Header;
use rocket::http::Status;
use rocket::launch;
use rocket::post;
use rocket::response::content::RawHtml;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::Build;
use rocket::Rocket;
use rocket::{Request, Response};
use serde::Deserialize;
use std::env;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

const INDEXPAGE: &str = include_str!("../static/index.html");

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
async fn index() -> RawHtml<String> {
    RawHtml(INDEXPAGE.to_string())
}

#[derive(Deserialize)]
struct ImageData {
    image: String,
}

#[post("/classify", format = "application/json", data = "<image_data>")]
async fn classify(image_data: Json<ImageData>) -> Result<RawHtml<String>, Status> {
    let base64_data = &image_data.image;

    // Decode the base64 data
    let image_bytes = decode(base64_data).map_err(|e| {
        eprintln!("Failed to decode base64 image data: {:?}", e);
        Status::BadRequest
    })?;

    // Save the image bytes to a temporary file
    let mut temp_file = NamedTempFile::new().map_err(|e| {
        eprintln!("Failed to create temporary file: {:?}", e);
        Status::InternalServerError
    })?;

    temp_file.write_all(&image_bytes).map_err(|e| {
        eprintln!("Failed to write image data to temporary file: {:?}", e);
        Status::InternalServerError
    })?;

    let path = temp_file.path();

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

    // Add a "Draw Again" button to return to the canvas
    result.push_str("<br><button onclick=\"window.location.href='/'\">Draw Again</button>");

    // Return the result to the user
    Ok(RawHtml(format!("<pre>{}</pre>", result)))
}

#[launch]
fn rocket() -> Rocket<Build> {
    if let Ok(dir) = env::current_dir() {
        println!("Current working directory: {:?}", dir);
    }

    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, classify])
        .configure(rocket::Config::figment().merge(("address", "0.0.0.0")))
}
