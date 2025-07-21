import type { ProfileId } from "../types/profiles";

export const profilesKeys = {
  root: ["profiles"],
  list: ["profiles", "list"],
  specific: (profileId: ProfileId | null) => ["profiles", "profile", profileId],
};
