import React, { useState } from 'react';

type Topic = 'amm' | 'liquidity' | 'impermanent-loss' | 'arbitrage' | 'solana';

interface Lesson {
  id: Topic;
  title: string;
  icon: string;
  description: string;
  difficulty: 'Beginner' | 'Intermediate' | 'Advanced';
}

const LESSONS: Lesson[] = [
  {
    id: 'amm',
    title: 'Automated Market Makers',
    icon: '🤖',
    description: 'Learn how AMMs work and the constant product formula',
    difficulty: 'Beginner',
  },
  {
    id: 'liquidity',
    title: 'Liquidity Provision',
    icon: '💧',
    description: 'Understand how to provide liquidity and earn fees',
    difficulty: 'Beginner',
  },
  {
    id: 'impermanent-loss',
    title: 'Impermanent Loss',
    icon: '📉',
    description: 'Learn about the risks of providing liquidity',
    difficulty: 'Intermediate',
  },
  {
    id: 'arbitrage',
    title: 'Arbitrage Trading',
    icon: '⚖️',
    description: 'Discover how price differences create trading opportunities',
    difficulty: 'Advanced',
  },
  {
    id: 'solana',
    title: 'Solana & Performance',
    icon: '⚡',
    description: 'Why Solana is ideal for high-frequency DeFi',
    difficulty: 'Intermediate',
  },
];

export const EducationalPanel: React.FC = () => {
  const [selectedTopic, setSelectedTopic] = useState<Topic>('amm');

  const renderContent = () => {
    switch (selectedTopic) {
      case 'amm':
        return (
          <div className="space-y-6">
            <h2 className="text-2xl font-bold text-gray-900">🤖 Automated Market Makers (AMMs)</h2>
            
            <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
              <h3 className="font-semibold text-blue-900 mb-2">What is an AMM?</h3>
              <p className="text-blue-800">
                An Automated Market Maker is a smart contract that holds reserves of two or more tokens 
                and allows users to trade between them using a mathematical formula instead of traditional order books.
              </p>
            </div>

            <div className="space-y-4">
              <h3 className="text-lg font-semibold">The Constant Product Formula</h3>
              <div className="bg-gray-100 p-4 rounded-lg font-mono text-center text-lg">
                x × y = k
              </div>
              <p className="text-gray-700">
                Where <code>x</code> and <code>y</code> are the reserves of two tokens, and <code>k</code> is a constant.
                When someone makes a trade, the product must remain constant (ignoring fees).
              </p>
            </div>

            <div className="grid md:grid-cols-2 gap-4">
              <div className="bg-green-50 border border-green-200 rounded-lg p-4">
                <h4 className="font-semibold text-green-900 mb-2">✅ Advantages</h4>
                <ul className="text-green-800 text-sm space-y-1">
                  <li>• Always liquid (no empty order books)</li>
                  <li>• Automatic price discovery</li>
                  <li>• Passive market making</li>
                  <li>• Composable with other DeFi protocols</li>
                </ul>
              </div>
              
              <div className="bg-red-50 border border-red-200 rounded-lg p-4">
                <h4 className="font-semibold text-red-900 mb-2">⚠️ Considerations</h4>
                <ul className="text-red-800 text-sm space-y-1">
                  <li>• Price impact on large trades</li>
                  <li>• Impermanent loss for LPs</li>
                  <li>• Slippage from formula curve</li>
                  <li>• MEV (front-running) risks</li>
                </ul>
              </div>
            </div>

            <div className="bg-purple-50 border border-purple-200 rounded-lg p-4">
              <h4 className="font-semibold text-purple-900 mb-2">🧮 Example Calculation</h4>
              <div className="text-purple-800 space-y-2">
                <p><strong>Initial State:</strong> 1000 USDC × 10 SOL = 10,000 (k)</p>
                <p><strong>Trade:</strong> User swaps 100 USDC for SOL</p>
                <p><strong>New USDC:</strong> 1000 + 100 = 1100</p>
                <p><strong>New SOL:</strong> 10,000 ÷ 1100 = 9.09 SOL</p>
                <p><strong>User Receives:</strong> 10 - 9.09 = 0.91 SOL</p>
                <p><strong>Price Impact:</strong> ~9% (higher for larger trades)</p>
              </div>
            </div>
          </div>
        );

      case 'liquidity':
        return (
          <div className="space-y-6">
            <h2 className="text-2xl font-bold text-gray-900">💧 Liquidity Provision</h2>
            
            <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
              <h3 className="font-semibold text-blue-900 mb-2">What is Liquidity Provision?</h3>
              <p className="text-blue-800">
                Liquidity providers (LPs) deposit equal values of two tokens into a pool to enable trading. 
                In return, they receive LP tokens representing their share and earn fees from all trades.
              </p>
            </div>

            <div className="grid md:grid-cols-2 gap-6">
              <div>
                <h3 className="text-lg font-semibold mb-3">How It Works</h3>
                <ol className="space-y-2 text-gray-700">
                  <li className="flex items-start space-x-2">
                    <span className="bg-purple-100 text-purple-800 w-6 h-6 rounded-full flex items-center justify-center text-sm font-bold">1</span>
                    <span>Deposit equal values of Token A and Token B</span>
                  </li>
                  <li className="flex items-start space-x-2">
                    <span className="bg-purple-100 text-purple-800 w-6 h-6 rounded-full flex items-center justify-center text-sm font-bold">2</span>
                    <span>Receive LP tokens representing your pool share</span>
                  </li>
                  <li className="flex items-start space-x-2">
                    <span className="bg-purple-100 text-purple-800 w-6 h-6 rounded-full flex items-center justify-center text-sm font-bold">3</span>
                    <span>Earn trading fees proportional to your share</span>
                  </li>
                  <li className="flex items-start space-x-2">
                    <span className="bg-purple-100 text-purple-800 w-6 h-6 rounded-full flex items-center justify-center text-sm font-bold">4</span>
                    <span>Redeem LP tokens for underlying assets + fees</span>
                  </li>
                </ol>
              </div>
              
              <div>
                <h3 className="text-lg font-semibold mb-3">LP Token Calculation</h3>
                <div className="bg-gray-100 p-4 rounded-lg space-y-2">
                  <div className="font-mono text-sm">
                    <div>Initial LP = √(amount_a × amount_b)</div>
                    <div>Additional LP = min(</div>
                    <div className="ml-4">amount_a / reserve_a,</div>
                    <div className="ml-4">amount_b / reserve_b</div>
                    <div>) × total_lp_supply</div>
                  </div>
                </div>
              </div>
            </div>

            <div className="bg-green-50 border border-green-200 rounded-lg p-4">
              <h4 className="font-semibold text-green-900 mb-2">💰 Earning Potential</h4>
              <div className="grid md:grid-cols-3 gap-4 text-green-800">
                <div>
                  <div className="font-semibold">Trading Fees</div>
                  <div className="text-sm">Typically 0.3% per trade</div>
                </div>
                <div>
                  <div className="font-semibold">Liquidity Rewards</div>
                  <div className="text-sm">Additional token incentives</div>
                </div>
                <div>
                  <div className="font-semibold">Yield Farming</div>
                  <div className="text-sm">Stake LP tokens for more rewards</div>
                </div>
              </div>
            </div>
          </div>
        );

      case 'impermanent-loss':
        return (
          <div className="space-y-6">
            <h2 className="text-2xl font-bold text-gray-900">📉 Impermanent Loss</h2>
            
            <div className="bg-red-50 border border-red-200 rounded-lg p-4">
              <h3 className="font-semibold text-red-900 mb-2">What is Impermanent Loss?</h3>
              <p className="text-red-800">
                Impermanent loss occurs when the price ratio of tokens in a liquidity pool changes compared 
                to when you deposited them. You end up with less value than if you had just held the tokens.
              </p>
            </div>

            <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
              <h4 className="font-semibold text-yellow-900 mb-2">📊 Example Scenario</h4>
              <div className="text-yellow-800 space-y-2">
                <p><strong>Initial Deposit:</strong> 1 ETH ($2000) + 2000 USDC = $4000 total</p>
                <p><strong>ETH doubles to $4000:</strong></p>
                <ul className="ml-6 space-y-1">
                  <li>• If you held: 1 ETH ($4000) + 2000 USDC = $6000</li>
                  <li>• If you provided liquidity: ~0.707 ETH ($2828) + 2828 USDC = $5656</li>
                  <li>• <strong>Impermanent Loss: $344 (5.7%)</strong></li>
                </ul>
              </div>
            </div>

            <div className="grid md:grid-cols-2 gap-4">
              <div>
                <h4 className="font-semibold mb-3">Loss by Price Change</h4>
                <div className="space-y-2 text-sm">
                  <div className="flex justify-between">
                    <span>1.25x price change:</span>
                    <span className="text-red-600">-0.6%</span>
                  </div>
                  <div className="flex justify-between">
                    <span>1.5x price change:</span>
                    <span className="text-red-600">-2.0%</span>
                  </div>
                  <div className="flex justify-between">
                    <span>2x price change:</span>
                    <span className="text-red-600">-5.7%</span>
                  </div>
                  <div className="flex justify-between">
                    <span>5x price change:</span>
                    <span className="text-red-600">-25.5%</span>
                  </div>
                </div>
              </div>
              
              <div>
                <h4 className="font-semibold mb-3">Mitigation Strategies</h4>
                <ul className="space-y-1 text-sm text-gray-700">
                  <li>• Provide liquidity to stable pairs</li>
                  <li>• Choose high-fee earning pools</li>
                  <li>• Shorter time horizons</li>
                  <li>• Use correlated token pairs</li>
                  <li>• Monitor and exit if needed</li>
                </ul>
              </div>
            </div>

            <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
              <h4 className="font-semibold text-blue-900 mb-2">💡 Key Takeaways</h4>
              <ul className="text-blue-800 space-y-1">
                <li>• Loss is "impermanent" - only realized when you withdraw</li>
                <li>• Trading fees can offset impermanent loss over time</li>
                <li>• Higher volatility = higher potential loss</li>
                <li>• Consider this risk vs. potential fee earnings</li>
              </ul>
            </div>
          </div>
        );

      case 'arbitrage':
        return (
          <div className="space-y-6">
            <h2 className="text-2xl font-bold text-gray-900">⚖️ Arbitrage Trading</h2>
            
            <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
              <h3 className="font-semibold text-blue-900 mb-2">What is Arbitrage?</h3>
              <p className="text-blue-800">
                Arbitrage is the practice of taking advantage of price differences for the same asset 
                across different markets to make risk-free profits.
              </p>
            </div>

            <div className="grid md:grid-cols-2 gap-6">
              <div className="bg-green-50 border border-green-200 rounded-lg p-4">
                <h4 className="font-semibold text-green-900 mb-2">Simple Arbitrage</h4>
                <div className="text-green-800 text-sm space-y-2">
                  <p><strong>Example:</strong></p>
                  <p>• Pool A: 1 SOL = 200 USDC</p>
                  <p>• Pool B: 1 SOL = 205 USDC</p>
                  <p>• Profit: Buy from A, sell to B = 5 USDC</p>
                </div>
              </div>
              
              <div className="bg-purple-50 border border-purple-200 rounded-lg p-4">
                <h4 className="font-semibold text-purple-900 mb-2">Flash Loan Arbitrage</h4>
                <div className="text-purple-800 text-sm space-y-2">
                  <p><strong>Steps:</strong></p>
                  <ol className="space-y-1">
                    <li>1. Borrow tokens (flash loan)</li>
                    <li>2. Execute arbitrage trades</li>
                    <li>3. Repay loan + fees</li>
                    <li>4. Keep profit</li>
                  </ol>
                </div>
              </div>
            </div>

            <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
              <h4 className="font-semibold text-yellow-900 mb-2">🔍 MEV (Maximal Extractable Value)</h4>
              <p className="text-yellow-800 text-sm mb-2">
                MEV refers to the maximum value that can be extracted from block production beyond 
                standard block rewards and gas fees.
              </p>
              <div className="grid md:grid-cols-3 gap-4 text-yellow-800">
                <div>
                  <div className="font-semibold">Front-running</div>
                  <div className="text-xs">Seeing pending trades and getting ahead</div>
                </div>
                <div>
                  <div className="font-semibold">Sandwich Attacks</div>
                  <div className="text-xs">Surrounding user trades with your own</div>
                </div>
                <div>
                  <div className="font-semibold">Liquidations</div>
                  <div className="text-xs">Being first to liquidate positions</div>
                </div>
              </div>
            </div>

            <div className="bg-gray-50 rounded-lg p-4">
              <h4 className="font-semibold text-gray-900 mb-2">🛡️ Protecting Yourself</h4>
              <ul className="text-gray-700 space-y-1 text-sm">
                <li>• Use slippage protection on trades</li>
                <li>• Consider private mempools</li>
                <li>• Break large trades into smaller ones</li>
                <li>• Use limit orders when possible</li>
                <li>• Monitor for unusual price movements</li>
              </ul>
            </div>
          </div>
        );

      case 'solana':
        return (
          <div className="space-y-6">
            <h2 className="text-2xl font-bold text-gray-900">⚡ Solana & High-Performance DeFi</h2>
            
            <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
              <h3 className="font-semibold text-blue-900 mb-2">Why Solana for DeFi?</h3>
              <p className="text-blue-800">
                Solana's unique architecture enables high-throughput, low-cost DeFi applications 
                that can compete with traditional finance in terms of speed and cost.
              </p>
            </div>

            <div className="grid md:grid-cols-3 gap-4">
              <div className="bg-green-50 border border-green-200 rounded-lg p-4 text-center">
                <div className="text-2xl mb-2">🚀</div>
                <div className="font-semibold text-green-900">Speed</div>
                <div className="text-green-800 text-sm">65,000+ TPS</div>
              </div>
              
              <div className="bg-blue-50 border border-blue-200 rounded-lg p-4 text-center">
                <div className="text-2xl mb-2">💰</div>
                <div className="font-semibold text-blue-900">Cost</div>
                <div className="text-blue-800 text-sm">$0.00025 per tx</div>
              </div>
              
              <div className="bg-purple-50 border border-purple-200 rounded-lg p-4 text-center">
                <div className="text-2xl mb-2">⏱️</div>
                <div className="font-semibold text-purple-900">Finality</div>
                <div className="text-purple-800 text-sm">~400ms</div>
              </div>
            </div>

            <div className="space-y-4">
              <h3 className="text-lg font-semibold">Technical Innovations</h3>
              
              <div className="grid md:grid-cols-2 gap-4">
                <div className="border border-gray-200 rounded-lg p-4">
                  <h4 className="font-semibold mb-2">🕐 Proof of History</h4>
                  <p className="text-gray-700 text-sm">
                    Creates a historical record that proves events occurred at specific moments, 
                    enabling faster consensus without waiting for network-wide agreement.
                  </p>
                </div>
                
                <div className="border border-gray-200 rounded-lg p-4">
                  <h4 className="font-semibold mb-2">🔄 Parallel Processing</h4>
                  <p className="text-gray-700 text-sm">
                    Sealevel runtime processes thousands of contracts in parallel, 
                    unlike Ethereum's sequential processing.
                  </p>
                </div>
                
                <div className="border border-gray-200 rounded-lg p-4">
                  <h4 className="font-semibold mb-2">📡 Gulf Stream</h4>
                  <p className="text-gray-700 text-sm">
                    Mempool-less transaction forwarding protocol that pushes transactions 
                    to validators before consensus, reducing confirmation times.
                  </p>
                </div>
                
                <div className="border border-gray-200 rounded-lg p-4">
                  <h4 className="font-semibold mb-2">🗜️ Turbine</h4>
                  <p className="text-gray-700 text-sm">
                    Block propagation protocol that breaks data into smaller packets 
                    and distributes them across the network efficiently.
                  </p>
                </div>
              </div>
            </div>

            <div className="bg-green-50 border border-green-200 rounded-lg p-4">
              <h4 className="font-semibold text-green-900 mb-2">🎯 Perfect for DEX</h4>
              <ul className="text-green-800 space-y-1 text-sm">
                <li>• Near-instant trade execution</li>
                <li>• Negligible gas fees enable micro-trades</li>
                <li>• High-frequency trading strategies possible</li>
                <li>• Better MEV protection with lower latency</li>
                <li>• Composability without gas concerns</li>
              </ul>
            </div>
          </div>
        );

      default:
        return null;
    }
  };

  return (
    <div className="max-w-4xl mx-auto">
      <div className="mb-6">
        <h1 className="text-2xl font-bold text-gray-900 mb-2">📚 Learn DeFi Concepts</h1>
        <p className="text-gray-600">
          Master the fundamentals of decentralized finance through interactive lessons and examples.
        </p>
      </div>

      {/* Topic Selection */}
      <div className="grid grid-cols-2 md:grid-cols-5 gap-3 mb-8">
        {LESSONS.map((lesson) => (
          <button
            key={lesson.id}
            onClick={() => setSelectedTopic(lesson.id)}
            className={`p-4 rounded-lg border-2 text-left transition-colors ${
              selectedTopic === lesson.id
                ? 'border-purple-500 bg-purple-50'
                : 'border-gray-200 bg-white hover:border-gray-300'
            }`}
          >
            <div className="text-2xl mb-2">{lesson.icon}</div>
            <div className="font-medium text-sm mb-1">{lesson.title}</div>
            <div className={`text-xs px-2 py-1 rounded ${
              lesson.difficulty === 'Beginner' ? 'bg-green-100 text-green-800' :
              lesson.difficulty === 'Intermediate' ? 'bg-yellow-100 text-yellow-800' :
              'bg-red-100 text-red-800'
            }`}>
              {lesson.difficulty}
            </div>
          </button>
        ))}
      </div>

      {/* Content */}
      <div className="bg-white rounded-lg shadow-sm border p-6">
        {renderContent()}
      </div>
    </div>
  );
};