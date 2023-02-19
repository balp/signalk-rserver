use awc::Client;
use serde_json::Value;




#[actix_web::main]
async fn main() {
    let client = Client::default();

    let req = client
        .get("http://homenuc:3002/signalk/v1/api/")
        .insert_header(("User-Agent", "awc/3.0"));
    let mut res = req.send().await.unwrap();

    println!("Response: {:?}", res);
    let json: Value = res.json().await.unwrap();
    println!("JSON: {:?}", json);
}