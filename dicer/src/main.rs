use actix_web::{App, get, HttpResponse, HttpServer, web};
use rand::distributions::{Distribution, Uniform};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct RollRequest {
    sides: u32,
    count: u32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct RollResponse {
    count: u32,
    sum: u64,
    results: Vec<u32>,
}

fn roll_dice(sides: u32, count: u32) -> RollResponse {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..=sides);
    let mut results: Vec<u32> = Vec::new();
    let mut sum: u64 = 0;
    for _ in 0..count {
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

#[cfg(not(tarpaulin_include))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/roll").service(roller)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use actix_web::{App, test};

    use super::*;

    #[test]
    fn test_roll_dice_sanity() {
        let expected = RollResponse{
            count: 1,
            sum: 1,
            results: vec![1]
        };
        let result = roll_dice(1,1);
        assert_eq!(result, expected);
    }

    #[actix_rt::test]
    async fn test_webserver_sanity() {
        let expected = RollResponse{
            count: 1,
            sum: 1,
            results: vec![1]
        };
        let mut app = test::init_service(
            App::new()
                .service(
                    web::scope("/roll").service(roller)
                )
        ).await;
        let req = test::TestRequest::get().uri("/roll/1/1").to_request();
        let resp: RollResponse = test::read_response_json(&mut app, req).await;

        assert_eq!(resp, expected);
    }
}
