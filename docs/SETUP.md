# Setup Guide for Solana DEX

This guide will walk you through setting up the Solana DEX for educational purposes.

## Prerequisites

Before you begin, ensure you have the following installed:

### Required Software

1. **Rust Programming Language**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   rustup component add rustfmt
   ```

2. **Node.js and npm**
   - Download from [nodejs.org](https://nodejs.org/) (v16 or higher)
   - Or use a version manager like nvm:
   ```bash
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
   nvm install 18
   nvm use 18
   ```

3. **Solana CLI Tools**
   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/v1.16.0/install)"
   export PATH="/home/$USER/.local/share/solana/install/active_release/bin:$PATH"
   ```

4. **Git**
   ```bash
   # Ubuntu/Debian
   sudo apt update && sudo apt install git
   
   # macOS
   brew install git
   
   # Windows
   # Download from https://git-scm.com/
   ```

### Optional but Recommended

5. **Visual Studio Code** with extensions:
   - Rust Analyzer
   - Solana Development Extension
   - JavaScript/TypeScript support

6. **Phantom Wallet** or other Solana wallet
   - Download from [phantom.app](https://phantom.app/)
   - Create a new wallet for development

## Environment Setup

### 1. Configure Solana CLI

```bash
# Set to devnet for educational use
solana config set --url https://api.devnet.solana.com

# Generate a new keypair
solana-keygen new --outfile ~/.config/solana/id.json

# Get some devnet SOL for testing
solana airdrop 2

# Verify your setup
solana balance
solana config get
```

### 2. Clone and Setup the Project

```bash
# Clone the repository
git clone https://github.com/djeday123/solana1.git
cd solana1

# Build the Solana program
cd programs/solana-dex
cargo build-bpf
cd ../..

# Build the client library
cd client
cargo build
cd ..

# Setup the web application
cd app
cp .env.example .env
npm install
```

### 3. Environment Configuration

Edit the `.env` file in the `app/` directory:

```env
# Development server port
PORT=3000
NODE_ENV=development

# Solana configuration
SOLANA_RPC_URL=https://api.devnet.solana.com
SOLANA_NETWORK=devnet

# This will be updated after deploying the program
DEX_PROGRAM_ID=DEX1111111111111111111111111111111111111111

# Educational features
ENABLE_TUTORIAL=true
SIMULATION_MODE=true
```

## Deployment (Optional)

### Deploy the Solana Program

1. **Build the program:**
   ```bash
   cd programs/solana-dex
   cargo build-bpf
   ```

2. **Deploy to devnet:**
   ```bash
   solana program deploy target/deploy/solana_dex_program.so
   ```

3. **Update the program ID:**
   - Copy the program ID from the deployment output
   - Update `declare_id!()` in `src/lib.rs`
   - Update `DEX_PROGRAM_ID` in your `.env` file
   - Rebuild and redeploy

### Run the Application

```bash
# Start the development server
cd app
npm run dev
```

Open your browser to `http://localhost:3000`

## Verification

### Test the Setup

1. **Check Solana program compilation:**
   ```bash
   cd programs/solana-dex
   cargo test
   ```

2. **Check client library:**
   ```bash
   cd client
   cargo test
   ```

3. **Check web application:**
   ```bash
   cd app
   npm test
   ```

4. **Verify wallet connection:**
   - Open the web app
   - Click "Connect Wallet"
   - Approve the connection in Phantom
   - Check that your wallet address appears

## Troubleshooting

### Common Issues

1. **"Program not found" error:**
   - Ensure you've deployed the program to devnet
   - Check that the program ID in your code matches the deployed program

2. **Wallet connection fails:**
   - Ensure Phantom wallet is installed and set to devnet
   - Clear browser cache and cookies
   - Try refreshing the page

3. **Build errors:**
   - Check Rust version: `rustc --version`
   - Ensure all dependencies are installed
   - Try `cargo clean` and rebuild

4. **Node.js issues:**
   - Check Node.js version: `node --version`
   - Clear npm cache: `npm cache clean --force`
   - Delete `node_modules` and run `npm install` again

### Getting Help

- Check the main [README.md](../README.md) for detailed documentation
- Review the [FAQ](FAQ.md) for common questions
- Open an issue on GitHub if you encounter problems
- Join our Discord for community support

## Next Steps

Once your setup is complete:

1. **Explore the Interface:** Navigate through the web application
2. **Try the Tutorial:** Click the tutorial button to learn DeFi concepts
3. **Make Test Trades:** Use devnet tokens to practice swapping
4. **Add Liquidity:** Learn about liquidity provision
5. **Study the Code:** Examine the Rust program and client code
6. **Modify and Experiment:** Try making small changes to understand the system

## Educational Resources

- [Solana Documentation](https://docs.solana.com/)
- [Rust Programming Language](https://doc.rust-lang.org/book/)
- [DeFi Concepts Explained](https://ethereum.org/en/defi/)
- [Automated Market Makers](https://chain.link/education-hub/what-is-an-automated-market-maker-amm)

Remember: This is educational software for learning purposes. Never use real mainnet funds with this code without proper security audits and testing.