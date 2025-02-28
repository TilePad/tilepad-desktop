<script lang="ts">
  import type { Snippet, Component } from "svelte";
  import type { SvelteHTMLElements } from "svelte/elements";

  import { page } from "$app/state";

  type Props = {
    icon: Component<SvelteHTMLElements["svg"]>;
    href: string;
    title: string;

    end?: Snippet;
  };

  const { icon: Icon, href, title }: Props = $props();
</script>

<a
  {href}
  class="button"
  class:button--selected={href === "/"
    ? page.route.id === href
    : page.route.id?.startsWith(href)}
>
  <Icon class="icon" />
  <div class="content">
    <p class="title">{title}</p>
  </div>
</a>

<style>
  .button {
    display: block;
    padding: 0.5rem;
    border: 1px solid #2f2f2f;
    text-decoration: none;
    background-color: #322e38;
    border-radius: 0.5rem;
    display: flex;
    gap: 0.75rem;
    align-items: center;
    transition:
      background-color 0.25s ease,
      transform 0.15s ease;
  }

  .button:active {
    transform: scale(0.98);
  }

  .content {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
  }

  .button:global(> .icon) {
    color: white;
    width: 1.5rem;
    height: 1.5rem;
  }

  .button:hover {
    background-color: #453f4d;
  }

  .button--selected {
    background-color: #4e465a;
  }

  .button--selected:hover {
    background-color: #665a77;
  }

  .title {
    color: #fff;
    line-height: 1;
  }

  .text {
    color: #ccc;
    font-size: 0.8rem;
    line-height: 1;
  }
</style>
