import React, { useState, useEffect } from 'react';

interface TokenInfo {
  symbol: string;
  name: string;
  icon: string;
  balance: number;
  price: number;
}

// Mock token data for educational purposes
const MOCK_TOKENS: TokenInfo[] = [
  { symbol: 'SOL', name: 'Solana', icon: '◎', balance: 10.5, price: 200 },
  { symbol: 'USDC', name: 'USD Coin', icon: '💵', balance: 1000, price: 1 },
  { symbol: 'EDU', name: 'Education Token', icon: '🎓', balance: 500, price: 2.5 },
  { symbol: 'LEARN', name: 'Learn Token', icon: '📚', balance: 250, price: 5 },
];

export const SwapInterface: React.FC = () => {
  const [fromToken, setFromToken] = useState<TokenInfo>(MOCK_TOKENS[0]);
  const [toToken, setToToken] = useState<TokenInfo>(MOCK_TOKENS[1]);
  const [fromAmount, setFromAmount] = useState<string>('');
  const [toAmount, setToAmount] = useState<string>('');
  const [slippage, setSlippage] = useState<number>(0.5);
  const [showAdvanced, setShowAdvanced] = useState<boolean>(false);

  // Calculate output amount using constant product formula
  // This is a simplified simulation for educational purposes
  const calculateSwapOutput = (inputAmount: number, inputPrice: number, outputPrice: number): number => {
    // Simulate AMM with constant product formula
    // In reality, this would use pool reserves: output = (reserveOut * input) / (reserveIn + input)
    const baseOutput = (inputAmount * inputPrice) / outputPrice;
    const slippageImpact = Math.min(inputAmount / 100, 0.05); // Max 5% slippage
    const outputWithSlippage = baseOutput * (1 - slippageImpact);
    return outputWithSlippage;
  };

  // Update output amount when input changes
  useEffect(() => {
    if (fromAmount && !isNaN(parseFloat(fromAmount))) {
      const inputAmount = parseFloat(fromAmount);
      const output = calculateSwapOutput(inputAmount, fromToken.price, toToken.price);
      setToAmount(output.toFixed(6));
    } else {
      setToAmount('');
    }
  }, [fromAmount, fromToken, toToken]);

  const handleSwapTokens = () => {
    setFromToken(toToken);
    setToToken(fromToken);
    setFromAmount(toAmount);
  };

  const handleSwap = () => {
    alert('🎓 Educational Note: In a real DEX, this would submit a transaction to the Solana blockchain!');
  };

  const priceImpact = fromAmount ? Math.min(parseFloat(fromAmount) / 100, 5) : 0;
  
  return (
    <div className="max-w-md mx-auto">
      <div className="mb-6">
        <h2 className="text-xl font-bold text-gray-900 mb-2">Swap Tokens</h2>
        <p className="text-sm text-gray-600">
          Exchange one token for another using our automated market maker (AMM).
        </p>
      </div>

      {/* From Token Input */}
      <div className="bg-gray-50 rounded-lg p-4 mb-2">
        <div className="flex justify-between items-center mb-2">
          <label className="text-sm font-medium text-gray-700">From</label>
          <span className="text-xs text-gray-500">
            Balance: {fromToken.balance} {fromToken.symbol}
          </span>
        </div>
        
        <div className="flex items-center space-x-3">
          <select 
            value={fromToken.symbol}
            onChange={(e) => setFromToken(MOCK_TOKENS.find(t => t.symbol === e.target.value) || MOCK_TOKENS[0])}
            className="flex-shrink-0 bg-white border border-gray-300 rounded-lg px-3 py-2 text-sm"
          >
            {MOCK_TOKENS.map(token => (
              <option key={token.symbol} value={token.symbol}>
                {token.icon} {token.symbol}
              </option>
            ))}
          </select>
          
          <input
            type="number"
            value={fromAmount}
            onChange={(e) => setFromAmount(e.target.value)}
            placeholder="0.0"
            className="flex-1 bg-transparent text-xl font-semibold outline-none"
          />
        </div>
        
        <div className="text-xs text-gray-500 mt-1">
          ≈ ${fromAmount ? (parseFloat(fromAmount) * fromToken.price).toFixed(2) : '0.00'}
        </div>
      </div>

      {/* Swap Button */}
      <div className="flex justify-center my-2">
        <button
          onClick={handleSwapTokens}
          className="p-2 bg-gray-100 hover:bg-gray-200 rounded-full transition-colors"
        >
          🔄
        </button>
      </div>

      {/* To Token Output */}
      <div className="bg-gray-50 rounded-lg p-4 mb-4">
        <div className="flex justify-between items-center mb-2">
          <label className="text-sm font-medium text-gray-700">To</label>
          <span className="text-xs text-gray-500">
            Balance: {toToken.balance} {toToken.symbol}
          </span>
        </div>
        
        <div className="flex items-center space-x-3">
          <select 
            value={toToken.symbol}
            onChange={(e) => setToToken(MOCK_TOKENS.find(t => t.symbol === e.target.value) || MOCK_TOKENS[1])}
            className="flex-shrink-0 bg-white border border-gray-300 rounded-lg px-3 py-2 text-sm"
          >
            {MOCK_TOKENS.filter(t => t.symbol !== fromToken.symbol).map(token => (
              <option key={token.symbol} value={token.symbol}>
                {token.icon} {token.symbol}
              </option>
            ))}
          </select>
          
          <input
            type="text"
            value={toAmount}
            readOnly
            placeholder="0.0"
            className="flex-1 bg-transparent text-xl font-semibold outline-none text-gray-600"
          />
        </div>
        
        <div className="text-xs text-gray-500 mt-1">
          ≈ ${toAmount ? (parseFloat(toAmount) * toToken.price).toFixed(2) : '0.00'}
        </div>
      </div>

      {/* Advanced Settings */}
      <div className="mb-4">
        <button
          onClick={() => setShowAdvanced(!showAdvanced)}
          className="text-sm text-purple-600 hover:text-purple-700 flex items-center space-x-1"
        >
          <span>Advanced Settings</span>
          <span className={`transition-transform ${showAdvanced ? 'rotate-180' : ''}`}>▼</span>
        </button>
        
        {showAdvanced && (
          <div className="mt-3 p-3 bg-gray-50 rounded-lg">
            <div className="flex justify-between items-center">
              <label className="text-sm font-medium text-gray-700">Slippage Tolerance</label>
              <div className="flex items-center space-x-2">
                <input
                  type="number"
                  value={slippage}
                  onChange={(e) => setSlippage(parseFloat(e.target.value))}
                  step="0.1"
                  min="0.1"
                  max="50"
                  className="w-16 px-2 py-1 text-sm border border-gray-300 rounded"
                />
                <span className="text-sm text-gray-500">%</span>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Transaction Info */}
      {fromAmount && toAmount && (
        <div className="bg-blue-50 border border-blue-200 rounded-lg p-3 mb-4 text-sm">
          <div className="flex justify-between mb-1">
            <span className="text-gray-600">Price Impact:</span>
            <span className={`font-medium ${priceImpact > 2 ? 'text-red-600' : 'text-green-600'}`}>
              {priceImpact.toFixed(2)}%
            </span>
          </div>
          <div className="flex justify-between mb-1">
            <span className="text-gray-600">Minimum Received:</span>
            <span className="font-medium">
              {(parseFloat(toAmount) * (1 - slippage / 100)).toFixed(6)} {toToken.symbol}
            </span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-600">Network Fee:</span>
            <span className="font-medium">~0.00025 SOL</span>
          </div>
        </div>
      )}

      {/* Swap Button */}
      <button
        onClick={handleSwap}
        disabled={!fromAmount || !toAmount || parseFloat(fromAmount) > fromToken.balance}
        className="w-full bg-purple-600 hover:bg-purple-700 disabled:bg-gray-300 disabled:cursor-not-allowed text-white font-semibold py-3 px-4 rounded-lg transition-colors"
      >
        {!fromAmount || !toAmount 
          ? 'Enter an amount' 
          : parseFloat(fromAmount) > fromToken.balance 
            ? 'Insufficient balance' 
            : 'Swap Tokens'}
      </button>

      {/* Educational Note */}
      <div className="mt-4 p-3 bg-yellow-50 border border-yellow-200 rounded-lg">
        <div className="text-sm">
          <div className="font-semibold text-yellow-800 mb-1">📚 Learning Note:</div>
          <p className="text-yellow-700">
            This swap uses the constant product formula (x × y = k). Notice how larger swaps have higher price impact. 
            The actual implementation would interact with our Solana program!
          </p>
        </div>
      </div>
    </div>
  );
};