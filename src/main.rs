use actix_cors::Cors;
use actix_web::{
    get, http, middleware, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
mod db;
mod utils;
use actix_web::http::header;
use db::Like;
use utils::parse_ip;
#[derive(Serialize, Deserialize, Debug)]
struct MyObj {
    likes: std::vec::Vec<db::Like>,
}

#[derive(Deserialize)]
struct Params {
    id: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("42")
}

#[get("/collect")]
pub async fn collect(req: HttpRequest) -> impl Responder {
    match parse_ip(req.connection_info().realip_remote_addr()) {
        Some(address) => match db::MysqlClient::collect(address.ip()).await {
            Ok(_) => HttpResponse::Ok().body("IP addres was inserted succesfuly"),
            Err(err) => HttpResponse::Ok().body(format!("Unable to insert ip address. {}", err)),
        },
        _ => HttpResponse::Ok().body("Unable to iterate through socket address"),
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

fn increment(like: &Like) -> i32 {
    let Like { count, .. } = like;

    count + 1
}

fn to_int(id: &str) -> i32 {
    id.parse::<i32>().unwrap_or(0)
}

async fn handle_like(web::Query(params): web::Query<Params>) -> impl Responder {
    let article_id = to_int(&params.id);
    let result = db::MysqlClient::select(params.id).await;

    match result {
        Ok(likes) => {
            if likes.is_empty() {
                db::MysqlClient::insert(article_id).await;
            } else {
                let count = increment(likes.first().unwrap());
                db::MysqlClient::update(article_id, count).await;
            }
            HttpResponse::Ok().json(MyObj { likes })
        }
        _ => HttpResponse::Ok().json(MyObj { likes: vec![] }),
    }
}

async fn get_like(web::Query(params): web::Query<Params>) -> impl Responder {
    let result = db::MysqlClient::select(params.id).await;

    match result {
        Ok(likes) => HttpResponse::Ok().json(MyObj { likes }),
        _ => HttpResponse::Ok().json(MyObj { likes: vec![] }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default().allowed_origin("*");

        App::new()
            .wrap(cors)
            .wrap(
                middleware::DefaultHeaders::new()
                    .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
            )
            .wrap(
                middleware::DefaultHeaders::new()
                    .header(http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true"),
            )
            //  .wrap(cors)
            .service(hello)
            .service(echo)
            .service(collect)
            .route("/like", web::get().to(handle_like))
            .route("/get-like", web::get().to(get_like))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
