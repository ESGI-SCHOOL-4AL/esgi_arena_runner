extern crate dotenv;
extern crate serde;
extern crate serde_json;
extern crate actix_web;
extern crate env_logger;
extern crate esgi_arena_resolver_algorithms;

use actix_web::{web, App, HttpResponse, HttpServer, Result };
use actix_web::middleware::Logger;
use serde::{ Serialize, Deserialize };
use std::env;
use env_logger::Env;
use esgi_arena_resolver_algorithms::chinese_rings::chinese_rings_resolver;
use esgi_arena_resolver_algorithms::graph::{ Point, fs_aps_from_matrix, get_field_by_index };
use esgi_arena_resolver_algorithms::a_star::{ a_star_resolver, get_start_to_end_points };
use esgi_arena_resolver_algorithms::dfs::dfs_fs_aps_recursive;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PointJson {
    x: usize,
    y: usize
}

impl PointJson {
    fn from_point(point: Point) -> Self {
        return Self {
            x: point.x.unwrap(),
            y: point.y.unwrap()
        }
    }
}

async fn chinese_rings_response(data: web::Json<usize>) -> Result<HttpResponse> {
    let chinese_rings_answer = chinese_rings_resolver(*data); 
    return Ok(HttpResponse::Ok().json(chinese_rings_answer));
}

async fn labyrinth_response(data: web::Json<Vec<Vec<i8>>>) -> Result<HttpResponse> {
    let cloned_data = data.clone();
    let (start_point, end_point) = get_start_to_end_points(cloned_data.clone()).unwrap();
    let (fs, aps) = fs_aps_from_matrix(cloned_data.clone()).unwrap();
    let start_field = get_field_by_index(cloned_data.clone(), start_point).unwrap();
    let end_field = get_field_by_index(cloned_data.clone(), end_point).unwrap();
    let labyrinth_answer: Vec<PointJson> = a_star_resolver(fs, aps, cloned_data.len(), (start_field, end_field))
        .unwrap()
        .iter()
            .map(|point| PointJson::from_point(*point))
            .collect();
    
    return Ok(HttpResponse::Ok().json(labyrinth_answer));
}

async fn escape_ways_response(data: web::Json<Vec<Vec<i8>>>) -> Result<HttpResponse> {
    let cloned_data = data.clone();
    let (start_point, end_point) = get_start_to_end_points(cloned_data.clone()).unwrap();
    let (fs, aps) = fs_aps_from_matrix(cloned_data.clone()).unwrap();
    let start_field = get_field_by_index(cloned_data.clone(), start_point).unwrap();
    let end_field = get_field_by_index(cloned_data.clone(), end_point).unwrap();
    let mut all_path = Vec::new();
    let mut all_path_point_json: Vec<Vec<PointJson>> = Vec::new();

    dfs_fs_aps_recursive(fs, aps, (start_field, end_field), cloned_data.len(), &mut Vec::new(), &mut Vec::new(), &mut all_path);
    
    for path in all_path {
        all_path_point_json.push(path.iter().map(|field| PointJson::from_point(field.coordinates)).collect());
    }

    return Ok(HttpResponse::Ok().json(all_path_point_json));
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

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn chinese_rings_test() {
        let size_sample: u8 = 4;

        let mut app = test::init_service(App::new().route("/chinese_rings", web::post().to(chinese_rings_response))).await;
        let req_1 = test::TestRequest::post().uri("/chinese_rings").set_json(&size_sample).to_request();
        let req_2 = test::TestRequest::post().uri("/chinese_rings").set_json(&size_sample).to_request();

        let resp = test::call_service(&mut app, req_1).await;
        let resp_body: Vec<Vec<bool>> = test::read_response_json(&mut app, req_2).await;        
        
        let expected_output = vec![
            vec![false, false, false, false], 
            vec![true, false, false, false], 
            vec![true, true, false, false], 
            vec![false, true, false, false], 
            vec![false, true, true, false], 
            vec![true, true, true, false], 
            vec![true, false, true, false], 
            vec![false, false, true, false], 
            vec![false, false, true, true], 
            vec![true, false, true, true], 
            vec![true, true, true, true]
        ];

        assert!(resp.status().is_success());
        assert_eq!(resp_body, expected_output)

    }

    #[actix_rt::test]
    async fn labyrinth_test() {
        let matrix_sample: Vec<Vec<i8>> = vec![
            vec![2, -1, 0],
            vec![0, -1, 0],
            vec![0, 0, 1]
        ];

        let mut app = test::init_service(App::new().route("/labyrinth", web::post().to(labyrinth_response))).await;
        let req_1 = test::TestRequest::post().uri("/labyrinth").set_json(&matrix_sample).to_request();
        let req_2 = test::TestRequest::post().uri("/labyrinth").set_json(&matrix_sample).to_request();

        let resp = test::call_service(&mut app, req_1).await;
        let resp_body: Vec<PointJson> = test::read_response_json(&mut app, req_2).await; 

        let expected_output = vec![
            PointJson {
                x: 2,
                y: 2
            },
            PointJson {
                x: 2,
                y: 1
            },
            PointJson {
                x: 2,
                y: 0
            },
            PointJson {
                x: 1,
                y: 0
            },
            PointJson {
                x: 0,
                y: 0
            }

        ];

        assert!(resp.status().is_success());
        assert_eq!(resp_body, expected_output)
    }

    #[actix_rt::test]
    async fn escape_ways_test() {
        let matrix_sample: Vec<Vec<i8>> = vec![
            vec![2, 0, 0],
            vec![0, -1, 0],
            vec![0, 0, 1]
        ];

        let mut app = test::init_service(App::new().route("/escape_ways", web::post().to(escape_ways_response))).await;
        let req_1 = test::TestRequest::post().uri("/escape_ways").set_json(&matrix_sample).to_request();
        let req_2 = test::TestRequest::post().uri("/escape_ways").set_json(&matrix_sample).to_request();

        let resp = test::call_service(&mut app, req_1).await;
        let resp_body: Vec<Vec<PointJson>> = test::read_response_json(&mut app, req_2).await; 

        let expected_output = vec![
            vec![
                PointJson {
                    x: 2,
                    y: 1
                },
                PointJson {
                    x: 2,
                    y: 0
                },
                PointJson {
                    x: 1,
                    y: 0
                },
                PointJson {
                    x: 0,
                    y: 0
                }
            ],
            vec![
                PointJson {
                    x: 1,
                    y: 2
                },
                PointJson {
                    x: 0,
                    y: 2
                },
                PointJson {
                    x: 0,
                    y: 1
                },
                PointJson {
                    x: 0,
                    y: 0
                }
            ]

        ];

        assert!(resp.status().is_success());
        assert_eq!(resp_body, expected_output)
    }
}