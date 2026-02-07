import React from 'react';

function PRList({ prs, onPRClick, onMainClick }) {
  return (
    <div className="space-y-3">
      {/* Return to Main button - always at the top */}
      <div
        onClick={onMainClick}
        className="bg-gradient-to-r from-purple-900/40 to-blue-900/40 hover:from-purple-800/50 hover:to-blue-800/50 border-2 border-purple-500 hover:border-purple-400 p-5 rounded-xl cursor-pointer transition-all duration-200 group"
      >
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-4">
            <div className="bg-purple-500/20 p-3 rounded-lg border border-purple-500">
              <svg className="w-6 h-6 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
              </svg>
            </div>
            <div>
              <h2 className="text-xl font-bold text-white group-hover:text-purple-300 transition-colors flex items-center gap-2">
                Return to Main Branch
                <span className="px-2 py-0.5 bg-purple-500/30 text-purple-300 text-xs font-semibold rounded border border-purple-500">
                  main
                </span>
              </h2>
              <p className="text-sm text-gray-400 mt-1">Checkout and launch the main branch</p>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <svg className="w-6 h-6 text-purple-400 group-hover:text-purple-300 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14 5l7 7m0 0l-7 7m7-7H3" />
            </svg>
          </div>
        </div>
      </div>

      {/* Separator */}
      {prs.length > 0 && (
        <div className="flex items-center gap-4 py-2">
          <div className="flex-1 border-t border-gray-700"></div>
          <span className="text-gray-500 text-sm font-medium">Pull Requests</span>
          <div className="flex-1 border-t border-gray-700"></div>
        </div>
      )}

      {/* PR List */}
      {prs.length === 0 ? (
        <div className="text-center py-12">
          <svg className="w-16 h-16 mx-auto text-gray-600 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
          </svg>
          <p className="text-gray-400 text-lg">No open pull requests found</p>
        </div>
      ) : (
        prs.map((pr) => (
          <div
            key={pr.number}
            onClick={() => onPRClick(pr.number, pr.head_ref)}
            className="bg-gray-800/50 hover:bg-gray-700/50 border border-gray-700 hover:border-blue-500 p-5 rounded-xl cursor-pointer transition-all duration-200 group"
          >
            <div className="flex items-start justify-between">
              <div className="flex-1">
                <div className="flex items-center gap-3 mb-2">
                  <span className="px-2.5 py-1 bg-green-900/30 text-green-400 text-xs font-semibold rounded-full border border-green-700">
                    #{pr.number}
                  </span>
                  <span className="text-gray-400 text-sm">by @{pr.user}</span>
                </div>
                <h2 className="text-lg font-semibold text-white group-hover:text-blue-400 transition-colors">
                  {pr.title}
                </h2>
                <div className="flex items-center gap-3 mt-2">
                  <span className="text-xs text-blue-400 bg-blue-900/30 px-2 py-0.5 rounded border border-blue-700">
                    ↳ {pr.head_ref}
                  </span>
                  <span className="text-sm text-gray-500">
                    Updated: {new Date(pr.updated_at).toLocaleDateString()}
                  </span>
                </div>
              </div>
              <div className="flex items-center gap-2 ml-4">
                <svg className="w-5 h-5 text-gray-500 group-hover:text-blue-400 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14 5l7 7m0 0l-7 7m7-7H3" />
                </svg>
              </div>
            </div>
          </div>
        ))
      )}
    </div>
  );
}

export default PRList;
