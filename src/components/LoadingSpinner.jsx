import React from 'react';

function LoadingSpinner() {
  return (
    <div className="flex items-center justify-center py-12">
      <div className="relative">
        <div className="w-12 h-12 border-4 border-gray-700 rounded-full"></div>
        <div className="w-12 h-12 border-4 border-blue-500 border-t-transparent rounded-full animate-spin absolute top-0 left-0"></div>
      </div>
      <span className="ml-4 text-gray-400 text-lg">Loading pull requests...</span>
    </div>
  );
}

export default LoadingSpinner;
