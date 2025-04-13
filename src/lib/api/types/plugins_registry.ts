export interface PluginRegistryEntry {
  id: string;
  name: string;
  authors: string[];
  description: string;
  repo: string;
}

export interface GithubRelease {
  tag_name: string;
  name: string;
  created_at: string;
  published_at: string;
  assets: GithubReleaseAsset[];
}

export interface GithubReleaseAsset {
  name: string;
  download_count: number;
  browser_download_url: string;
}
