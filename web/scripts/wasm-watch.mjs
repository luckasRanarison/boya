import { spawn } from "child_process";
import fs from "fs";
import chalk from "chalk";

let dirty = false;
let running = false;
let debounce = false;

const [, , path] = process.argv;

if (!path) {
  console.error(chalk.red("Error: Path not provided"));
  process.exit(1);
}

function log(message) {
  console.log(
    `${chalk.dim(new Date().toLocaleTimeString("en-US"))} ${chalk.cyan.bold("[wasm-watch]")} ${message}`,
  );
}

function build() {
  const process = spawn("npm", ["run", "wasm:build"], {});
  const start = Date.now();
  let stderr = "";

  process.stderr.on("data", (chunk) => {
    stderr += chunk;
  });

  process.on("exit", (code) => {
    running = false;

    if (code === 0) {
      log(
        `Build completed! ${chalk.yellow(`(${(Date.now() - start) / 1000}s)`)}`,
      );
    } else {
      console.error(chalk.red(stderr));
    }

    if (dirty) {
      build();
    }
  });
}

fs.watch(path, { recursive: true }, (event, filename) => {
  if (!filename || debounce) return;

  log(`${filename} ${chalk.yellow(`(${event}d)`)}`);
  log("Rebuilding boya_wasm...");
  dirty = false;
  debounce = true;

  setTimeout(() => {
    debounce = false;
  }, 500);

  if (running) {
    dirty = true;
    return;
  }

  build();
});
