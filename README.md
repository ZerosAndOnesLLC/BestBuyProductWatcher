# Best Buy Product Availability Checker

Rust-based tool to monitor Best Buy product availability and send SMS notifications when items become available for purchase.

## Features

- Monitor multiple Best Buy product URLs simultaneously
- Real-time availability checking
- SMS notifications via AWS SNS
- Configurable check intervals
- Browser-like request headers to avoid blocking
- Detailed logging

## Prerequisites

- Rust (latest stable)
- AWS account with SNS access
- AWS credentials configured

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/bb_product_watcher
cd bb_product_watcher
```

2. Create a `.env` file:
```
RUST_LOG=info
AWS_REGION=us-west-2
AWS_ACCESS_KEY_ID=your_access_key
AWS_SECRET_ACCESS_KEY=your_secret_key
PHONE_NUMBER=+1234567890
```

3. Create `products.txt` with URLs to monitor (one per line):
```
https://www.bestbuy.com/site/product1...
https://www.bestbuy.com/site/product2...
```

## Usage

```bash
cargo run
```

The app will:
1. Load product URLs from products.txt
2. Check each product's availability every 30 seconds
3. Send SMS when a product becomes available
4. Continue monitoring until stopped

## Configuration

- Edit `CHECK_INTERVAL` in main.rs to modify check frequency
- Adjust AWS region in .env if needed
- Modify user agent and headers in create_client() if required

## AWS Setup

1. Create an AWS account
2. Set up an IAM user with SNS permissions
3. Generate access keys
4. Add keys to .env file

## Contributing

1. Fork the repository
2. Create feature branch
3. Commit changes
4. Push to branch
5. Create Pull Request

## License

MIT

## Disclaimer

Use responsibly. Excessive requests may be blocked by Best Buy.
