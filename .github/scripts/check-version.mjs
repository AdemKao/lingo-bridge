import { readFileSync, existsSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join, resolve } from "node:path";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const load = (file) => readFileSync(join(root, file), "utf8");
const npm = JSON.parse(load("apps/desktop/package.json")).version;
const tauri = JSON.parse(load("apps/desktop/src-tauri/tauri.conf.json")).version;
const cargo = load("Cargo.toml");
const workspace = cargo.match(/\[workspace\.package\][\s\S]*?\bversion\s*=\s*"([^"]+)"/)?.[1];
const versions = { npm, tauri, workspace };
const fail = (message) => { console.error("Version check failed: " + message); process.exitCode = 1; };

if (!Object.values(versions).every((value) => value === npm)) {
  fail("Cargo workspace, npm and Tauri versions differ: " + JSON.stringify(versions));
}
if (!/^\d+\.\d+\.\d+(?:-alpha\.\d+|-beta\.\d+|-rc\.\d+)?$/.test(npm)) {
  fail("Expected an explicit SemVer (e.g. 0.1.0-alpha.1); got " + npm);
}
const tag = "v" + npm;
for (const path of [
  "CHANGELOG.md",
  "CHANGELOG.zh-TW.md",
  "docs/releases/" + tag + ".md",
  "docs/releases/" + tag + ".zh-TW.md",
]) {
  if (!existsSync(join(root, path))) {
    fail("Missing versioned file " + path);
    continue;
  }
  if (!load(path).includes(npm)) fail("Version not mentioned in " + path);
}

if (!process.exitCode) console.log("Version check OK: " + tag);
