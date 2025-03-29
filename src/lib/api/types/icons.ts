export type IconPackId = string;

export interface IconPack {
  path: string;
  manifest: IconPackManifest;
  icons: Icon[];
}

export interface Icon {
  name: string;
  path: string;
}

export interface IconPackManifest {
  icons: IconsManifest;
}

export interface IconsManifest {
  id: IconPackId;
  name: string;
  version: string;
  authors: string[];
  description: string | null;
  icon: string | null;
}
