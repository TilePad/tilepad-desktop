import type { ProfileId } from "./profiles";
import type { Uuid } from "./shared";

export type FolderId = Uuid;

export interface FolderModel {
  id: FolderId;
  name: string;
  config: FolderConfig;
  profile_id: ProfileId;
  default: boolean;
  order: number;
}

export interface FolderConfig {}

export type CreateFolder = Omit<FolderModel, "id">;

export type UpdateFolder = Partial<Omit<FolderModel, "id">>;
