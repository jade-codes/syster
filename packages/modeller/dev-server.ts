/**
 * Simple development server for the Syster Modeller
 */

const PORT = 3000;

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
    
    // Serve static files from src directory
    const filePath = `./src${url.pathname}`;
    const file = Bun.file(filePath);
    
    if (await file.exists()) {
      return new Response(file);
    }
    
    // Serve from node_modules
    if (url.pathname.startsWith('/node_modules/')) {
      const file = Bun.file(`.${url.pathname}`);
      if (await file.exists()) {
        return new Response(file);
      }
    }
    
    return new Response('Not Found', { status: 404 });
  },
});

console.log(`🚀 Syster Modeller dev server running at http://localhost:${PORT}`);
