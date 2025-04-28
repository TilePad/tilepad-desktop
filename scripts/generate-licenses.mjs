import fs from 'fs';
import { exec } from 'child_process';

/**
 * Creates the markdown output
 * 
 * @param {*} jsonData The JSON licenses list 
 * @returns The formatted markdown output
 */
function createLicenseMarkdown(jsonData) {
    const groups = {};

    // Group modules by license
    jsonData.forEach(pkg => {
        const license = pkg.license ?? 'Unknown';
        if (!groups[license]) {
            groups[license] = [];
        }
        groups[license].push(pkg);
    });

    let markdown = "# Third party licenses\n";
    markdown += "This page lists the licenses of third party dependencies used by this project\n\n"

    // Create top level list of licenses
    markdown += "## Licenses\n"
    for (const [license, _] of Object.entries(groups)) {
        markdown += `- ${license}\n`;
    }

    // Create individual license type sections
    for (const [license, packages] of Object.entries(groups)) {
        markdown += `## ${license}\n\n`;
        packages.forEach(pkg => {
            markdown += `- [${pkg.name}](${pkg.repository}) - ${pkg.version}\n`
        });
        markdown += "\n---\n";
    }

    return markdown;
};

exec('cargo license --json', (error, stdout, stderr) => {
    if (error) {
        console.error("Failed to get license data", stderr ?? error.message);
        return;
    }

    const json = JSON.parse(stdout);
    const markdown = createLicenseMarkdown(json);
    fs.writeFileSync('THIRD_PARTY_LICENSES.md', markdown, 'utf8');
    console.log("generated THIRD_PARTY_LICENSES.md")
});