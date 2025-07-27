<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import SkeletonList from "./SkeletonList.svelte";

  const { Story } = defineMeta({
    title: "Skeleton/SkeletonList",
    component: SkeletonList,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story name="Default" />
