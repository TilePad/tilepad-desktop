import { queryClient } from "../client";
import { profilesKeys } from "./profiles.keys";

export function invalidateProfilesList() {
  queryClient.invalidateQueries({
    queryKey: profilesKeys.list,
    exact: false,
  });
}
