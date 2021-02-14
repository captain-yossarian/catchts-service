use actix_cors::Cors;
use actix_web::{get, http, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
mod db;

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: std::vec::Vec<db::Payment>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    let client = db::Client::new().await;
    match client {
        Ok(val) => HttpResponse::Ok().json(MyObj { name: val }),
        _ => HttpResponse::Ok().json(MyObj {
            name: vec![db::Payment {
                customer_id: 1,
                amount: 2,
                account_name: None,
            }],
        }),
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
            //  .wrap(cors)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
