import { readFileSync, writeFileSync } from "fs";

const files = [
  "../normal/cargo.toml",
  "../weak/cargo.toml",
  "../package.json",
  "../README.md"
];
const NEW_VERSION: string = "2.0.0"

files.forEach(file => {
  const content = readFileSync(__dirname + "/" + file, "utf8");
  let newContent = content
  if (file.includes("json")) {
    newContent = content.replace(/\"version\": \".*\"/, `"version": "${NEW_VERSION}"`);
  } else {
    newContent = content.replace(/version = "[0-9.]+"/, `version = "${NEW_VERSION}"`);
  }
  writeFileSync(__dirname + "/" + file, newContent, "utf8");
})




