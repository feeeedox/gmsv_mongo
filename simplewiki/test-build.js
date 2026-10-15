import { execSync } from 'child_process';
import fs from 'fs';
import path from 'path';

const configPath = path.join(process.cwd(), 'nuxt.config.ts');
let baseUrl = '/';

if (fs.existsSync(configPath)) {
  const configContent = fs.readFileSync(configPath, 'utf8');
  const match = configContent.match(/baseURL:\s*process\.env\.NODE_ENV\s*===\s*'production'\s*?\?\s*?['"]([^'"]+)['"]/);
  if (match && match[1]) {
    baseUrl = match[1];
  }
}

console.log(`\x1b[36m%s\x1b[0m`, `🤖 Simulating github pages environment on: "${baseUrl}"...`);

const publicDir = path.join(process.cwd(), '.output', 'public');

if (!fs.existsSync(publicDir)) {
  console.error(`\x1b[31m%s\x1b[0m`, `❌ Error: The directory ".output/public" does not exist.`);
  console.log(`Please make sure to build the project first by running "npx nuxi build" before starting the server.`);
  process.exit(1);
}

console.log(`\x1b[32m%s\x1b[0m`, `🚀 Starting server on http://localhost:8080${baseUrl}`);
try {
  execSync(`npx sirv-cli .output/public --port 8080 --assets --start ${baseUrl}`, { stdio: 'inherit' });
} catch (error) {  console.log(`\n👋 Server stopped.`);
}