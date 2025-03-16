<script lang="ts">
  import type { Snippet, Component } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";

  import SolarInfoCircleBoldDuotone from "~icons/solar/info-circle-bold-duotone";
  import SolarDangerTriangleBoldDuotone from "~icons/solar/danger-triangle-bold-duotone";

  type Props = {
    severity: "success" | "info" | "warning" | "error" | "tip";
    title?: string;
    icon?: Component;
    children?: Snippet;
  } & HTMLAttributes<HTMLDivElement>;

  const defaultTitle: Partial<Record<string, string>> = {
    success: "SUCCESS",
    info: "INFO",
    warning: "WARNING",
    error: "ERROR",
    tip: "TIP",
  };

  const defaultIcon: Partial<Record<string, Component>> = {
    success: SolarInfoCircleBoldDuotone,
    info: SolarInfoCircleBoldDuotone,
    warning: SolarDangerTriangleBoldDuotone,
    error: SolarDangerTriangleBoldDuotone,
    tip: SolarInfoCircleBoldDuotone,
  };

  const {
    severity,
    title = defaultTitle[severity],
    icon: Icon = defaultIcon[severity],
    children,
    ...divAttr
  }: Props = $props();
</script>

<div {...divAttr} class="aside" data-severity={severity}>
  <span class="aside__title">
    {#if Icon}
      <span class="aside__title__icon">
        <Icon />
      </span>
    {/if}

    {title}
  </span>

  <div class="aside__content">
    {@render children?.()}
  </div>
</div>

<style>
  .aside {
    display: inline-block;
    margin: 0 0.2rem;
    background-color: #222;
    border: 1px solid #111;
    color: #fff;
    font-size: 1rem;
    font-weight: normal;

    border-radius: 0.25rem;
    padding: 0.25rem 0.75rem;
    padding-bottom: 1rem;

    vertical-align: middle;
  }

  .aside__content {
    color: #fff;
  }

  .aside__title {
    font-weight: bold;
    display: flex;
    gap: 0.25rem;
    padding: 0.25rem 0;
  }

  .aside[data-severity="tip"] {
    border-color: #dd82f0;
    background-color: #3c1b42;
    color: #dd82f0;
  }

  .aside[data-severity="error"] {
    border-color: #f08282;
    background-color: #421b1b;
    color: #f08282;
  }

  .aside[data-severity="warning"] {
    border-color: #eef082;
    background-color: #423f1b;
    color: #f0ee82;
  }

  .aside[data-severity="success"] {
    border-color: #a1f082;
    background-color: #1b421b;
    color: #a1f082;
  }

  .aside[data-severity="info"] {
    border-color: #82bbf0;
    background-color: #1b2f42;
    color: #82bbf0;
  }
</style>
