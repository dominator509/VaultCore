import { spawnSync } from "node:child_process";

const args = process.argv.slice(2).filter((arg) => arg !== "--");
const result = spawnSync("pnpm", ["exec", "playwright", "test", ...args], {
  cwd: new URL("..", import.meta.url),
  shell: true,
  stdio: "inherit",
});

process.exit(result.status ?? 1);
