import React, { useState } from 'react';
import { SwapInterface } from './SwapInterface';
import { PoolInterface } from './PoolInterface';
import { EducationalPanel } from './EducationalPanel';

type Tab = 'swap' | 'pool' | 'learn';

export const StudentDEX: React.FC = () => {
  const [activeTab, setActiveTab] = useState<Tab>('swap');
  const [connected, setConnected] = useState<boolean>(true); // Demo mode

  const tabs = [
    { id: 'swap', name: 'Swap Tokens', icon: '🔄' },
    { id: 'pool', name: 'Liquidity Pool', icon: '💧' },
    { id: 'learn', name: 'Learn & Analyze', icon: '📚' },
  ];

  if (!connected) {
    return (
      <div className="bg-white rounded-lg shadow-md p-8 text-center">
        <div className="text-6xl mb-4">🔗</div>
        <h2 className="text-2xl font-bold text-gray-900 mb-4">Connect Your Wallet</h2>
        <p className="text-gray-600 mb-6">
          To start learning and interacting with the Student DEX, please connect your Solana wallet.
          For testing purposes, you can use the development wallet provided.
        </p>
        <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
          <h3 className="font-semibold text-blue-900 mb-2">💡 Getting Started Tips:</h3>
          <ul className="text-sm text-blue-800 text-left space-y-1">
            <li>• Make sure you're on Solana Devnet</li>
            <li>• Get some SOL from the Solana faucet</li>
            <li>• Create test tokens for learning</li>
            <li>• Always double-check transaction details</li>
          </ul>
        </div>
        <button 
          onClick={() => setConnected(true)}
          className="mt-4 bg-purple-600 hover:bg-purple-700 text-white px-6 py-2 rounded-lg font-medium transition-colors"
        >
          Connect Demo Wallet
        </button>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow-md overflow-hidden">
      {/* Tab Navigation */}
      <div className="border-b border-gray-200">
        <nav className="flex">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id as Tab)}
              className={`flex-1 px-6 py-4 text-center font-medium transition-colors ${
                activeTab === tab.id
                  ? 'text-purple-600 border-b-2 border-purple-600 bg-purple-50'
                  : 'text-gray-500 hover:text-gray-700 hover:bg-gray-50'
              }`}
            >
              <div className="flex items-center justify-center space-x-2">
                <span className="text-lg">{tab.icon}</span>
                <span className="hidden sm:block">{tab.name}</span>
              </div>
            </button>
          ))}
        </nav>
      </div>

      {/* Tab Content */}
      <div className="p-6">
        {activeTab === 'swap' && <SwapInterface />}
        {activeTab === 'pool' && <PoolInterface />}
        {activeTab === 'learn' && <EducationalPanel />}
      </div>
    </div>
  );
};