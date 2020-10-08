extern crate rand;
use rand::thread_rng;
use rand::Rng;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rusqlite::{Connection, Result, named_params, Error};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct NemIdAuth {
    // We need to have it in camelCase as that is how the json response has it
    nemIdCode: String,
    nemId: String,
}

#[derive(Debug)]
struct User {
    id: i32,
    cpr: String,
    nem_id: String,
    password: String,
}

fn generate_code() -> String {
    let mut range = thread_rng();
    let mut random_int_string: String = "".to_owned();
    for _ in 0..5 {
        let temp: &str = &String::from(range.gen_range(0, 10).to_string());
        random_int_string.push_str(temp);
    }
    return random_int_string;
}

fn check_user_from_db(nem_id_auth: web::Json<NemIdAuth>) -> Result<User, Error> {
    let conn = Connection::open("../NemID_ESB/nem_id_database.sqlite")?;

    let mut stmt = conn.prepare(
        "SELECT * FROM user WHERE NemId == :nem_id AND Password == :password;",
    )?;

    let mut rows = stmt.query_named(
        named_params!{ 
            ":nem_id": &String::from(nem_id_auth.nemId.to_string()), 
            ":password": &String::from(nem_id_auth.nemIdCode.to_string()),
        })?;
    
    // Empty User as I don't know rust that well 😅
    let mut user: User = User {id: -1, cpr: String::from("null"), nem_id: String::from("null"), password: String::from("null")};

    while let Some(row) = rows.next()? {
        user = User {
            id: row.get(0)?,
            cpr: row.get(1)?,
            nem_id: row.get(2)?,
            password: row.get(3)?,
        };
    }

    // If the user is empty then no user were found
    // Return err in this case else return ok with user that is found
    if user.id == -1 {
        Err(Error::QueryReturnedNoRows)
    } else {
        Ok(user)
    }

}

async fn authenticate_user(nem_id_auth: web::Json<NemIdAuth>) -> impl Responder {
    let user = check_user_from_db(nem_id_auth);

    match user {
        Err(err) => HttpResponse::Forbidden().json(json!({"error": format!("{}", err)})),
        Ok(_) => HttpResponse::Ok().json(json!({"generatedCode": format!("{}", generate_code())})),
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
