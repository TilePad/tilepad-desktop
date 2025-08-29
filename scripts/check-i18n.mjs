import fs from "node:fs";

// Load the two translation files
const baseFile = JSON.parse(
  fs.readFileSync("src/lib/i18n/locales/en.json", "utf8"),
);

const languages = ["cs", "de", "es", "fr"];

/**
 * Recursively find missing keys in an object
 *
 * @param {*} base
 * @param {*} compare
 * @param {*} path
 * @returns {String[]}
 */
function findMissingKeys(base, compare, path = "") {
  const missing = [];

  for (const key in base) {
    const fullPath = path ? `${path}.${key}` : key;

    if (!(key in compare)) {
      missing.push(fullPath);
    } else if (typeof base[key] === "object" && base[key] !== null) {
      // Recursively check nested objects
      missing.push(...findMissingKeys(base[key], compare[key], fullPath));
    }
  }

  return missing;
}

for (const language of languages) {
  const compareFile = JSON.parse(
    fs.readFileSync("src/lib/i18n/locales/" + language + ".json", "utf8"),
  );

  const missingKeys = findMissingKeys(baseFile, compareFile);

  if (missingKeys.length > 0) {
    console.log(language + "] Missing keys:");
    missingKeys.forEach((key) => console.log("- " + key));
  } else {
    console.log(language + "] No missing keys");
  }
}
