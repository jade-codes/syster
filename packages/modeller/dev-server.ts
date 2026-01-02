/**
 * Simple development server for the Syster Modeller
 */
import { resolve } from 'path';

const PORT = 3000;
const ALLOWED_DIR = resolve(process.cwd());

const server = Bun.serve({
  port: PORT,
  async fetch(req) {
    const url = new URL(req.url);
    
    // Serve the HTML file for the root path
    if (url.pathname === '/') {
      const html = await Bun.file('./src/index.html').text();
      return new Response(html, {
        headers: { 'Content-Type': 'text/html' },
      });
    }
    
    // Transpile and serve TypeScript/TSX files
    if (url.pathname.endsWith('.tsx') || url.pathname.endsWith('.ts')) {
      const srcDir = resolve(ALLOWED_DIR, 'src');
      const filePath = resolve(srcDir, `.${url.pathname}`);
      
      if (!filePath.startsWith(srcDir)) {
        return new Response('Forbidden', { status: 403 });
      }
      
      const file = Bun.file(filePath);
      
      if (await file.exists()) {
        const transpiled = await Bun.build({
          entrypoints: [filePath],
          target: 'browser',
          format: 'esm',
        });
        
        if (transpiled.outputs[0]) {
          return new Response(await transpiled.outputs[0].text(), {
            headers: { 
              'Content-Type': 'application/javascript',
              'Access-Control-Allow-Origin': '*',
            },
          });
        }
      }
    }
    
    // Serve CSS files
    if (url.pathname.endsWith('.css')) {
      const srcDir = resolve(ALLOWED_DIR, 'src');
      const filePath = resolve(srcDir, `.${url.pathname}`);
      
      if (!filePath.startsWith(srcDir)) {
        return new Response('Forbidden', { status: 403 });
      }
      
      const file = Bun.file(filePath);
      
      if (await file.exists()) {
        return new Response(file, {
          headers: { 'Content-Type': 'text/css' },
        });
      }
    }
    
    // Serve from node_modules with whitelist-based security
    if (url.pathname.startsWith('/node_modules/')) {
      const nodeModulesDir = resolve(ALLOWED_DIR, 'node_modules');
      const filePath = resolve(nodeModulesDir, url.pathname.slice('/node_modules/'.length));
      
      if (!filePath.startsWith(nodeModulesDir)) {
        return new Response('Forbidden', { status: 403 });
      }
      
      const file = Bun.file(filePath);
      if (await file.exists()) {
        return new Response(file);
      }
    }
    
    return new Response('Not Found', { status: 404 });
  },
});

console.log(`🚀 Syster Modeller dev server running at http://localhost:${PORT}`);
