// Simple development server using Bun with hot module reloading
import { resolve } from 'path';

const ALLOWED_DIR = resolve(process.cwd());

// Validate that the resolved path is within the allowed directory
function isPathSafe(requestedPath: string): boolean {
  const resolvedPath = resolve(ALLOWED_DIR, requestedPath);
  return resolvedPath.startsWith(ALLOWED_DIR);
}

const server = Bun.serve({
  port: 3000,
  async fetch(req) {
    const url = new URL(req.url);
    
    // Serve index.html for root
    if (url.pathname === '/') {
      return new Response(Bun.file('index.html'));
    }
    
    // Handle TypeScript/TSX files - transpile on the fly
    if (url.pathname.endsWith('.tsx') || url.pathname.endsWith('.ts')) {
      const filePath = url.pathname.slice(1);
      
      if (!isPathSafe(filePath)) {
        return new Response('Forbidden', { status: 403 });
      }
      
      const file = Bun.file(filePath);
      
      if (await file.exists()) {
        const transpiled = await Bun.build({
          entrypoints: [filePath],
          minify: false,
          sourcemap: 'inline',
          target: 'browser',
          format: 'esm',
        });
        
        if (transpiled.outputs.length > 0) {
          return new Response(await transpiled.outputs[0].text(), {
            headers: {
              'Content-Type': 'application/javascript',
            },
          });
        }
      }
      
      return new Response('Not Found', { status: 404 });
    }
    
    // Serve static files with path validation
    const filePath = url.pathname.slice(1);
    
    if (!isPathSafe(filePath)) {
      return new Response('Forbidden', { status: 403 });
    }
    
    const file = Bun.file(filePath);
    
    if (await file.exists()) {
      return new Response(file);
    }
    
    // 404
    return new Response('Not Found', { status: 404 });
  },
});

console.log(`🚀 Dev server running at http://localhost:${server.port}`);
console.log(`📦 Open your browser to view the SysML Viewer`);
