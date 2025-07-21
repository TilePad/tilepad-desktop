export const iconRegistryKey = {
  root: ["icon-registry"],
  list: ["icon-registry", "list"],
  specificManifest: (repo: string) => ["icon-registry", "manifest", repo],
  specificReadme: (repo: string) => ["icon-registry", "readme", repo],
};
