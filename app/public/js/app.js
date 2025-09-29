// Solana DEX Educational App
class SolanaDexApp {
    constructor() {
        this.connection = null;
        this.wallet = null;
        this.programId = new solanaWeb3.PublicKey('DEX1111111111111111111111111111111111111111');
        this.isConnected = false;
        
        this.init();
    }

    async init() {
        // Initialize connection to devnet for educational purposes
        this.connection = new solanaWeb3.Connection(
            'https://api.devnet.solana.com',
            'confirmed'
        );
        
        this.bindEvents();
        this.updateUI();
        
        // Check if wallet is available
        this.checkWalletAvailability();
    }

    checkWalletAvailability() {
        if (typeof window.solana === 'undefined') {
            this.showNotification('Please install a Solana wallet (like Phantom) to use this DEX', 'warning');
        }
    }

    bindEvents() {
        // Wallet connection
        document.getElementById('connectWallet').addEventListener('click', () => this.connectWallet());
        document.getElementById('disconnect').addEventListener('click', () => this.disconnectWallet());
        
        // Swap functionality
        document.getElementById('swapDirection').addEventListener('click', () => this.swapTokenSelection());
        document.getElementById('fromAmount').addEventListener('input', (e) => this.calculateSwapOutput(e.target.value));
        document.getElementById('swapButton').addEventListener('click', () => this.performSwap());
        
        // Liquidity functionality
        document.getElementById('addLiquidityButton').addEventListener('click', () => this.addLiquidity());
        document.getElementById('liquidityAmountA').addEventListener('input', () => this.calculateLiquidityB());
        
        // Slippage slider
        document.getElementById('slippageSlider').addEventListener('input', (e) => {
            document.getElementById('slippageValue').textContent = e.target.value + '%';
        });
    }

    async connectWallet() {
        try {
            if (typeof window.solana === 'undefined') {
                throw new Error('Solana wallet not found');
            }

            const response = await window.solana.connect();
            this.wallet = window.solana;
            this.isConnected = true;
            
            this.showNotification('Wallet connected successfully!', 'success');
            this.updateUI();
            this.loadBalances();
            
        } catch (error) {
            console.error('Failed to connect wallet:', error);
            this.showNotification('Failed to connect wallet: ' + error.message, 'error');
        }
    }

    async disconnectWallet() {
        try {
            if (this.wallet) {
                await this.wallet.disconnect();
            }
            this.wallet = null;
            this.isConnected = false;
            
            this.showNotification('Wallet disconnected', 'info');
            this.updateUI();
            
        } catch (error) {
            console.error('Failed to disconnect wallet:', error);
        }
    }

    updateUI() {
        const connectButton = document.getElementById('connectWallet');
        const walletInfo = document.getElementById('walletInfo');
        const swapButton = document.getElementById('swapButton');
        const liquidityButton = document.getElementById('addLiquidityButton');
        
        if (this.isConnected && this.wallet) {
            connectButton.classList.add('hidden');
            walletInfo.classList.remove('hidden');
            
            // Show shortened wallet address
            const address = this.wallet.publicKey.toString();
            const shortAddress = address.slice(0, 4) + '...' + address.slice(-4);
            document.getElementById('walletAddress').textContent = shortAddress;
            
            swapButton.textContent = 'Swap Tokens';
            swapButton.disabled = false;
            
            liquidityButton.textContent = 'Add Liquidity';
            liquidityButton.disabled = false;
            
        } else {
            connectButton.classList.remove('hidden');
            walletInfo.classList.add('hidden');
            
            swapButton.textContent = 'Connect wallet to swap';
            swapButton.disabled = true;
            
            liquidityButton.textContent = 'Connect wallet to add liquidity';
            liquidityButton.disabled = true;
        }
    }

    async loadBalances() {
        if (!this.isConnected || !this.wallet) return;

        try {
            // Load SOL balance
            const balance = await this.connection.getBalance(this.wallet.publicKey);
            const solBalance = (balance / solanaWeb3.LAMPORTS_PER_SOL).toFixed(4);
            
            // Update balance displays (simplified for demo)
            document.getElementById('fromBalance').textContent = solBalance + ' SOL';
            document.getElementById('toBalance').textContent = '0.0 USDC';
            
        } catch (error) {
            console.error('Failed to load balances:', error);
        }
    }

    swapTokenSelection() {
        const fromToken = document.getElementById('fromToken');
        const toToken = document.getElementById('toToken');
        const fromAmount = document.getElementById('fromAmount');
        const toAmount = document.getElementById('toAmount');
        
        // Swap the selections
        const fromValue = fromToken.value;
        const toValue = toToken.value;
        
        fromToken.value = toValue;
        toToken.value = fromValue;
        
        // Clear amounts
        fromAmount.value = '';
        toAmount.value = '';
        
        this.updateExchangeRate();
    }

    calculateSwapOutput(inputAmount) {
        if (!inputAmount || inputAmount <= 0) {
            document.getElementById('toAmount').value = '';
            return;
        }

        const fromToken = document.getElementById('fromToken').value;
        const toToken = document.getElementById('toToken').value;
        
        // Simplified price calculation for educational purposes
        let exchangeRate = this.getExchangeRate(fromToken, toToken);
        let outputAmount = parseFloat(inputAmount) * exchangeRate;
        
        // Apply slippage and fees
        const slippage = parseFloat(document.getElementById('slippageSlider').value) / 100;
        const fee = 0.003; // 0.3% fee
        
        outputAmount = outputAmount * (1 - fee) * (1 - slippage);
        
        document.getElementById('toAmount').value = outputAmount.toFixed(6);
        this.updateExchangeRate();
    }

    getExchangeRate(fromToken, toToken) {
        // Simplified exchange rates for educational purposes
        const rates = {
            'SOL_USDC': 20.5,
            'USDC_SOL': 1/20.5,
            'TOKEN_A_TOKEN_B': 1.5,
            'TOKEN_B_TOKEN_A': 1/1.5,
            'SOL_TOKEN_A': 10.25,
            'TOKEN_A_SOL': 1/10.25,
            'USDC_TOKEN_B': 0.5,
            'TOKEN_B_USDC': 2.0
        };
        
        const key = `${fromToken}_${toToken}`;
        return rates[key] || 1;
    }

    updateExchangeRate() {
        const fromToken = document.getElementById('fromToken').value;
        const toToken = document.getElementById('toToken').value;
        const rate = this.getExchangeRate(fromToken, toToken);
        
        document.getElementById('exchangeRate').textContent = 
            `1 ${fromToken} = ${rate.toFixed(4)} ${toToken}`;
    }

    async performSwap() {
        if (!this.isConnected) {
            this.showNotification('Please connect your wallet first', 'warning');
            return;
        }

        const fromAmount = document.getElementById('fromAmount').value;
        const toAmount = document.getElementById('toAmount').value;
        
        if (!fromAmount || fromAmount <= 0) {
            this.showNotification('Please enter a valid amount to swap', 'warning');
            return;
        }

        this.showLoading(true);
        
        try {
            // In a real implementation, this would create and send a transaction
            // For educational purposes, we'll simulate the swap
            await this.simulateTransaction('swap');
            
            this.showNotification(
                `Successfully swapped ${fromAmount} tokens! (Simulated)`, 
                'success'
            );
            
            // Clear the form
            document.getElementById('fromAmount').value = '';
            document.getElementById('toAmount').value = '';
            
            // Refresh balances
            await this.loadBalances();
            
        } catch (error) {
            console.error('Swap failed:', error);
            this.showNotification('Swap failed: ' + error.message, 'error');
        } finally {
            this.showLoading(false);
        }
    }

    calculateLiquidityB() {
        const amountA = parseFloat(document.getElementById('liquidityAmountA').value);
        if (!amountA || amountA <= 0) {
            document.getElementById('liquidityAmountB').value = '';
            return;
        }
        
        // Simplified ratio calculation (1:1.5 ratio for demo)
        const amountB = amountA * 1.5;
        document.getElementById('liquidityAmountB').value = amountB.toFixed(6);
        
        // Calculate pool share (simplified)
        const poolShare = (amountA / (amountA + 1000)) * 100; // Assuming 1000 total liquidity
        document.getElementById('poolShare').textContent = poolShare.toFixed(2) + '%';
        
        const lpTokens = Math.sqrt(amountA * amountB);
        document.getElementById('lpTokens').textContent = lpTokens.toFixed(6);
    }

    async addLiquidity() {
        if (!this.isConnected) {
            this.showNotification('Please connect your wallet first', 'warning');
            return;
        }

        const amountA = document.getElementById('liquidityAmountA').value;
        const amountB = document.getElementById('liquidityAmountB').value;
        
        if (!amountA || !amountB || amountA <= 0 || amountB <= 0) {
            this.showNotification('Please enter valid amounts for both tokens', 'warning');
            return;
        }

        this.showLoading(true);
        
        try {
            // Simulate adding liquidity
            await this.simulateTransaction('add_liquidity');
            
            this.showNotification(
                `Successfully added liquidity: ${amountA} TOKEN A + ${amountB} TOKEN B! (Simulated)`, 
                'success'
            );
            
            // Clear the form
            document.getElementById('liquidityAmountA').value = '';
            document.getElementById('liquidityAmountB').value = '';
            document.getElementById('poolShare').textContent = '0%';
            document.getElementById('lpTokens').textContent = '0';
            
            // Update stats (simulated)
            this.updatePoolStats();
            
        } catch (error) {
            console.error('Add liquidity failed:', error);
            this.showNotification('Add liquidity failed: ' + error.message, 'error');
        } finally {
            this.showLoading(false);
        }
    }

    async simulateTransaction(type) {
        // Simulate network delay
        await new Promise(resolve => setTimeout(resolve, 2000 + Math.random() * 2000));
        
        // Simulate occasional failures for educational purposes
        if (Math.random() < 0.1) {
            throw new Error('Transaction failed (simulated network error)');
        }
        
        return {
            signature: 'simulated_' + Math.random().toString(36).substr(2, 9)
        };
    }

    updatePoolStats() {
        // Simulate updating pool statistics
        const tvl = Math.floor(Math.random() * 1000000) + 500000;
        const volume24h = Math.floor(Math.random() * 100000) + 50000;
        const totalLPs = Math.floor(Math.random() * 1000) + 100;
        const apr = (Math.random() * 50 + 10).toFixed(1);
        
        document.getElementById('tvl').textContent = '$' + tvl.toLocaleString();
        document.getElementById('volume24h').textContent = '$' + volume24h.toLocaleString();
        document.getElementById('totalLPs').textContent = totalLPs.toLocaleString();
        document.getElementById('currentAPR').textContent = apr + '%';
    }

    showLoading(show) {
        const overlay = document.getElementById('loadingOverlay');
        if (show) {
            overlay.classList.remove('hidden');
        } else {
            overlay.classList.add('hidden');
        }
    }

    showNotification(message, type = 'info') {
        // Create notification element
        const notification = document.createElement('div');
        notification.className = `notification notification-${type}`;
        notification.innerHTML = `
            <div class="notification-content">
                <span class="notification-icon">${this.getNotificationIcon(type)}</span>
                <span class="notification-message">${message}</span>
                <button class="notification-close" onclick="this.parentElement.parentElement.remove()">×</button>
            </div>
        `;
        
        // Add styles if not already added
        if (!document.getElementById('notification-styles')) {
            const styles = document.createElement('style');
            styles.id = 'notification-styles';
            styles.textContent = `
                .notification {
                    position: fixed;
                    top: 20px;
                    right: 20px;
                    z-index: 1000;
                    min-width: 300px;
                    max-width: 500px;
                    padding: 16px;
                    border-radius: 12px;
                    box-shadow: 0 10px 25px rgba(0,0,0,0.1);
                    animation: slideIn 0.3s ease;
                }
                .notification-success { background: #d4edda; border-left: 4px solid #28a745; color: #155724; }
                .notification-error { background: #f8d7da; border-left: 4px solid #dc3545; color: #721c24; }
                .notification-warning { background: #fff3cd; border-left: 4px solid #ffc107; color: #856404; }
                .notification-info { background: #d1ecf1; border-left: 4px solid #17a2b8; color: #0c5460; }
                .notification-content { display: flex; align-items: center; gap: 10px; }
                .notification-close { background: none; border: none; font-size: 20px; cursor: pointer; margin-left: auto; }
                @keyframes slideIn { from { transform: translateX(100%); opacity: 0; } to { transform: translateX(0); opacity: 1; } }
            `;
            document.head.appendChild(styles);
        }
        
        document.body.appendChild(notification);
        
        // Auto remove after 5 seconds
        setTimeout(() => {
            if (notification.parentElement) {
                notification.remove();
            }
        }, 5000);
    }

    getNotificationIcon(type) {
        const icons = {
            success: '✅',
            error: '❌',
            warning: '⚠️',
            info: 'ℹ️'
        };
        return icons[type] || 'ℹ️';
    }
}

// Tutorial function
function showTutorial() {
    const tutorialSteps = [
        {
            title: "Welcome to Solana DEX!",
            content: "This educational DEX will teach you DeFi concepts through hands-on experience."
        },
        {
            title: "Connect Your Wallet",
            content: "First, install a Solana wallet like Phantom and connect it to start trading."
        },
        {
            title: "Understanding Swaps",
            content: "Swaps exchange one token for another using automated market maker (AMM) algorithms."
        },
        {
            title: "Adding Liquidity",
            content: "Provide token pairs to earn fees from trades. You'll receive LP tokens representing your share."
        },
        {
            title: "Pool Statistics",
            content: "Monitor TVL, volume, and APR to understand pool performance and yields."
        }
    ];
    
    let currentStep = 0;
    
    function showStep(step) {
        alert(`${tutorialSteps[step].title}\n\n${tutorialSteps[step].content}\n\nStep ${step + 1} of ${tutorialSteps.length}`);
        currentStep++;
        if (currentStep < tutorialSteps.length) {
            setTimeout(() => showStep(currentStep), 100);
        }
    }
    
    showStep(0);
}

// Initialize the app when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    window.dexApp = new SolanaDexApp();
    
    // Update pool stats on load
    window.dexApp.updatePoolStats();
    
    // Update exchange rate on load
    window.dexApp.updateExchangeRate();
});