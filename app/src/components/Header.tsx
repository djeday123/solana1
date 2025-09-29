import React from 'react';

export const Header: React.FC = () => {
  return (
    <header className="bg-white shadow-sm border-b">
      <div className="container mx-auto px-4 py-4 flex justify-between items-center">
        <div className="flex items-center space-x-3">
          <div className="text-2xl font-bold text-purple-600">
            🎓 Student DEX
          </div>
          <span className="text-sm bg-purple-100 text-purple-800 px-2 py-1 rounded-full">
            Educational
          </span>
        </div>
        
        <div className="flex items-center space-x-4">
          <nav className="hidden md:flex space-x-6">
            <a href="#swap" className="text-gray-600 hover:text-gray-900">Swap</a>
            <a href="#pool" className="text-gray-600 hover:text-gray-900">Pool</a>
            <a href="#learn" className="text-gray-600 hover:text-gray-900">Learn</a>
          </nav>
          
          <button className="bg-purple-600 hover:bg-purple-700 text-white px-4 py-2 rounded-lg font-medium transition-colors">
            Connect Wallet (Demo)
          </button>
        </div>
      </div>
    </header>
  );
};