export const foldersKeys = {
  root: ["folders"],
  list: (profileId: ProfileId) => ["folders", "profile", profileId, "list"],

  specific: (profileId: ProfileId, folderId: FolderId | null) => [
    "folders",
    "profile",
    profileId,
    "folder",
    folderId,
  ],
};
