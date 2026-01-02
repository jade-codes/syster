import React from 'react';
import { Viewer } from './Viewer';

/**
 * Demo application wrapper for the Viewer component.
 * Shows an empty diagram canvas for development.
 */
export const App: React.FC = () => {
  return (
    <div style={{ width: '100vw', height: '100vh' }}>
      <Viewer />
    </div>
  );
};

export default App;
