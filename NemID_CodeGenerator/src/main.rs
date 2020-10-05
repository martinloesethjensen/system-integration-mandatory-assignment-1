extern crate rand;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use serde::Deserialize;
use rand::thread_rng;
use rand::Rng;

#[derive(Deserialize)]
struct NemIdUser {
    cpr: String,
    email: String,
}

async fn generate_nem_id(item: web::Json<NemIdUser>) -> impl Responder {
    let mut range = thread_rng();
    let mut random_int_string: String = "".to_owned();
    for _ in 0..5 {
        let temp: &str = &String::from(range.gen_range(0, 10).to_string());
        random_int_string.push_str(temp);
    }
    let last_four_cpr: String = item.cpr.chars().skip(6).take(4).collect();
    HttpResponse::Ok().json(json!({"nemId": format!("{}-{}", random_int_string, last_four_cpr)}))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/generate-nemId", web::post().to(generate_nem_id))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}