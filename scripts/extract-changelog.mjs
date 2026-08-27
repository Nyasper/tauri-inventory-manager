import { readFileSync } from "node:fs";

const tag = process.env.GITHUB_REF_NAME ?? process.argv[2] ?? "";
const version = tag.replace(/^v/, "");

const content = readFileSync("CHANGELOG.md", "utf8");
const lines = content.split(/\r?\n/);

const header = `## [${version}]`;
let start = -1;

for (let i = 0; i < lines.length; i += 1) {
  if (lines[i].startsWith(header)) {
    start = i;
    break;
  }
}

if (start === -1) {
  console.log(`## ${tag}`);
  console.log("");
  console.log("See CHANGELOG.md for details.");
  process.exit(0);
}

const body = [];
for (let i = start + 1; i < lines.length; i += 1) {
  if (/^## /.test(lines[i])) break;
  body.push(lines[i]);
}

console.log(body.join("\n").trim());
