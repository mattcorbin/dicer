use actix_web::{App, get, HttpResponse, HttpServer, web};
use rand::distributions::{Distribution, Uniform};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct RollRequest {
    sides: u32,
    count: u32,
}

#[derive(Serialize)]
struct RollResponse {
    count: u32,
    sum: u64,
    results: Vec<u32>,
}

fn roll_dice(sides: u32, count: u32) -> RollResponse {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..sides);
    let mut results: Vec<u32> = Vec::new();
    let mut sum: u64 = 0;
    for _ in 1..count {
        let roll = die.sample(&mut rng);
        results.push(roll);
        sum += roll as u64;
    }
    RollResponse {
        count,
        sum,
        results,
    }
}

#[get("/{sides}/{count}")]
async fn roller(req: web::Path<RollRequest>) -> HttpResponse {
    HttpResponse::Ok().json(roll_dice(req.sides, req.count))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/roll").service(roller)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
