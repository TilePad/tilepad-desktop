export const pluginRegistryKey = {
  root: ["plugin-registry"],
  list: ["plugin-registry", "list"],
  specificManifest: (repo: string) => ["plugin-registry", "manifest", repo],
  specificReadme: (repo: string) => ["plugin-registry", "readme", repo],
};
