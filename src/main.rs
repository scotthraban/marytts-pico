use actix_web::{web, get, App, HttpServer, HttpResponse, middleware::Logger};
use std::process::Command;
use std::fs::{File, remove_file};
use std::io::Read;
use std::collections::HashMap;
use uuid::Uuid;
use env_logger::Env;
use serde_urlencoded;

#[get("/process")]
async fn process_get(info: web::Query<HashMap<String, String>>) -> HttpResponse {
    let input_text = match info.get("INPUT_TEXT") {
        Some(text) => text,
        None => return HttpResponse::BadRequest().body("INPUT_TEXT query parameter is missing"),
    };
    
    let locale = info.get("LOCALE").map_or("en_US".to_string(), |loc| loc.replace('_', "-"));

    process_generate_wav(input_text, &locale).await
}

//#[post("/process")]
async fn process_post(body: web::Bytes) -> HttpResponse {

    // Should be able to let the library do this for me, but because the darn library does not send a content-type
    //  it doesn't work. That would also let me uncomment the handler annotation and use .service to bind
    let form_data: HashMap<String, String> = serde_urlencoded::from_bytes(&body).unwrap_or_default();
    
    let input_text = match form_data.get("INPUT_TEXT") {
        Some(text) => text,
        None => return HttpResponse::BadRequest().body("INPUT_TEXT query parameter is missing"),
    };
    
    let locale = form_data.get("LOCALE").map_or("en_US".to_string(), |loc| loc.replace('_', "-"));

    process_generate_wav(input_text, &locale).await
}

async fn process_generate_wav(input_text: &str, locale: &str) -> HttpResponse {
    let output_file = format!("/tmp/output_{}.wav", Uuid::new_v4()); // Generate a unique filename

    let exe_path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("/usr/bin/pico2wave");

    // Call the executable with the input_text
    let status = Command::new(exe_path)
        .arg("-l")
        .arg(locale)
        .arg("-w")
        .arg(&output_file)
        .arg(input_text)
        .status()
        .expect("Failed to execute process");

    if status.success() {
        // Read the resulting file
        let mut file = File::open(&output_file).expect("Failed to open output file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read output file");

        // Delete the output file
        remove_file(&output_file).expect("Failed to delete output file");

        // Return the file contents
        HttpResponse::Ok()
            .content_type("audio/x-wav")
            .body(contents)
    } else {
        HttpResponse::InternalServerError().body("Process execution failed")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(process_get)
            .route("/process", web::post().to(process_post))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}