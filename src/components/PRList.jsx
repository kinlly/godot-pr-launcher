import React from 'react';

function PRList({ prs, onPRClick }) {
  if (prs.length === 0) {
    return (
      <div className="text-center py-12">
        <svg className="w-16 h-16 mx-auto text-gray-600 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
        <p className="text-gray-400 text-lg">No open pull requests found</p>
      </div>
    );
  }

  return (
    <div className="space-y-3">
      {prs.map((pr) => (
        <div
          key={pr.number}
          onClick={() => onPRClick(pr.number)}
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
              <p className="text-sm text-gray-500 mt-2">
                Updated: {new Date(pr.updated_at).toLocaleDateString()}
              </p>
            </div>
            <div className="flex items-center gap-2 ml-4">
              <svg className="w-5 h-5 text-gray-500 group-hover:text-blue-400 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14 5l7 7m0 0l-7 7m7-7H3" />
              </svg>
            </div>
          </div>
        </div>
      ))}
    </div>
  );
}

export default PRList;
