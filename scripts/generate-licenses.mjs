import fs from 'fs';
import { promisify } from 'util';
import { exec } from 'child_process';

const execAsync = promisify(exec);


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

    markdown += '\n---\n\n'

    // Create individual license type sections
    for (const [license, packages] of Object.entries(groups)) {
        markdown += `## ${license}\n\n`;
        packages.forEach(pkg => {
            markdown += `- [${pkg.name}](${pkg.repository}) - ${pkg.version}\n`
        });
        markdown += "\n---\n\n";
    }

    return markdown;
};

async function getNpmLicenses() {
    const { stdout } = await execAsync('npm ls --json --long');
    const data = JSON.parse(stdout);

    const result = [];

    function collectDependencies(deps) {
        if (!deps) return;
        for (const [name, info] of Object.entries(deps)) {
            if (info && info.version) {
                result.push({
                    name,
                    version: info.version,
                    license: info.license ?? 'Unknown',
                    repository: info.repository?.url?.replace(/^git\+/, '').replace(/\.git$/, '') ?? ''
                });
                collectDependencies(info.dependencies);
            }
        }
    }

    collectDependencies(data.dependencies);
    return result;
}

async function getCargoLicenses() {
    const { stdout } = await execAsync('cargo license --json');
    const data = JSON.parse(stdout);

    return data.map(pkg => ({
        name: pkg.name,
        version: pkg.version,
        license: pkg.license ?? 'Unknown',
        repository: pkg.repository ?? ''
    }));
}


async function main() {
    try {
        const [npmLicenses, cargoLicenses] = await Promise.all([
            getNpmLicenses(),
            getCargoLicenses()
        ]);

        const allLicenses = [...npmLicenses, ...cargoLicenses];
        const markdown = createLicenseMarkdown(allLicenses);
        fs.writeFileSync('THIRD_PARTY_LICENSES.md', markdown, 'utf8');
        console.log("Generated THIRD_PARTY_LICENSES.md");
    } catch (err) {
        console.error("Failed to collect license data:", err);
    }
}

main();