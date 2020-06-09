extern crate dotenv;
extern crate serde;
extern crate serde_json;
extern crate actix_web;
extern crate env_logger;

use actix_web::{web, App, HttpResponse, HttpServer, Result };
use actix_web::middleware::Logger;
use serde::Serialize;
use std::env;
use env_logger::Env;

#[derive(Serialize)]
struct Test {
    x: u8,
    y: u8
}

async fn chinese_rings_response(data: web::Json<u8>) -> Result<HttpResponse> {
    println!("{}", data);
    let to_answer = vec![Test { x: 2, y: 3 }, Test { x: 6, y: 7 }];
    return Ok(HttpResponse::Ok().json(to_answer));
}

async fn labyrinth_response(data: web::Json<Vec<Vec<i8>>>) -> Result<HttpResponse> {
    println!("{:?}", data);
    return Ok(HttpResponse::Ok().body("labyrinth 0"));
}

async fn escape_ways_response(data: web::Json<Vec<Vec<i8>>>) -> Result<HttpResponse> {
    println!("{:?}", data);
    return Ok(HttpResponse::Ok().body("escape_ways"));
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");

    let app_host = env::var("LISTEN_IP").expect("LISTEN_IP not found.");
    let app_port = env::var("LISTEN_PORT").expect("LISTEN_PORT not found.");
    let connection_string = format!("{}:{}", &app_host, &app_port);

    env_logger::from_env(Env::default().default_filter_or("info")).init();

    println!("Server up listen on port {} ...", &app_port);

    return HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            
            .route("/chinese_rings", web::post().to(chinese_rings_response))
            .route("/labyrinth", web::post().to(labyrinth_response))
            .route("/escape_ways", web::post().to(escape_ways_response))
    })
    .bind(connection_string)?
    .run()
    .await;
}