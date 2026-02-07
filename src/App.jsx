import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import PRList from './components/PRList';
import LoadingSpinner from './components/LoadingSpinner';

function App() {
  const [prs, setPRs] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    loadPRs();
  }, []);

  async function loadPRs() {
    try {
      setLoading(true);
      setError(null);
      const pulls = await invoke('get_pull_requests');
      setPRs(pulls);
    } catch (err) {
      console.error('Error fetching PRs:', err);
      setError(err.toString());
    } finally {
      setLoading(false);
    }
  }

  async function handlePRClick(prNumber) {
    try {
      const result = await invoke('launch_godot', { prNumber });
      console.log(result);
    } catch (err) {
      console.error('Error launching Godot:', err);
      alert(`Error: ${err}`);
    }
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-900 via-gray-800 to-gray-900 text-white">
      {/* Header */}
      <header className="bg-gray-800/50 backdrop-blur-sm border-b border-gray-700 sticky top-0 z-10">
        <div className="max-w-7xl mx-auto px-6 py-4 flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold bg-gradient-to-r from-blue-400 to-purple-500 bg-clip-text text-transparent">
              Godot PR Launcher
            </h1>
            <p className="text-sm text-gray-400 mt-1">kinlly/ylbtm</p>
          </div>
          <button
            onClick={loadPRs}
            className="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors flex items-center gap-2"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            Refresh
          </button>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-6 py-8">
        {loading && <LoadingSpinner />}
        
        {error && (
          <div className="bg-red-900/20 border border-red-500 rounded-lg p-4 text-red-200">
            <p className="font-semibold">Error loading PRs</p>
            <p className="text-sm mt-1">{error}</p>
          </div>
        )}

        {!loading && !error && (
          <PRList prs={prs} onPRClick={handlePRClick} />
        )}
      </main>
    </div>
  );
}

export default App;
