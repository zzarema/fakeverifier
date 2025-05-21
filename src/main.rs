use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Clone)]
struct Product {
    productId: String,
    productName: String,
    manufacturer: String,
    origin: String,
    authentic: bool,
}

// Статическая база продуктов
fn products_db() -> HashMap<&'static str, Product> {
    let mut map = HashMap::new();
    map.insert("1234567890", Product {
        productId: "1234567890".into(),
        productName: "Adidas Running Shoes".into(),
        manufacturer: "Adidas Co.".into(),
        origin: "Germany".into(),
        authentic: true,
    });
    map.insert("9876543210", Product {
        productId: "9876543210".into(),
        productName: "Apple AirPods Pro".into(),
        manufacturer: "Apple Inc.".into(),
        origin: "USA".into(),
        authentic: true,
    });
    map.insert("000000", Product {
        productId: "000000".into(),
        productName: "Basic T-Shirt".into(),
        manufacturer: "Generic Wear".into(),
        origin: "Bangladesh".into(),
        authentic: false,
    });
    map.insert("111111", Product {
        productId: "111111".into(),
        productName: "Smartwatch".into(),
        manufacturer: "Unknown".into(),
        origin: "China".into(),
        authentic: false,
    });
    map.insert("222222", Product {
        productId: "222222".into(),
        productName: "Nike Sneakers".into(),
        manufacturer: "Nike Inc.".into(),
        origin: "Vietnam".into(),
        authentic: true,
    });
    map.insert("333333", Product {
        productId: "333333".into(),
        productName: "Bluetooth Speaker".into(),
        manufacturer: "AudioTech".into(),
        origin: "South Korea".into(),
        authentic: true,
    });
    map
}

#[get("/api/product/{product_id}")]
async fn get_product(path: web::Path<String>) -> impl Responder {
    let product_id = path.into_inner();
    let db = products_db();

    match db.get(product_id.as_str()) {
        Some(product) => HttpResponse::Ok().json(product),
        None => HttpResponse::NotFound().body("Product not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Fake Detector API is running at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(get_product)
            // Статические файлы из папки frontend (тут будет index.html и прочее)
            .service(Files::new("/", "./frontend").index_file("index.html"))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
