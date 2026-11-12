import React, { useState, useMemo } from 'react';
import { 
  Shield, 
  Cpu, 
  Code2, 
  Play, 
  Database, 
  Terminal, 
  Coins, 
  Layers, 
  FileCode, 
  CheckCircle2, 
  AlertTriangle, 
  Activity, 
  Zap, 
  Lock, 
  RefreshCw, 
  Copy, 
  Check, 
  Search,
  ChevronRight,
  Server
} from 'lucide-react';
import { PROGRAMS } from './programs_data';

export default function App() {
  const programKeys = Object.keys(PROGRAMS);
  const [selectedProgramKey, setSelectedProgramKey] = useState<string>(programKeys[0] || 'solana-staking-simulation');
  const [selectedFileIndex, setSelectedFileIndex] = useState<number>(0);
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [copied, setCopied] = useState<boolean>(false);

  // Staking Simulator State
  const [stakeAmount, setStakeAmount] = useState<number>(100);
  const [durationDays, setDurationDays] = useState<number>(90);
  const [protectionTier, setProtectionTier] = useState<string>('100% Maximum Protection');
  const [isSimulating, setIsSimulating] = useState<boolean>(false);
  const [simulationResult, setSimulationResult] = useState({
    rewards: 2.45,
    fee: 0.0009,
    protectedAmount: 100,
    effectiveApy: 10.0
  });

  const activeProgram = PROGRAMS[selectedProgramKey] || { name: selectedProgramKey, files: [] };
  const activeFile = activeProgram.files[selectedFileIndex] || activeProgram.files[0] || { path: 'Cargo.toml', content: '// No files found' };

  // Filter files based on search query
  const filteredFiles = useMemo(() => {
    if (!searchQuery) return activeProgram.files;
    return activeProgram.files.filter((f: any) => 
      f.path.toLowerCase().includes(searchQuery.toLowerCase()) ||
      f.content.toLowerCase().includes(searchQuery.toLowerCase())
    );
  }, [activeProgram, searchQuery]);

  const handleCopyCode = () => {
    navigator.clipboard.writeText(activeFile.content);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const runSimulation = () => {
    setIsSimulating(true);
    setTimeout(() => {
      const apy = protectionTier.includes('100%') ? 10.5 : 8.5;
      const totalRewards = stakeAmount * (apy / 100) * (durationDays / 365);
      const fee = stakeAmount * 0.000009; // 0.0009% fee
      setSimulationResult({
        rewards: Number(totalRewards.toFixed(4)),
        fee: Number(fee.toFixed(6)),
        protectedAmount: stakeAmount,
        effectiveApy: apy
      });
      setIsSimulating(false);
    }, 300);
  };

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100 flex flex-col font-sans">
      {/* Top Navbar */}
      <header className="border-b border-slate-800 bg-slate-900/80 backdrop-blur-md sticky top-0 z-50 px-6 py-4 flex items-center justify-between">
        <div className="flex items-center space-x-3">
          <div className="bg-gradient-to-tr from-purple-600 to-indigo-500 p-2.5 rounded-xl shadow-lg shadow-purple-500/20">
            <Shield className="w-6 h-6 text-white" />
          </div>
          <div>
            <div className="flex items-center space-x-2">
              <h1 className="text-xl font-bold tracking-tight text-white">Tanihana Solana Staking Packages</h1>
              <span className="text-xs bg-purple-500/20 text-purple-300 border border-purple-500/30 px-2 py-0.5 rounded-full font-medium">Lib & On-Chain</span>
            </div>
            <p className="text-xs text-slate-400">Rust Smart Contract Packages with 100% Protection & Rewards Library</p>
          </div>
        </div>

        {/* Network Metrics */}
        <div className="hidden md:flex items-center space-x-6 text-sm">
          <div className="flex items-center space-x-2 bg-slate-800/60 px-3 py-1.5 rounded-lg border border-slate-700/50">
            <span className="w-2.5 h-2.5 rounded-full bg-emerald-500 animate-pulse"></span>
            <span className="text-slate-300 font-medium">Solana BPF / Anchor</span>
          </div>
          <div className="flex items-center space-x-1.5 text-slate-400">
            <Layers className="w-4 h-4 text-purple-400" />
            <span>Packages: <strong className="text-slate-200">{programKeys.length} Crates</strong></span>
          </div>
          <div className="flex items-center space-x-1.5 text-slate-400">
            <Lock className="w-4 h-4 text-indigo-400" />
            <span>Protection Rate: <strong className="text-slate-200">0.0009%</strong></span>
          </div>
        </div>
      </header>

      {/* Main Layout */}
      <div className="flex-1 max-w-7xl w-full mx-auto px-4 py-8 grid grid-cols-1 lg:grid-cols-4 gap-8">
        
        {/* Left Sidebar: Crate / Package Switcher */}
        <div className="lg:col-span-1 space-y-6">
          <div className="bg-slate-900/60 border border-slate-800 rounded-2xl p-5 shadow-xl">
            <h2 className="text-sm font-semibold uppercase tracking-wider text-slate-400 mb-4 flex items-center gap-2">
              <Server className="w-4 h-4 text-purple-400" />
              Rust Packages ({programKeys.length})
            </h2>
            <div className="space-y-2">
              {programKeys.map((key) => {
                const prog = PROGRAMS[key];
                const isSelected = selectedProgramKey === key;
                return (
                  <button
                    key={key}
                    onClick={() => {
                      setSelectedProgramKey(key);
                      setSelectedFileIndex(0);
                      setSearchQuery('');
                    }}
                    className={`w-full text-left p-3 rounded-xl transition-all duration-200 flex items-start justify-between group ${
                      isSelected 
                        ? 'bg-purple-600/20 border border-purple-500/40 text-white shadow-md shadow-purple-500/10' 
                        : 'bg-slate-800/40 hover:bg-slate-800 text-slate-300 border border-transparent'
                    }`}
                  >
                    <div>
                      <div className="font-medium text-xs font-mono flex items-center gap-2">
                        <Cpu className={`w-4 h-4 ${isSelected ? 'text-purple-400' : 'text-slate-500'}`} />
                        {prog.name}
                      </div>
                      <div className="text-[10px] text-slate-400 mt-1">{prog.files.length} source files</div>
                    </div>
                    <ChevronRight className={`w-4 h-4 mt-1 transition-transform ${isSelected ? 'text-purple-400 translate-x-0.5' : 'text-slate-600'}`} />
                  </button>
                );
              })}
            </div>
          </div>

          {/* Search Box */}
          <div className="bg-slate-900/60 border border-slate-800 rounded-2xl p-5 shadow-xl space-y-3">
            <h3 className="text-sm font-semibold text-slate-300 flex items-center gap-2">
              <Search className="w-4 h-4 text-purple-400" />
              Search Code
            </h3>
            <div className="relative">
              <input
                type="text"
                placeholder="Search functions, structs..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-xs text-slate-200 focus:outline-none focus:border-purple-500"
              />
            </div>
          </div>

          {/* Package Info Card */}
          <div className="bg-gradient-to-br from-purple-950/40 to-slate-900 border border-purple-500/20 rounded-2xl p-5 shadow-xl">
            <h3 className="text-sm font-semibold text-purple-300 mb-2 flex items-center gap-2">
              <Shield className="w-4 h-4 text-purple-400" />
              100% Protection Spec
            </h3>
            <p className="text-xs text-slate-300 leading-relaxed">
              Every Rust package implements robust on-chain staking with a <strong>0.0009% fee</strong> and a minimum wallet balance requirement of <strong>0.04 cents</strong>, guaranteeing zero principal loss.
            </p>
          </div>
        </div>

        {/* Right Area: Simulator & Code Viewer */}
        <div className="lg:col-span-3 space-y-6">
          
          {/* Header Banner */}
          <div className="bg-slate-900/60 border border-slate-800 rounded-2xl p-6 shadow-xl flex flex-col md:flex-row md:items-center justify-between gap-4">
            <div>
              <div className="flex items-center gap-2 mb-1">
                <span className="px-2.5 py-0.5 rounded-full text-xs font-semibold bg-indigo-500/20 text-indigo-300 border border-indigo-500/30 font-mono">
                  crate::{activeProgram.name}
                </span>
                <span className="px-2.5 py-0.5 rounded-full text-xs font-semibold bg-emerald-500/20 text-emerald-300 border border-emerald-500/30">
                  no_std ready
                </span>
              </div>
              <h2 className="text-2xl font-bold font-mono text-white">{activeProgram.name}</h2>
              <p className="text-slate-400 text-xs mt-1">Contains {activeProgram.files.length} Rust and configuration files.</p>
            </div>
          </div>

          {/* Interactive Staking Simulator */}
          <div className="bg-slate-900/70 border border-slate-800 rounded-2xl p-6 shadow-xl space-y-6">
            <div className="flex items-center justify-between border-b border-slate-800 pb-4">
              <div className="flex items-center space-x-2">
                <div className="p-2 bg-purple-500/20 rounded-lg text-purple-400">
                  <Coins className="w-5 h-5" />
                </div>
                <div>
                  <h3 className="font-semibold text-lg text-white">Staking & Protection Calculator</h3>
                  <p className="text-xs text-slate-400">Simulate APY, 0.0009% protection fee, and 100% coverage</p>
                </div>
              </div>
              <button
                onClick={runSimulation}
                disabled={isSimulating}
                className="px-4 py-2 bg-gradient-to-r from-purple-600 to-indigo-600 hover:from-purple-500 hover:to-indigo-500 text-white font-medium text-sm rounded-xl shadow-lg shadow-purple-600/30 transition-all flex items-center gap-2 cursor-pointer disabled:opacity-50"
              >
                <RefreshCw className={`w-4 h-4 ${isSimulating ? 'animate-spin' : ''}`} />
                {isSimulating ? 'Simulating...' : 'Run Simulation'}
              </button>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
              <div className="space-y-2">
                <label className="text-xs font-medium text-slate-400 flex justify-between">
                  <span>Stake Amount (SOL)</span>
                  <span className="text-purple-400 font-mono font-bold">{stakeAmount} SOL</span>
                </label>
                <input
                  type="range"
                  min="1"
                  max="1000"
                  value={stakeAmount}
                  onChange={(e) => setStakeAmount(Number(e.target.value))}
                  className="w-full accent-purple-500 bg-slate-800 rounded-lg h-2 cursor-pointer"
                />
              </div>

              <div className="space-y-2">
                <label className="text-xs font-medium text-slate-400 flex justify-between">
                  <span>Duration (Days)</span>
                  <span className="text-purple-400 font-mono font-bold">{durationDays} Days</span>
                </label>
                <input
                  type="range"
                  min="30"
                  max="365"
                  step="30"
                  value={durationDays}
                  onChange={(e) => setDurationDays(Number(e.target.value))}
                  className="w-full accent-purple-500 bg-slate-800 rounded-lg h-2 cursor-pointer"
                />
              </div>

              <div className="space-y-2">
                <label className="text-xs font-medium text-slate-400">Protection Tier</label>
                <select
                  value={protectionTier}
                  onChange={(e) => setProtectionTier(e.target.value)}
                  className="w-full bg-slate-800 border border-slate-700 text-slate-200 text-sm rounded-xl px-3 py-2.5 focus:outline-none focus:border-purple-500"
                >
                  <option>100% Maximum Protection</option>
                  <option>90% Standard Protection</option>
                </select>
              </div>
            </div>

            {/* Results */}
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4 pt-2">
              <div className="bg-slate-800/50 border border-slate-700/60 rounded-xl p-4">
                <div className="text-xs text-slate-400 mb-1">Estimated Rewards</div>
                <div className="text-xl font-bold font-mono text-emerald-400">+{simulationResult.rewards} SOL</div>
                <div className="text-[10px] text-slate-500 mt-1">{simulationResult.effectiveApy}% APY</div>
              </div>
              <div className="bg-slate-800/50 border border-slate-700/60 rounded-xl p-4">
                <div className="text-xs text-slate-400 mb-1">Fee (0.0009%)</div>
                <div className="text-xl font-bold font-mono text-purple-300">{simulationResult.fee} SOL</div>
                <div className="text-[10px] text-slate-500 mt-1">Min balance: $0.04</div>
              </div>
              <div className="bg-slate-800/50 border border-slate-700/60 rounded-xl p-4">
                <div className="text-xs text-slate-400 mb-1">Protected Principal</div>
                <div className="text-xl font-bold font-mono text-indigo-300">{simulationResult.protectedAmount} SOL</div>
                <div className="text-[10px] text-slate-500 mt-1">100% Guaranteed</div>
              </div>
              <div className="bg-slate-800/50 border border-slate-700/60 rounded-xl p-4">
                <div className="text-xs text-slate-400 mb-1">Slashing Defense</div>
                <div className="text-lg font-bold font-mono text-emerald-400 flex items-center gap-1.5 mt-0.5">
                  <CheckCircle2 className="w-4 h-4" /> Active
                </div>
                <div className="text-[10px] text-slate-500 mt-1">Zero Risk</div>
              </div>
            </div>
          </div>

          {/* Code Viewer */}
          <div className="bg-slate-900/70 border border-slate-800 rounded-2xl shadow-xl overflow-hidden">
            <div className="bg-slate-900 border-b border-slate-800 px-4 py-3 flex items-center justify-between flex-wrap gap-2">
              <div className="flex items-center space-x-2 overflow-x-auto">
                <FileCode className="w-4 h-4 text-purple-400" />
                <span className="text-xs font-semibold uppercase tracking-wider text-slate-400 mr-2">Files:</span>
                {filteredFiles.map((file: any, idx: number) => {
                  const originalIndex = activeProgram.files.findIndex((f: any) => f.path === file.path);
                  return (
                    <button
                      key={file.path}
                      onClick={() => setSelectedFileIndex(originalIndex)}
                      className={`px-3 py-1.5 rounded-lg text-xs font-mono transition-all duration-150 flex items-center gap-1.5 ${
                        selectedFileIndex === originalIndex
                          ? 'bg-purple-600/30 text-purple-200 border border-purple-500/50 font-semibold'
                          : 'bg-slate-800/60 text-slate-400 hover:text-slate-200 hover:bg-slate-800 border border-transparent'
                      }`}
                    >
                      {file.path}
                    </button>
                  );
                })}
              </div>

              <button
                onClick={handleCopyCode}
                className="flex items-center space-x-1.5 px-3 py-1.5 bg-slate-800 hover:bg-slate-700 text-slate-300 hover:text-white rounded-lg text-xs font-medium transition-colors border border-slate-700 cursor-pointer"
              >
                {copied ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                <span>{copied ? 'Copied!' : 'Copy Code'}</span>
              </button>
            </div>

            <div className="p-6 bg-slate-950/80 overflow-x-auto max-h-[550px] overflow-y-auto font-mono text-xs leading-relaxed text-slate-300">
              <pre className="font-mono">{activeFile.content}</pre>
            </div>
          </div>

        </div>

      </div>

      {/* Footer */}
      <footer className="border-t border-slate-800/80 py-6 px-6 text-center text-xs text-slate-500 bg-slate-900/40">
        Tanihana Solana Staking Packages &bull; 100% Maximum Protection Library &bull; Built with Rust & Solana BPF
      </footer>
    </div>
  );
}
