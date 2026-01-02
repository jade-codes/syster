import { useEffect, useState } from 'react';

/**
 * Mock LSP connection data
 */
interface LspConnectionState {
  connected: boolean;
  serverInfo?: {
    name: string;
    version: string;
  };
}

/**
 * Hook for managing LSP (Language Server Protocol) connection.
 * 
 * This is a stub implementation that returns mock data.
 * Real LSP connection will be implemented later.
 * 
 * @returns LSP connection state
 */
export const useLspConnection = (): LspConnectionState => {
  const [connectionState, setConnectionState] = useState<LspConnectionState>({
    connected: false,
  });

  useEffect(() => {
    // Simulate connection after component mount
    const timer = setTimeout(() => {
      setConnectionState({
        connected: true,
        serverInfo: {
          name: 'syster-lsp',
          version: '0.1.0',
        },
      });
    }, 1000);

    return () => clearTimeout(timer);
  }, []);

  return connectionState;
};
