use actix_cors::Cors;
use actix_web::{get, http, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
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
    HttpResponse::Ok().json(MyObj {
        name: "John Doe 2".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        //  let cors = Cors::default()
        // .allowed_origin("http://localhost:3000")
        // .allowed_methods(vec!["GET"])
        // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        // .allowed_header(http::header::CONTENT_TYPE)
        // .max_age(3600);

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
