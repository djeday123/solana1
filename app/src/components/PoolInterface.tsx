import React, { useState } from 'react';

interface Pool {
  id: string;
  tokenA: { symbol: string; icon: string; amount: number };
  tokenB: { symbol: string; icon: string; amount: number };
  totalLiquidity: number;
  yourShare: number;
  apr: number;
  volume24h: number;
}

// Mock pool data for educational purposes
const MOCK_POOLS: Pool[] = [
  {
    id: 'SOL-USDC',
    tokenA: { symbol: 'SOL', icon: '◎', amount: 1000 },
    tokenB: { symbol: 'USDC', icon: '💵', amount: 200000 },
    totalLiquidity: 400000,
    yourShare: 2500,
    apr: 12.5,
    volume24h: 50000,
  },
  {
    id: 'EDU-LEARN',
    tokenA: { symbol: 'EDU', icon: '🎓', amount: 5000 },
    tokenB: { symbol: 'LEARN', icon: '📚', amount: 2000 },
    totalLiquidity: 25000,
    yourShare: 500,
    apr: 25.0,
    volume24h: 8000,
  },
];

type PoolAction = 'add' | 'remove' | 'create';

export const PoolInterface: React.FC = () => {
  const [activeAction, setActiveAction] = useState<PoolAction>('add');
  const [selectedPool, setSelectedPool] = useState<Pool>(MOCK_POOLS[0]);
  const [amountA, setAmountA] = useState<string>('');
  const [amountB, setAmountB] = useState<string>('');
  const [lpTokensToRemove, setLpTokensToRemove] = useState<string>('');

  // Calculate proportional amounts for adding liquidity
  const handleAmountAChange = (value: string) => {
    setAmountA(value);
    if (value && !isNaN(parseFloat(value))) {
      const ratio = selectedPool.tokenB.amount / selectedPool.tokenA.amount;
      setAmountB((parseFloat(value) * ratio).toFixed(6));
    } else {
      setAmountB('');
    }
  };

  const handleAmountBChange = (value: string) => {
    setAmountB(value);
    if (value && !isNaN(parseFloat(value))) {
      const ratio = selectedPool.tokenA.amount / selectedPool.tokenB.amount;
      setAmountA((parseFloat(value) * ratio).toFixed(6));
    } else {
      setAmountA('');
    }
  };

  const calculateLPTokens = (): number => {
    if (!amountA || !amountB) return 0;
    // Simplified LP token calculation: geometric mean
    return Math.sqrt(parseFloat(amountA) * parseFloat(amountB));
  };

  const handleAction = () => {
    const action = activeAction === 'add' ? 'add liquidity to' : 
                  activeAction === 'remove' ? 'remove liquidity from' : 'create';
    alert(`🎓 Educational Note: In a real DEX, this would ${action} the ${selectedPool.id} pool on Solana!`);
  };

  return (
    <div className="max-w-2xl mx-auto">
      <div className="mb-6">
        <h2 className="text-xl font-bold text-gray-900 mb-2">Liquidity Pools</h2>
        <p className="text-sm text-gray-600">
          Provide liquidity to earn trading fees or remove your existing liquidity positions.
        </p>
      </div>

      {/* Action Tabs */}
      <div className="flex bg-gray-100 rounded-lg p-1 mb-6">
        {[
          { id: 'add', name: 'Add Liquidity', icon: '➕' },
          { id: 'remove', name: 'Remove Liquidity', icon: '➖' },
          { id: 'create', name: 'Create Pool', icon: '🆕' },
        ].map((action) => (
          <button
            key={action.id}
            onClick={() => setActiveAction(action.id as PoolAction)}
            className={`flex-1 flex items-center justify-center space-x-2 py-2 px-4 rounded-md text-sm font-medium transition-colors ${
              activeAction === action.id
                ? 'bg-white text-purple-600 shadow-sm'
                : 'text-gray-600 hover:text-gray-900'
            }`}
          >
            <span>{action.icon}</span>
            <span className="hidden sm:block">{action.name}</span>
          </button>
        ))}
      </div>

      {/* Pool Selection */}
      <div className="bg-gray-50 rounded-lg p-4 mb-6">
        <h3 className="font-semibold text-gray-900 mb-3">Select Pool</h3>
        <div className="grid gap-3">
          {MOCK_POOLS.map((pool) => (
            <div
              key={pool.id}
              onClick={() => setSelectedPool(pool)}
              className={`p-4 rounded-lg border-2 cursor-pointer transition-colors ${
                selectedPool.id === pool.id
                  ? 'border-purple-500 bg-purple-50'
                  : 'border-gray-200 bg-white hover:border-gray-300'
              }`}
            >
              <div className="flex justify-between items-start mb-2">
                <div className="flex items-center space-x-2">
                  <span className="text-lg">{pool.tokenA.icon}</span>
                  <span className="text-lg">{pool.tokenB.icon}</span>
                  <span className="font-semibold">{pool.id}</span>
                </div>
                <div className="text-right">
                  <div className="text-sm font-medium text-green-600">{pool.apr}% APR</div>
                  <div className="text-xs text-gray-500">24h Volume: ${pool.volume24h.toLocaleString()}</div>
                </div>
              </div>
              
              <div className="grid grid-cols-2 gap-4 text-sm">
                <div>
                  <div className="text-gray-600">Pool Liquidity</div>
                  <div className="font-medium">${pool.totalLiquidity.toLocaleString()}</div>
                </div>
                <div>
                  <div className="text-gray-600">Your Position</div>
                  <div className="font-medium">${pool.yourShare.toLocaleString()}</div>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Action Interface */}
      {activeAction === 'add' && (
        <div className="space-y-4">
          <h3 className="font-semibold text-gray-900">Add Liquidity to {selectedPool.id}</h3>
          
          {/* Token A Input */}
          <div className="bg-gray-50 rounded-lg p-4">
            <div className="flex justify-between items-center mb-2">
              <label className="text-sm font-medium text-gray-700">
                {selectedPool.tokenA.icon} {selectedPool.tokenA.symbol}
              </label>
              <span className="text-xs text-gray-500">Balance: 10.5</span>
            </div>
            <input
              type="number"
              value={amountA}
              onChange={(e) => handleAmountAChange(e.target.value)}
              placeholder="0.0"
              className="w-full bg-transparent text-xl font-semibold outline-none"
            />
          </div>

          {/* Token B Input */}
          <div className="bg-gray-50 rounded-lg p-4">
            <div className="flex justify-between items-center mb-2">
              <label className="text-sm font-medium text-gray-700">
                {selectedPool.tokenB.icon} {selectedPool.tokenB.symbol}
              </label>
              <span className="text-xs text-gray-500">Balance: 1000</span>
            </div>
            <input
              type="number"
              value={amountB}
              onChange={(e) => handleAmountBChange(e.target.value)}
              placeholder="0.0"
              className="w-full bg-transparent text-xl font-semibold outline-none"
            />
          </div>

          {/* LP Token Info */}
          {amountA && amountB && (
            <div className="bg-blue-50 border border-blue-200 rounded-lg p-3">
              <div className="text-sm">
                <div className="flex justify-between mb-1">
                  <span className="text-gray-600">LP Tokens to receive:</span>
                  <span className="font-medium">{calculateLPTokens().toFixed(6)}</span>
                </div>
                <div className="flex justify-between mb-1">
                  <span className="text-gray-600">Pool share:</span>
                  <span className="font-medium">0.05%</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Price ratio:</span>
                  <span className="font-medium">
                    1 {selectedPool.tokenA.symbol} = {(selectedPool.tokenB.amount / selectedPool.tokenA.amount).toFixed(2)} {selectedPool.tokenB.symbol}
                  </span>
                </div>
              </div>
            </div>
          )}
        </div>
      )}

      {activeAction === 'remove' && (
        <div className="space-y-4">
          <h3 className="font-semibold text-gray-900">Remove Liquidity from {selectedPool.id}</h3>
          
          <div className="bg-gray-50 rounded-lg p-4">
            <div className="flex justify-between items-center mb-2">
              <label className="text-sm font-medium text-gray-700">LP Tokens to Remove</label>
              <span className="text-xs text-gray-500">Your LP Balance: 12.5</span>
            </div>
            <input
              type="number"
              value={lpTokensToRemove}
              onChange={(e) => setLpTokensToRemove(e.target.value)}
              placeholder="0.0"
              className="w-full bg-transparent text-xl font-semibold outline-none"
            />
          </div>

          {/* Percentage Buttons */}
          <div className="flex space-x-2">
            {['25%', '50%', '75%', '100%'].map((percentage) => (
              <button
                key={percentage}
                onClick={() => setLpTokensToRemove((12.5 * parseInt(percentage) / 100).toString())}
                className="flex-1 py-2 px-3 bg-gray-100 hover:bg-gray-200 rounded-lg text-sm font-medium transition-colors"
              >
                {percentage}
              </button>
            ))}
          </div>

          {/* Withdrawal Preview */}
          {lpTokensToRemove && (
            <div className="bg-blue-50 border border-blue-200 rounded-lg p-3">
              <div className="text-sm">
                <div className="font-medium text-blue-900 mb-2">You will receive:</div>
                <div className="flex justify-between mb-1">
                  <span className="text-gray-600">{selectedPool.tokenA.icon} {selectedPool.tokenA.symbol}:</span>
                  <span className="font-medium">{(parseFloat(lpTokensToRemove) * 0.8).toFixed(6)}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">{selectedPool.tokenB.icon} {selectedPool.tokenB.symbol}:</span>
                  <span className="font-medium">{(parseFloat(lpTokensToRemove) * 160).toFixed(6)}</span>
                </div>
              </div>
            </div>
          )}
        </div>
      )}

      {activeAction === 'create' && (
        <div className="space-y-4">
          <h3 className="font-semibold text-gray-900">Create New Pool</h3>
          <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
            <div className="text-sm">
              <div className="font-semibold text-yellow-800 mb-2">⚠️ Important:</div>
              <p className="text-yellow-700">
                Creating a new pool means you set the initial price ratio. Make sure you research 
                the fair market price of both tokens before proceeding.
              </p>
            </div>
          </div>
          
          <div className="text-center py-8">
            <div className="text-4xl mb-4">🚧</div>
            <p className="text-gray-600">Pool creation interface coming soon...</p>
          </div>
        </div>
      )}

      {/* Action Button */}
      <button
        onClick={handleAction}
        disabled={
          (activeAction === 'add' && (!amountA || !amountB)) ||
          (activeAction === 'remove' && !lpTokensToRemove) ||
          activeAction === 'create'
        }
        className="w-full bg-purple-600 hover:bg-purple-700 disabled:bg-gray-300 disabled:cursor-not-allowed text-white font-semibold py-3 px-4 rounded-lg transition-colors mt-6"
      >
        {activeAction === 'add' ? 'Add Liquidity' : 
         activeAction === 'remove' ? 'Remove Liquidity' : 
         'Create Pool (Coming Soon)'}
      </button>

      {/* Educational Note */}
      <div className="mt-6 p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
        <div className="text-sm">
          <div className="font-semibold text-yellow-800 mb-2">📚 Learning Notes:</div>
          <ul className="text-yellow-700 space-y-1">
            <li>• <strong>Adding Liquidity:</strong> You must provide tokens in the current pool ratio to avoid impermanent loss</li>
            <li>• <strong>LP Tokens:</strong> Represent your share of the pool and your claim to trading fees</li>
            <li>• <strong>Impermanent Loss:</strong> Potential loss when token prices diverge from when you deposited</li>
            <li>• <strong>APR:</strong> Annual percentage return from trading fees earned by the pool</li>
          </ul>
        </div>
      </div>
    </div>
  );
};