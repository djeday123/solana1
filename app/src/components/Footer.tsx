import React from 'react';

export const Footer: React.FC = () => {
  return (
    <footer className="bg-gray-800 text-white py-8 mt-12">
      <div className="container mx-auto px-4">
        <div className="grid md:grid-cols-3 gap-8">
          <div>
            <h3 className="text-lg font-semibold mb-4">🎓 Student DEX</h3>
            <p className="text-gray-300 text-sm">
              An educational decentralized exchange built on Solana to help students 
              learn about DeFi, AMMs, and blockchain development.
            </p>
          </div>
          
          <div>
            <h3 className="text-lg font-semibold mb-4">Learn More</h3>
            <ul className="space-y-2 text-sm">
              <li><a href="https://docs.solana.com/" className="text-gray-300 hover:text-white">Solana Docs</a></li>
              <li><a href="https://solanacookbook.com/" className="text-gray-300 hover:text-white">Solana Cookbook</a></li>
              <li><a href="https://ethereum.org/en/developers/docs/dexs/" className="text-gray-300 hover:text-white">DEX Theory</a></li>
              <li><a href="https://finematics.com/automated-market-maker-amm-explained/" className="text-gray-300 hover:text-white">AMM Explained</a></li>
            </ul>
          </div>
          
          <div>
            <h3 className="text-lg font-semibold mb-4">Educational Warning</h3>
            <div className="bg-yellow-600 p-3 rounded text-xs">
              ⚠️ This is an educational project. Do not use real funds or deploy 
              to mainnet without proper security audits. For learning purposes only.
            </div>
          </div>
        </div>
        
        <div className="border-t border-gray-700 mt-8 pt-8 text-center text-sm text-gray-400">
          <p>Built with ❤️ for Solana education • Open source • MIT License</p>
        </div>
      </div>
    </footer>
  );
};