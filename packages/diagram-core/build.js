import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

// Check if dist directory exists
const distDir = path.join(__dirname, 'dist');
if (fs.existsSync(distDir)) {
  console.log('Build completed successfully!');
  console.log('Generated files in dist/');
} else {
  console.error('Build failed: dist directory not found');
  process.exit(1);
}

