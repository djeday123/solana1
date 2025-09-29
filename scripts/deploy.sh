#!/bin/bash

# Student DEX - Deployment Script
# Builds and deploys the Solana program to devnet

echo "🚀 Student DEX - Building and Deploying"
echo "======================================"

# Check if we're in the right directory
if [ ! -d "programs/student-dex" ]; then
    echo "❌ Error: Run this script from the project root directory"
    exit 1
fi

# Check Solana CLI
if ! command -v solana &> /dev/null; then
    echo "❌ Solana CLI not found. Please run setup-devnet.sh first"
    exit 1
fi

# Ensure we're on devnet
echo "🌐 Setting cluster to devnet..."
solana config set --url devnet

# Build the program
echo "🔨 Building Solana program..."
cd programs/student-dex

if ! cargo build-bpf; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful!"

# Check if we have enough SOL for deployment
BALANCE=$(solana balance | grep -o "[0-9.]*")
if (( $(echo "$BALANCE < 1" | bc -l) )); then
    echo "💸 Low balance ($BALANCE SOL). Requesting airdrop..."
    solana airdrop 2
fi

# Deploy the program
echo "🚀 Deploying to devnet..."
PROGRAM_ID=$(solana program deploy target/deploy/student_dex.so | grep "Program Id:" | awk '{print $3}')

if [ -z "$PROGRAM_ID" ]; then
    echo "❌ Deployment failed!"
    exit 1
fi

echo "✅ Deployment successful!"
echo "📍 Program ID: $PROGRAM_ID"

# Save program ID for frontend
echo "💾 Saving program ID to frontend config..."
cd ../../app/src
echo "export const STUDENT_DEX_PROGRAM_ID = '$PROGRAM_ID';" > config.ts

echo ""
echo "🎉 Deployment complete!"
echo ""
echo "Program ID: $PROGRAM_ID"
echo ""
echo "Next steps:"
echo "1. Update frontend to use this Program ID"
echo "2. Start frontend: cd app && npm start"
echo "3. Test the DEX functionality"
echo ""
echo "💡 To verify deployment:"
echo "  solana program show $PROGRAM_ID"