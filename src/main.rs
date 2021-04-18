use actix_web::{
    get, http, middleware, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::Result;

mod db;
mod utils;
use db::{Data, Like};
use utils::parse_ip;

#[derive(Serialize, Deserialize, Debug)]
struct MyObj {
    likes: std::vec::Vec<db::Like>,
}

#[derive(Deserialize)]
struct Params {
    id: String,
}

#[post("/collect")]
pub async fn collect(req_body: String) -> impl Responder {
    let data: Result<Data> = serde_json::from_str(&req_body);
    match data {
        Ok(val) => match db::MysqlClient::collect(val).await {
            Ok(_) => println!("inserted"),
            Err(err) => println!("Error {}", err),
        },
        Err(err) => println!("Error {}", err),
    };
    HttpResponse::Ok().body("Unable to iterate through socket address")
}

#[get("/session")]
pub async fn session(req: HttpRequest) -> impl Responder {
    match parse_ip(req.connection_info().realip_remote_addr()) {
        Some(address) => match db::MysqlClient::session(address.ip()).await {
            Ok(last_inserted_id) => HttpResponse::Ok().body(last_inserted_id.to_string()),
            Err(err) => HttpResponse::Ok().body(format!("Unable to insert ip address. {}", err)),
        },
        _ => HttpResponse::Ok().body("Unable to iterate through socket address"),
    }
}

#[get("/metrics/{name}")]
pub async fn metrics(web::Path(select_by): web::Path<String>) -> impl Responder {
    match db::MysqlClient::metrics(select_by).await {
        Ok(val) => HttpResponse::Ok().body(val),
        _ => HttpResponse::Ok().body(""),
    }
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
        App::new()
            .wrap(
                middleware::DefaultHeaders::new()
                    .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
            )
            .wrap(
                middleware::DefaultHeaders::new()
                    .header(http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true"),
            )
            .service(session)
            .service(collect)
            .service(metrics)
            .route("/like", web::get().to(handle_like))
            .route("/get-like", web::get().to(get_like))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
