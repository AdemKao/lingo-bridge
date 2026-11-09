import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const load = (name) => fs.readFileSync(path.join(root, name), "utf8");
const version = load("VERSION").trim();
if (!/^\d+\.\d+\.\d+(?:-(?:alpha|beta|rc)\.\d+)?$/.test(version)) {
  throw new Error("Invalid VERSION string: " + version);
}
const pkg = JSON.parse(load("apps/desktop/package.json"));
const tauri = JSON.parse(load("apps/desktop/src-tauri/tauri.conf.json"));
const cargo = load("Cargo.toml");
const cargoVersion = cargo.match(/\[workspace\.package\][\s\S]*?^version = "([^"]+)"/m)?.[1];
const sources = {
  "VERSION": version,
  "Cargo workspace": cargoVersion,
  "npm package": pkg.version,
  "Tauri": tauri.version,
};
for (const [source, actual] of Object.entries(sources)) {
  if (actual !== version) {
    throw new Error(source + " version mismatch: " + actual + " != " + version);
  }
}
const tag = process.argv.find((arg) => arg.startsWith("--tag="))?.slice(6);
if (tag && tag !== "v" + version) {
  throw new Error("Requested release tag " + tag + " does not match v" + version);
}
const releaseNotes = "docs/releases/v" + version + ".md";
if (!fs.existsSync(path.join(root, releaseNotes))) {
  throw new Error("Missing release notes: " + releaseNotes);
}
console.log("Version verified: v" + version + " in Cargo, npm, Tauri and release notes.");
