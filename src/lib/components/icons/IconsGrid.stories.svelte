<script module lang="ts">
  import type { ComponentProps } from "svelte";

  import { defineMeta } from "@storybook/addon-svelte-csf";

  import IconsGrid from "./IconsGrid.svelte";

  const { Story } = defineMeta({
    title: "Icons/IconsGrid",
    component: IconsGrid,
    render: template,
    tags: ["autodocs"],
    argTypes: {},
  });

  const generateIcons = (count: number) =>
    Array.from({ length: count }, (_, i) => {
      const num = i + 1;
      const color = `hsl(${Math.floor(Math.random() * 360)}, 70%, 60%)`;

      const svg = encodeURIComponent(`
        <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64">
          <rect width="64" height="64" fill="${color}" rx="10" />
          <text x="50%" y="50%" text-anchor="middle" dy=".35em" font-size="18" fill="white" font-family="Arial">
            ${num}
          </text>
        </svg>
      `);

      return {
        name: `Icon ${num}`,
        path: "#",
        src: `data:image/svg+xml,${svg}`,
      };
    });
</script>

{#snippet template(args: ComponentProps<typeof IconsGrid>)}
  <div style="width: 30rem; height: 500px">
    <IconsGrid {...args} />
  </div>
{/snippet}

<Story
  name="Default"
  args={{
    items: generateIcons(5000),
    columns: 6,
    itemHeight: 64,
    onClickIcon: () => {},
  }}
/>
