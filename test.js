
// static-server.js
const http = require('http');
const fs = require('fs');
const path = require('path');

const port = 3000;
const baseDir = process.cwd();

const mimeTypes = {
  '.html': 'text/html',
  '.js': 'text/javascript',
  '.css': 'text/css',
  '.json': 'application/json',
  '.png': 'image/png',
  '.jpg': 'image/jpeg',
  '.gif': 'image/gif',
  '.svg': 'image/svg+xml',
  '.txt': 'text/plain',
  '.wasm': 'application/wasm'
};

const server = http.createServer((req, res) => {
  let filePath = path.join(baseDir, decodeURIComponent(req.url));

  // Default to index.html
  if (req.url === '/') {
    filePath = path.join(baseDir, 'index.html');
  }

  // Prevent directory traversal
  if (!filePath.startsWith(baseDir)) {
    res.writeHead(403);
    return res.end('Forbidden');
  }

  fs.stat(filePath, (err, stats) => {
    if (err || !stats.isFile()) {
      res.writeHead(404);
      return res.end('Not Found');
    }

    const ext = path.extname(filePath);
    const contentType = mimeTypes[ext] || 'application/octet-stream';

    res.writeHead(200, { 'Content-Type': contentType });

    const stream = fs.createReadStream(filePath);
    stream.pipe(res);
  });
});

server.listen(port, () => {
  console.log(`Static server running at http://localhost:${port}`);
});
