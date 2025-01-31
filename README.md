# Best Buy Stock Checker

A Rust-based application that monitors Best Buy product availability and sends SMS notifications when items become available for purchase. Built with robust error handling, anti-bot detection measures, and AWS SNS integration.

## Features

- **Multi-Product Monitoring**: Track multiple Best Buy products simultaneously
- **Real-time Availability**: Checks product stock status every 30 seconds (configurable)
- **Smart Notifications**: SMS alerts via AWS SNS when products become available
- **Anti-Detection**: Uses Chrome-like headers to avoid bot detection
- **Efficient Logging**: Detailed timestamp-based logging with product names
- **Status Memory**: Remembers previous product status to prevent duplicate notifications

## Prerequisites

- Rust (latest stable version)
- AWS Account with SNS permissions
- Active Best Buy product URLs
- Cell phone number for SMS notifications

## Installation

1. Clone and build the repository:
```bash
git clone https://github.com/yourusername/bb_product_watcher
cd bb_product_watcher
cargo build --release
```

2. Create `.env` file in the project root:
```env
RUST_LOG=info
AWS_REGION=us-west-2
AWS_ACCESS_KEY_ID=your_access_key
AWS_SECRET_ACCESS_KEY=your_secret_key
PHONE_NUMBER=+1234567890
```

3. Create `products.csv` with your desired products:
```csv
url,name
https://www.bestbuy.com/site/nvidia-geforce-rtx-5090-32gb-gddr7-graphics-card/6614151.p,RTX 5090 FE
https://www.bestbuy.com/site/nvidia-geforce-rtx-4090-24gb-gddr6x/6521430.p,RTX 4090 FE
```

## AWS Configuration

1. Create an AWS account if you don't have one
2. Set up an IAM user:
   - Go to IAM in AWS Console
   - Create new user
   - Attach `AmazonSNSFullAccess` policy
3. Generate access keys:
   - Under Security Credentials
   - Create Access Key
   - Save both Access Key ID and Secret
4. Add keys to your `.env` file

## Usage

1. Start the application:
```bash
cargo run --release
```

2. Monitor the logs for status:
```
2025-01-30 11:12:42 [INFO] - 🔥 Starting Best Buy product availability checker...
2025-01-30 11:12:42 [INFO] - 🔎 Checking product: RTX 5090 FE
```

3. Receive SMS notifications when products become available

## Configuration Options

### Check Interval
Modify `CHECK_INTERVAL` in `main.rs` to adjust check frequency (default: 30 seconds)
```rust
const CHECK_INTERVAL: u64 = 30; // Time in seconds
```

### Product List
Update `products.csv` anytime to add/remove products. Format:
```csv
url,name
[product-url],[display-name]
```

### Logging
Adjust logging level in `.env`:
```env
RUST_LOG=debug  # For more detailed logs
RUST_LOG=info   # For standard logs
```

## Error Handling

- Connection failures: Automatic retry on next interval
- Invalid URLs: Logged and skipped
- AWS failures: Logged with error details
- Bot detection: Uses browser-like headers to minimize blocks

## Performance

- Memory usage: ~10MB idle
- CPU usage: Minimal (~1-2% during checks)
- Network: ~100KB per product check

## Limitations

- Best Buy may rate limit excessive requests
- SMS charges may apply through AWS
- Products must be from bestbuy.com

## License

MIT License

## Disclaimer

This tool is for personal use only. Excessive requests may result in IP blocks from Best Buy. Users are responsible for any AWS charges incurred through SNS usage.