import React from 'react';
import './App.css';
import { StudentDEX } from './components/StudentDEX';
import { Header } from './components/Header';
import { Footer } from './components/Footer';

function App() {
  return (
    <div className="min-h-screen bg-gray-50">
      <Header />
      <main className="container mx-auto px-4 py-8">
        <div className="max-w-4xl mx-auto">
          <div className="text-center mb-8">
            <h1 className="text-4xl font-bold text-gray-900 mb-4">
              🎓 Student DEX - Learn Solana DeFi
            </h1>
            <p className="text-lg text-gray-600 max-w-2xl mx-auto">
              An educational decentralized exchange built on Solana. 
              Learn about AMMs, liquidity provision, and token swapping 
              in a hands-on environment.
            </p>
          </div>
          
          {/* Educational Info Section */}
          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
            <div className="bg-white p-6 rounded-lg shadow-md">
              <div className="text-2xl mb-3">🔄</div>
              <h3 className="text-lg font-semibold mb-2">Automated Market Maker</h3>
              <p className="text-gray-600 text-sm">
                Learn how constant product formula (x × y = k) enables automatic price discovery without order books.
              </p>
            </div>
            
            <div className="bg-white p-6 rounded-lg shadow-md">
              <div className="text-2xl mb-3">💧</div>
              <h3 className="text-lg font-semibold mb-2">Liquidity Provision</h3>
              <p className="text-gray-600 text-sm">
                Understand how providing liquidity earns fees but comes with impermanent loss risk.
              </p>
            </div>
            
            <div className="bg-white p-6 rounded-lg shadow-md">
              <div className="text-2xl mb-3">⚡</div>
              <h3 className="text-lg font-semibold mb-2">Fast & Cheap</h3>
              <p className="text-gray-600 text-sm">
                Experience Solana's high throughput and low fees compared to other blockchains.
              </p>
            </div>
          </div>

          {/* Main DEX Interface */}
          <StudentDEX />
        </div>
      </main>
      <Footer />
    </div>
  );
}

export default App;
