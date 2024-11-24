import { existsSync, rmSync } from "fs";
import { join } from "path";
import { spawnSync } from "child_process";

const dir = join(import.meta.dirname, "..", "dist");

await (async () => {
  if (existsSync(dir)) {
    rmSync(dir, { recursive: true, force: true });
    console.log("[DEV] [CLEAN] Cleaned up ./dist directory");
  } else {
    console.log("[DEV] [CLEAN] ./dist directory does not exist.. Skipping");
  }

  if (process.argv.includes("--compile")) {
    console.log("[DEV] [COMPILE] Compiling..");
    spawnSync("npx", ["tsc"], {
      cwd: join(import.meta.dirname, ".."),
      stdio: "inherit",
      shell: true,
    });
  }
})();
