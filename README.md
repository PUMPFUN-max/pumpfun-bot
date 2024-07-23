# PumpFun Trading Bot

Welcome to the PumpFun Trading Bot repository! This bot is designed for traders on the PumpFun trading platform who wish to learn how trading bots are created and how they operate. The bot monitors new coins, buys them, waits for 10 seconds, and then sells them. 

## Overview

This project provides an educational example of a trading bot. The bot's main functionality includes:

1. **Monitoring New Coins**: The bot continuously monitors for the release of new coins.
2. **Buying**: Once a new coin is detected, the bot executes a buy order.
3. **Sleeping**: The bot pauses for 10 seconds to allow the market to react.
4. **Selling**: After the pause, the bot executes a sell order.

## How It Works

The bot spams transactions to ensure it gets an entry as soon as possible. The effectiveness of this approach depends heavily on the quality of the provided node. For instance, my test on a cheap 1 SOL node and 50 spammed transactions, got an entry in about 10 seconds.
Note: If you plan running it in 

### Key File

- **check_logs_buy_sell.rs**: Contains the core logic for buying, sleeping, and selling. The sleep function is located at line 208.

## Environment Variables

Below are the necessary environment variables to configure the bot:

```env
PAYER=your_private_key
spam_limit=50
budget_limit=80000
budget_price=10000
investment=0.02
slippage=0.0 
RPC_HTTPS_URL=http://aaaaaaaaa.com
WSS_HTTPS_URL=wss://aaaaaaaaa.com
```

**Note**: The `slippage` variable in the environment is not used and can be ignored.

## How to Run

### Prerequisites

- **Rust**: Ensure you have Rust installed. You can install Rust using [rustup](https://rustup.rs/).

### Steps

1. **Clone the Repository**:
   ```bash
   git clone <repository-url>
   cd <repository-directory>
   ```

2. **Rename the Repository (for VS Code Users)**:
   If you are using VS Code, rename the repository to `sniper`:

3. **Build the Project**:
   ```bash
   cargo build 
   ```

4. **Set Up Environment Variables**:
   Create a `.env` file in the project directory and add the necessary environment variables as described above.

5. **Run the Bot**:
   ```bash
   cargo run 
   ```
   
## Disclaimer

This bot is for educational purposes only. It is designed to demonstrate how trading bots function and should not be used for actual trading without a thorough understanding of the risks involved.

## Contact

For any questions, suggestions, or feedback, please reach out to the developer:

- **All contacts are in my profile**

Happy Learning!
