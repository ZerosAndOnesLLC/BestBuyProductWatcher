use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Client;
use scraper::{Html, Selector};
use std::time::Duration;
use tokio::time::sleep;
use log::{info, error, warn};
use std::io::Write;
use dotenv::dotenv;

const PRODUCT_URL: &str = "https://www.bestbuy.com/site/nvidia-geforce-rtx-5090-32gb-gddr7-graphics-card-dark-gun-metal/6614151.p?skuId=6614151";
const CHECK_INTERVAL: u64 = 30;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let env = env_logger::Env::default().filter_or("RUST_LOG", "info");
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();

    info!("🔥 Starting Best Buy GPU availability checker...");
    let client = create_client();

    loop {
        info!("🔎 Checking product availability...");
        match check_product_availability(&client).await {
            Ok(true) => {
                info!("✅ PRODUCT AVAILABLE! GO BUY IT NOW!!!");
                send_notification().await;
            }
            Ok(false) => info!("❌ Product is still out of stock."),
            Err(e) => error!("⚠️ Error checking product: {}", e),
        }

        info!("⏳ Sleeping for {} seconds before next check...", CHECK_INTERVAL);
        sleep(Duration::from_secs(CHECK_INTERVAL)).await;
    }
}

fn create_client() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36"));
    headers.insert("accept", HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    headers.insert("accept-language", HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert("cache-control", HeaderValue::from_static("no-cache"));
    headers.insert("pragma", HeaderValue::from_static("no-cache"));
    headers.insert("sec-ch-ua", HeaderValue::from_static("\"Chromium\";v=\"122\", \"Google Chrome\";v=\"122\", \"Not(A:Brand\";v=\"24\""));
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
    headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"Windows\""));
    headers.insert("sec-fetch-dest", HeaderValue::from_static("document"));
    headers.insert("sec-fetch-mode", HeaderValue::from_static("navigate"));
    headers.insert("sec-fetch-site", HeaderValue::from_static("none"));
    headers.insert("sec-fetch-user", HeaderValue::from_static("?1"));
    headers.insert("upgrade-insecure-requests", HeaderValue::from_static("1"));

    Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed to create HTTP client")
}

async fn check_product_availability(client: &Client) -> Result<bool, reqwest::Error> {
    let response = client.get(PRODUCT_URL).send().await?;
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

/* async fn send_notification() {
    info!("🚀 GO BUY IT NOW! DO NOT WASTE TIME!!!!!! GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO ");
    // TODO: Implement actual notification logic (Webhook, Email, SMS, etc.)
} */

async fn send_notification() {
    info!("🚀 GO BUY IT NOW! DO NOT WASTE TIME!!!!!! GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO GO ");

    let config = aws_config::load_from_env().await;
    let client = aws_sdk_sns::Client::new(&config);

    let phone = std::env::var("PHONE_NUMBER").expect("PHONE_NUMBER must be set");
    let msg = "🚨 RTX 5090 IN STOCK! GO BUY NOW: ".to_string() + PRODUCT_URL;

    match client.publish()
        .phone_number(phone)
        .message(msg)
        .send()
        .await 
    {
        Ok(_) => info!("✅ SMS notification sent successfully!"),
        Err(e) => error!("❌ Failed to send SMS: {}", e),
    }
}