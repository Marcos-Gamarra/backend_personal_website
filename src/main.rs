use actix_web::{
    get,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use bb8_postgres;
use tokio_postgres::NoTls;
mod db;

#[get("/test")]
async fn test(
    pool: Data<bb8::Pool<bb8_postgres::PostgresConnectionManager<NoTls>>>,
) -> impl Responder {
    let word = db::test(pool).await;
    HttpResponse::Ok().body(word)
}

#[get("/wordle/check_word/{word}")]
async fn check_word(
    path: web::Path<String>,
    pool: Data<bb8::Pool<bb8_postgres::PostgresConnectionManager<NoTls>>>,
) -> impl Responder {
    let word = path.into_inner();
    let is_valid_word = db::check_word(word, pool).await;

    if is_valid_word {
        return HttpResponse::Ok()
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .body(format!("{}", "true"));
    }

    HttpResponse::Ok().body(format!("{}", "false"))
}

#[get("/wordle/get_word")]
async fn get_word(
    pool: Data<bb8::Pool<bb8_postgres::PostgresConnectionManager<NoTls>>>,
) -> impl Responder {
    let word = db::get_word(pool).await.unwrap();
    //response with word as body and add cors header
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .body(format!("{}", word))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::init_pool().await;
    let pool = Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&pool))
            .service(get_word)
            .service(check_word)
            .service(test)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
