import { readFileSync, writeFileSync } from "fs";
import { join, resolve } from "path";

const configPath = join(resolve(), "src-tauri/tauri.conf.json");
const config = JSON.parse(readFileSync(configPath, "utf8"));

// Replace the secret
config.plugins.oauth.google.clientId = process.env.TAURI_GOOGLE_CLIENT_ID;
config.plugins.oauth.google.clientSecret = process.env.TAURI_GOOGLE_CLIENT_SECRET;

// Write back to file
writeFileSync(configPath, JSON.stringify(config, null, 2));
