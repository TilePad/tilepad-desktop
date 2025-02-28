import type { Uuid } from "./shared";
import type { ProfileId } from "./profiles";

export type FolderId = Uuid;

export interface FolderModel {
  id: FolderId;
  name: string;
  config: Partial<FolderConfig>;
  profile_id: ProfileId;
  default: boolean;
  order: number;
}

export interface FolderConfig {
  rows: number;
  columns: number;
}

export type CreateFolder = Omit<FolderModel, "id">;

export type UpdateFolder = Partial<Omit<FolderModel, "id">>;
