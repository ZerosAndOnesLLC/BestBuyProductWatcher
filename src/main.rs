use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Client;
use scraper::{Html, Selector};
use std::time::Duration;
use tokio::time::sleep;
use log::{info, error, warn};
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;
use dotenv::dotenv;
use std::io::Write;

const CHECK_INTERVAL: u64 = 30;

struct Product {
    url: String,
    name: String,
    last_status: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let env = env_logger::Env::default().filter_or("RUST_LOG", "info");
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("🔥 Starting Best Buy product availability checker...");
    let client = create_client();
    check_products(&client).await?;
    
    Ok(())
}

fn create_client() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36"));
    headers.insert("accept", HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    headers.insert("accept-language", HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert("accept-encoding", HeaderValue::from_static("gzip, deflate, br"));
    headers.insert("cache-control", HeaderValue::from_static("no-cache"));
    headers.insert("pragma", HeaderValue::from_static("no-cache"));
    headers.insert("sec-ch-ua", HeaderValue::from_static("\"Not A(Brand\";v=\"99\", \"Google Chrome\";v=\"121\", \"Chromium\";v=\"121\""));
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
    headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"Windows\""));
    headers.insert("sec-fetch-dest", HeaderValue::from_static("document"));
    headers.insert("sec-fetch-mode", HeaderValue::from_static("navigate"));
    headers.insert("sec-fetch-site", HeaderValue::from_static("none"));
    headers.insert("sec-fetch-user", HeaderValue::from_static("?1"));
    headers.insert("upgrade-insecure-requests", HeaderValue::from_static("1"));
    headers.insert("referer", HeaderValue::from_static("https://www.bestbuy.com"));
    headers.insert("origin", HeaderValue::from_static("https://www.bestbuy.com"));
    headers.insert("cookie", HeaderValue::from_static("locStoreId=281"));

    Client::builder()
        .gzip(true)
        .default_headers(headers)
        .timeout(Duration::from_secs(15))
        .build()
        .expect("Failed to create HTTP client")
}

async fn check_products(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("products.csv");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut products: Vec<Product> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 2 {
            error!("Invalid line in products.txt: {}", line);
            continue;
        }
        
        products.push(Product {
            url: parts[0].trim().to_string(),
            name: parts[1].trim().to_string(),
            last_status: false,
        });
    }

    loop {
        for product in &mut products {
            info!("🔎 Checking product: {}", product.name);
            
            match check_single_product(client, &product.url).await {
                Ok(true) => {
                    if !product.last_status {
                        info!("✅ PRODUCT AVAILABLE: {}", product.name);
                        send_notification_with_details(&product.url, &product.name).await;
                        product.last_status = true;
                    }
                },
                Ok(false) => {
                    product.last_status = false;
                    info!("❌ Product still out of stock: {}", product.name);
                },
                Err(e) => error!("⚠️ Error checking product {}: {}", product.name, e),
            }
        }

        info!("⏳ Sleeping for {} seconds before next check...", CHECK_INTERVAL);
        sleep(Duration::from_secs(CHECK_INTERVAL)).await;
    }
}

async fn check_single_product(client: &Client, url: &str) -> Result<bool, reqwest::Error> {
    let response = client.get(url).send().await?;
    info!("Response status: {}", response.status());
    
    let body = response.text().await?;
    info!("Response body length: {}", body.len());
    
    let document = Html::parse_document(&body);
    
    // Direct "Sold Out" button selector
    let sold_out_selector = Selector::parse("button.c-button.c-button-disabled.c-button-lg.add-to-cart-button").unwrap();
    if let Some(element) = document.select(&sold_out_selector).next() {
        let button_text = element.text().collect::<Vec<_>>().join(" ");
        let trimmed_text = button_text.trim();
        info!("Found primary button: {}", trimmed_text);
        return Ok(!trimmed_text.to_lowercase().contains("sold out"));
    }
    
    // Backup check for fulfillment button
    let fulfillment_selector = Selector::parse("[data-button-state='ADD_TO_CART'], [data-button-state='SOLD_OUT']").unwrap();
    if let Some(element) = document.select(&fulfillment_selector).next() {
        let state = element.value().attr("data-button-state").unwrap_or("");
        info!("Button state: {}", state);
        return Ok(state == "ADD_TO_CART");
    }

    warn!("Could not find status button");
    Ok(false)
}

async fn send_notification_with_details(url: &str, name: &str) {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_sns::Client::new(&config);

    let phone = std::env::var("PHONE_NUMBER").expect("PHONE_NUMBER must be set");
    let msg = format!("🚨 PRODUCT IN STOCK! GO BUY NOW: GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO {}", url);

    match client.publish()
        .phone_number(phone)
        .message(msg)
        .send()
        .await 
    {
        Ok(_) => info!("✅ SMS notification sent for: {}", name),
        Err(e) => error!("❌ Failed to send SMS for {}: {}", name, e),
    }
}