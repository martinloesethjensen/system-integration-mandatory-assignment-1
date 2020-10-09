use rand::{thread_rng, Rng};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rusqlite::{Connection, Result, named_params, Error};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct NemIdAuth {
    nem_id_code: String,
    nem_id: String,
}

fn generate_code() -> String {
    let mut random_int_string = "".to_owned();
    for _ in 0..6 {
        let temp: &str = &thread_rng().gen_range(0, 10).to_string();
        random_int_string.push_str(temp);
    }
    return random_int_string;
}

fn check_user_from_db(nem_id_auth: web::Json<NemIdAuth>) -> Result<(), Error> {
    let conn = Connection::open("../NemID_ESB/nem_id_database.sqlite")?;

    let mut stmt = conn.prepare(
        "SELECT * FROM user WHERE NemId == :nem_id AND Password == :password;",
    )?;

    let mut rows = stmt.query_named(
        named_params!{ 
            ":nem_id": &String::from(nem_id_auth.nem_id.to_string()), 
            ":password": &String::from(nem_id_auth.nem_id_code.to_string()),
        })?;
    
    match rows.next()? {
        Some(_) => Ok(()),
        None => Err(Error::QueryReturnedNoRows),
    }
}

async fn authenticate_user(nem_id_auth: web::Json<NemIdAuth>) -> impl Responder {
    match check_user_from_db(nem_id_auth) {
        Ok(_) => HttpResponse::Ok().json(json!({"generatedCode": format!("{}", generate_code())})),
        Err(err) => HttpResponse::Forbidden().json(json!({"error": format!("{}", err)})),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/nemid-auth", web::post().to(authenticate_user))
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}
