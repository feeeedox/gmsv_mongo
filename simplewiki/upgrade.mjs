#!/usr/bin/env node
/**
 * SimpleWiki Upgrade
 * ------------------
 * Downloads the latest version from github.com/feeeedox/simplewiki and
 * updates ONLY the framework files (components, pages, configs,
 * package.json, etc.).
 *
 * User-owned files such as `content/`, `public/`, and `app/app.config.ts`
 * are left untouched.
 *
 * Usage:
 *   node upgrade.mjs            # Run the upgrade
 *   node upgrade.mjs --dry-run  # Preview changes without modifying anything
 *   node upgrade.mjs --branch=dev
 *
 * Requirements:
 *   Git must be installed and available in your PATH.
 */

import { execSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import os from "node:os";

const REPO_URL = "https://github.com/feeeedox/simplewiki.git";
const CWD = process.cwd();

const args = process.argv.slice(2);
const DRY_RUN = args.includes("--dry-run");
const branchArg = args.find((a) => a.startsWith("--branch="));
const BRANCH = branchArg ? branchArg.split("=")[1] : "main";

const MANAGED_PATHS = [
  "app/app.vue",
  "app/assets",
  "app/components",
  "app/pages",
  "app/util",
  "nuxt.config.ts",
  "content.config.ts",
  "tsconfig.json",
  "renovate.json",
  "test-build.js",
  ".github",
];

const PROTECTED_PATHS = [
  "content",
  "public",
  "app/app.config.ts",
  ".git",
  "node_modules",
  ".env",
];

function log(msg) {
  console.log(msg);
}

function isProtected(relPath) {
  return PROTECTED_PATHS.some(
    (p) => relPath === p || relPath.startsWith(p + path.sep)
  );
}

function ensureCleanGitState() {
  try {
    const status = execSync("git status --porcelain", { cwd: CWD }).toString();

    if (status.trim().length > 0) {
      log(
        "⚠️  Your working directory contains uncommitted changes.\n" +
          "   Please commit or stash them before running the upgrade.\n" +
          "   This allows you to easily review or revert changes using\n" +
          "   'git diff' or 'git checkout .'.\n"
      );

      if (!DRY_RUN) process.exit(1);
    }
  } catch {
    log(
      "ℹ️  No Git repository detected. Automatic rollback via Git is unavailable."
    );
  }
}

function copyRecursive(src, dest) {
  const stat = fs.statSync(src);

  if (stat.isDirectory()) {
    fs.mkdirSync(dest, { recursive: true });

    for (const entry of fs.readdirSync(src)) {
      copyRecursive(path.join(src, entry), path.join(dest, entry));
    }
  } else {
    fs.mkdirSync(path.dirname(dest), { recursive: true });
    fs.copyFileSync(src, dest);
  }
}

function removeIfExists(target) {
  if (fs.existsSync(target)) {
    fs.rmSync(target, { recursive: true, force: true });
  }
}

function syncManagedPath(tmpRepo, relPath) {
  if (isProtected(relPath)) {
    log(`⏭️  Skipped (protected): ${relPath}`);
    return;
  }

  const src = path.join(tmpRepo, relPath);
  const dest = path.join(CWD, relPath);

  if (!fs.existsSync(src)) {
    log(`ℹ️  ${relPath} no longer exists upstream. Consider removing it manually if needed.`);
    return;
  }

  if (DRY_RUN) {
    log(`🔍 Would update: ${relPath}`);
    return;
  }

  removeIfExists(dest);
  copyRecursive(src, dest);

  log(`✅ Updated: ${relPath}`);
}

function mergePackageJson(tmpRepo) {
  const localPath = path.join(CWD, "package.json");
  const upstreamPath = path.join(tmpRepo, "package.json");

  if (!fs.existsSync(localPath) || !fs.existsSync(upstreamPath)) return;

  const local = JSON.parse(fs.readFileSync(localPath, "utf-8"));
  const upstream = JSON.parse(fs.readFileSync(upstreamPath, "utf-8"));

  const mergedDeps = {
    ...(local.dependencies || {}),
    ...(upstream.dependencies || {}),
  };

  const mergedDevDeps = {
    ...(local.devDependencies || {}),
    ...(upstream.devDependencies || {}),
  };

  const mergedScripts = {
    ...(local.scripts || {}),
    ...(upstream.scripts || {}),
  };

  const merged = {
    ...local,
    scripts: mergedScripts,
    dependencies: mergedDeps,
    devDependencies: mergedDevDeps,
  };

  if (DRY_RUN) {
    log("🔍 Would merge package.json (dependencies, devDependencies, scripts).");
    return;
  }

  fs.writeFileSync(localPath, JSON.stringify(merged, null, 2) + "\n");

  log("✅ package.json merged (your custom dependencies and scripts were preserved).");
}

function detectPackageManager() {
  if (
    fs.existsSync(path.join(CWD, "bun.lock")) ||
    fs.existsSync(path.join(CWD, "bun.lockb"))
  )
    return "bun install";

  if (fs.existsSync(path.join(CWD, "pnpm-lock.yaml")))
    return "pnpm install";

  if (fs.existsSync(path.join(CWD, "yarn.lock")))
    return "yarn install";

  return "npm install";
}

function main() {
  log(`\n📦 SimpleWiki Upgrade (Branch: ${BRANCH})${DRY_RUN ? " [dry-run]" : ""}\n`);

  ensureCleanGitState();

  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "simplewiki-upgrade-"));

  log(`⬇️  Cloning upstream repository into ${tmpDir}...`);

  execSync(
    `git clone --depth 1 --branch ${BRANCH} ${REPO_URL} "${tmpDir}"`,
    {
      stdio: "inherit",
    }
  );

  for (const relPath of MANAGED_PATHS) {
    syncManagedPath(tmpDir, relPath);
  }

  mergePackageJson(tmpDir);

  fs.rmSync(tmpDir, { recursive: true, force: true });

  if (DRY_RUN) {
    log("\n✔️  Dry run completed. No files were modified.");
    return;
  }

  const installCmd = detectPackageManager();

  log(`\n📥 Installing dependencies (${installCmd})...`);

  execSync(installCmd, {
    cwd: CWD,
    stdio: "inherit",
  });

  log(
    "\n✅ Upgrade completed successfully!\n" +
      "   Please review the changes with 'git diff'.\n" +
      "   If you modified framework files, you may need to reapply\n" +
      "   your custom changes.\n"
  );
}

main();
