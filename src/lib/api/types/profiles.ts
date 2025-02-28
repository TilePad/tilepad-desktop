import type { Uuid } from "./shared";

export type ProfileId = Uuid;

export interface ProfileModel {
  id: ProfileId;
  name: string;
  default: boolean;
  config: ProfileConfig;
  order: number;
}

export type ProfileConfig = object;

export type CreateProfile = Omit<ProfileModel, "id">;

export type UpdateProfile = Partial<Omit<ProfileModel, "id">>;
