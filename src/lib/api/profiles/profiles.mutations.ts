export function createCreateProfileMutation() {
  return createMutation({
    mutationFn: ({ create }: { create: CreateProfile }) =>
      createProfile(create),
  });
}
