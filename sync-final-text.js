#!/usr/bin/env node

// Regenerate the `finalText` value of an edit trace from its edits
const path = require("path");
const fs = require("fs");
// Apply the paper editing trace to a regular JavaScript array (using .splice, one char at a time)
const traceFileName = process.argv[2];
if (!traceFileName) {
  throw new Error(`Usage: sync-final-text.js trace_file_name`);
}
const { edits, finalText = null } = require(path.resolve(traceFileName));

const chars = [];
for (let edit of edits) chars.splice(...edit);
const generated = chars.join("");
if (generated === finalText) {
  console.log("finalText was already up to date, doing nothing");
  process.exit(0);
} else {
  console.log("Writing out new finalText...");
  fs.writeFileSync(
    traceFileName,
    JSON.stringify({ edits, finalText: generated })
  );
}
