# Rust IBKR API Restful

This project is a Rust application that scans all option chains of NASDAQ stocks using the IBKR Web API v1.0. It is designed to identify profitable strike prices based on a specified profit margin.

## Features

- Scans all option chains for NASDAQ stocks.
- Utilizes the IBKR Web API for fetching data.
- Analyzes option chains to find strikes with a profit margin of at least 2%.

## Project Structure

```
rust_ibkr_api_restful
├── src
│   ├── main.rs          # Entry point of the application
│   ├── lib.rs           # Library root, exports main functionalities
│   ├── api              # API module for IBKR interactions
│   │   ├── mod.rs       # API module declaration
│   │   ├── client.rs    # API client implementation
│   │   └── gateway.rs    # Gateway for IBKR API
│   ├── models           # Data models for option chains and strikes
│   │   ├── mod.rs       # Models module declaration
│   │   ├── option_chain.rs # Option chain data structures
│   │   └── strike.rs    # Strike price data structures
│   ├── scanner          # Module for scanning and analyzing options
│   │   ├── mod.rs       # Scanner module declaration
│   │   └── analyzer.rs  # Logic for analyzing option chains
│   └── utils            # Utility functions for data processing
│       ├── mod.rs       # Utils module declaration
│       └── helpers.rs   # Helper functions
├── tests                # Integration tests for the application
│   └── integration_tests.rs # Tests for API client and scanning functionalities
├── Cargo.toml           # Project configuration and dependencies
└── README.md            # Project documentation
```

## Setup Instructions

1. Ensure you have Rust installed on your machine. You can install it from [rust-lang.org](https://www.rust-lang.org/).
2. Clone the repository:
   ```
   git clone <repository-url>
   cd rust_ibkr_api_restful
   ```
3. Install the required dependencies by running:
   ```
   cargo build
   ```
4. Configure your IBKR API credentials in the appropriate configuration file or environment variables.
5. Run the application:
   ```
   cargo run
   ```

## Usage

After running the application, it will automatically scan the option chains for NASDAQ stocks and output the profitable strikes based on the defined criteria.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.