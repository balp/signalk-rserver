use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde_json::json;

use signalk_rserver::signalk::full;

mod signalk;

#[get("/signalk/v1/api/")]
async fn signalk_v1(data: web::Data<full::V1FullFormat>) -> Result<impl Responder> {
    Ok(web::Json(data))
}

#[get("/signalk")]
async fn signalk_discovery() -> impl Responder {
    let body = json!({
            "endpoints": {
                "v1": {
                    "version": "1.7.0",
                    "signalk-http": "http://localhost:3001/signalk/v1/api/",
                }
            },
            "server": {
                "id": "signalk-rserver",
                "version": "0.1.0"
            }
    });
    HttpResponse::Ok().body(body.to_string())
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let self_uuid = "";
    // let self_link = format!("vessels.urn:mrn:signalk:uuid:{self_uuid}");
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(full::V1FullFormat {
                // vessels: signalk::V1VesselFormat{ mmsi: "826512345".to_string()},
                version: "1.7.0".to_string(),
                self_: "vessels.urn:mrn:signalk:uuid:d6d08b72-88e2-4911-9429-ede4d5819549"
                    .to_string(),
                vessels: None,
                sources: None,
            }))
            .service(signalk_discovery)
            .service(signalk_v1)
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
