#!/bin/bash

# Student DEX - Devnet Setup Script
# This script helps students set up their development environment

echo "🎓 Student DEX - Setting up Solana Devnet Environment"
echo "=================================================="

# Check if Solana CLI is installed
if ! command -v solana &> /dev/null; then
    echo "❌ Solana CLI not found. Please install it from https://docs.solana.com/cli/install-solana-cli-tools"
    exit 1
fi

echo "✅ Solana CLI found"

# Set cluster to devnet
echo "🌐 Setting Solana cluster to devnet..."
solana config set --url devnet

# Generate a new keypair for students (if none exists)
if [ ! -f ~/.config/solana/id.json ]; then
    echo "🔑 Generating new keypair..."
    solana-keygen new --outfile ~/.config/solana/id.json --no-bip39-passphrase
else
    echo "✅ Keypair already exists"
fi

# Show wallet address
WALLET_ADDRESS=$(solana address)
echo "📍 Your wallet address: $WALLET_ADDRESS"

# Check balance
BALANCE=$(solana balance)
echo "💰 Current balance: $BALANCE"

# Airdrop some SOL if balance is low
if [[ $BALANCE == "0 SOL" ]]; then
    echo "💸 Requesting airdrop of 2 SOL for testing..."
    solana airdrop 2
    echo "✅ Airdrop completed"
else
    echo "✅ You have sufficient SOL for testing"
fi

echo ""
echo "🎉 Setup complete!"
echo ""
echo "Next steps:"
echo "1. Build the Solana program: cd programs/student-dex && cargo build-bpf"
echo "2. Deploy to devnet: solana program deploy target/deploy/student_dex.so"
echo "3. Start the frontend: cd app && npm start"
echo ""
echo "💡 Useful Solana commands for students:"
echo "  solana balance          - Check your SOL balance"
echo "  solana airdrop 1        - Request 1 SOL from faucet"
echo "  solana address          - Show your wallet address"
echo "  solana transaction <tx> - View transaction details"
echo ""
echo "📚 Learn more: https://docs.solana.com/"