# Best Buy Product Availability Checker

Rust-based tool to monitor Best Buy product availability and send SMS notifications when items become available for purchase.

## Features

- Monitor multiple Best Buy products
- Real-time availability checking
- SMS notifications via AWS SNS
- Configurable check intervals
- Bot detection avoidance
- Detailed logging

## Prerequisites

- Rust (latest stable)
- AWS account with SNS access
- AWS credentials configured

## Installation

1. Clone and build:
```bash
git clone https://github.com/ZerosAndOnesLLC/BestBuyProductWatcher
cd bb_product_watcher
cargo build --release
```

2. Create `.env`:
```
RUST_LOG=info
AWS_REGION=us-west-2
AWS_ACCESS_KEY_ID=your_access_key
AWS_SECRET_ACCESS_KEY=your_secret_key
PHONE_NUMBER=+1234567890
```

3. Create `products.txt` (CSV format):
```
https://www.bestbuy.com/site/product-url,Product Name
https://www.bestbuy.com/site/nvidia-geforce...,RTX 5090 FE
```

## Usage

```bash
cargo run --release
```

## AWS Setup

1. Create AWS account
2. Create IAM user with SNS permissions
3. Generate access keys
4. Add to .env file

## Configuration

Modify `CHECK_INTERVAL` in main.rs to adjust check frequency (default: 30 seconds).

## License

MIT

## Disclaimer

Use responsibly. Excessive requests may be blocked by Best Buy.