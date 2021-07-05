/*fn main() {
    println!("Hello, world!");
}
*/
use actix_web::{
    web, App, Error, HttpResponse, HttpServer,
};

use json::JsonValue;
use bytes::{Bytes};

async fn echo(body: Bytes) -> Result<HttpResponse, Error> {
    let result = json::parse(std::str::from_utf8(&body).unwrap());
    // return result
    let injson: JsonValue = match result {
    Ok(v) => v,
    Err(e) => json::object! {"err" => e.to_string() },
    };
    Ok(HttpResponse::Ok()
    .content_type("application/json")
    .body(injson.dump()))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
        HttpServer::new(|| {
        App::new()
        .data(web::JsonConfig::default().limit(4096))
        .service(web::resource("/echo").route(web::post().to(echo)))
        })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
