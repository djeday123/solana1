# Solana DEX for Students

A simple educational Decentralized Exchange (DEX) built on Solana to help students learn about:
- Solana program development
- AMM (Automated Market Maker) mechanics
- Liquidity pools and swapping
- DeFi concepts and implementation

## 🎯 Learning Objectives

This project helps students understand:
1. **Smart Contract Development**: How to build Solana programs
2. **AMM Mechanics**: Constant product formula (x * y = k)
3. **Liquidity Provision**: Adding and removing liquidity from pools
4. **Token Swapping**: How users can swap between different tokens
5. **Price Discovery**: How prices are determined in AMMs

## 📁 Project Structure

```
solana-student-dex/
├── programs/
│   └── student-dex/           # Solana program (smart contract)
│       ├── src/
│       │   ├── lib.rs         # Main program entry point
│       │   ├── instructions/  # Program instructions
│       │   └── state/         # Account structures
│       └── Cargo.toml
├── app/                       # Frontend React application
│   ├── src/
│   │   ├── components/        # React components
│   │   ├── utils/             # Utility functions
│   │   └── hooks/             # Custom React hooks
│   └── package.json
├── tests/                     # Integration tests
└── scripts/                   # Deployment and utility scripts
```

## 🚀 Core Features

### 1. Liquidity Pool Creation
- Students can create new trading pairs
- Initialize pools with token ratios
- Learn about initial liquidity provision

### 2. Add/Remove Liquidity
- Provide liquidity to earn fees
- Understand impermanent loss
- Calculate LP token shares

### 3. Token Swapping
- Swap between any tokens in available pools
- Real-time price calculation
- Slippage protection

### 4. Educational Dashboard
- Real-time pool statistics
- Visual representation of AMM curves
- Transaction history and analytics

## 🛠 Technology Stack

- **Blockchain**: Solana
- **Smart Contracts**: Rust + Solana SDK
- **Frontend**: React + TypeScript
- **Wallet Integration**: Solana Wallet Adapter
- **Styling**: Tailwind CSS

## 📚 Educational Resources

### AMM Basics
The DEX uses the constant product formula: `x * y = k`
- `x` = Token A reserves
- `y` = Token B reserves  
- `k` = Constant product

### Price Calculation
```
Price of Token A = Token B reserves / Token A reserves
Price of Token B = Token A reserves / Token B reserves
```

### Swap Calculation
For swapping `dx` amount of Token A for Token B:
```
dy = (y * dx) / (x + dx)
```

## 🏃‍♂️ Quick Start

### Prerequisites
- Node.js 16+
- Rust 1.60+
- Solana CLI 1.14+
- Phantom/Solflare wallet

### Setup Instructions

1. **Clone the repository**
```bash
git clone https://github.com/djeday123/solana1.git
cd solana1
```

2. **Install dependencies**
```bash
# Install Rust dependencies
cargo build

# Install frontend dependencies
cd app && npm install
```

3. **Deploy to Devnet**
```bash
# Set Solana cluster to devnet
solana config set --url devnet

# Deploy the program
cargo build-bpf
solana program deploy target/deploy/student_dex.so
```

4. **Run the frontend**
```bash
cd app && npm start
```

## 🎓 Student Exercises

### Exercise 1: Understanding AMM
1. Create a pool with 100 TokenA and 200 TokenB
2. Calculate the initial price of each token
3. Perform a swap and observe price changes

### Exercise 2: Liquidity Provision
1. Add liquidity to an existing pool
2. Calculate your share of the pool
3. Remove liquidity and see the changes

### Exercise 3: Arbitrage Opportunities
1. Create price differences between pools
2. Identify arbitrage opportunities
3. Execute arbitrage trades

## 🔒 Security Considerations

This is an educational project. Key security features implemented:
- Slippage protection
- Overflow/underflow checks
- Access control for admin functions
- Input validation

**⚠️ Warning**: This is for educational purposes only. Do not use in production without proper security audits.

## 📈 Advanced Features

- **Fee Collection**: 0.3% trading fee distribution
- **Governance**: Community voting on parameters
- **Multi-hop Swaps**: Route through multiple pools
- **Price Oracles**: Integration with external price feeds

## 🤝 Contributing

Students and educators are welcome to contribute:
1. Add new educational exercises
2. Improve documentation
3. Add new features
4. Report bugs

## 📖 Additional Resources

- [Solana Documentation](https://docs.solana.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [AMM Theory](https://ethereum.org/en/developers/docs/dexs/)
- [DeFi Explainer](https://finematics.com/automated-market-maker-amm-explained/)

## 📄 License

MIT License - see LICENSE file for details.

---

**Built with ❤️ for Solana education**
