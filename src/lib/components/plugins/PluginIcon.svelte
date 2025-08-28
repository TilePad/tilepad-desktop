<script lang="ts">
  import missingLogo from "$lib/assets/missing-icon.png";

  type Props = {
    icon: string | null;
    name: string;
    height: number;
  };

  const { icon, name, height }: Props = $props();
  let bgColor: string = $state(
    "radial-gradient(circle at 50% 50%, rgba(200,200,255,0.2), rgba(200,200,255,0.1))",
  );
  let imgEl: HTMLImageElement | undefined = $state();

  $effect(() => {
    const currentImg = imgEl;

    if (!currentImg) {
      return;
    }

    if (currentImg.complete) {
      extractColor(currentImg);
    } else {
      currentImg.onload = () => extractColor(currentImg);
    }
  });

  /**
   * Handles extracting a color to use for the background color
   * from the provided icon image. This calculates and sets the
   * background to an average of the colors in the image
   *
   * @param imgEl Icon image element
   */
  function extractColor(imgEl: HTMLImageElement) {
    const canvas = document.createElement("canvas");
    canvas.width = imgEl.naturalWidth;
    canvas.height = imgEl.naturalHeight;

    const ctx = canvas.getContext("2d");
    if (ctx === null) return;

    ctx.drawImage(imgEl, 0, 0);

    const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
    const data = imageData.data;

    let r = 0;
    let g = 0;
    let b = 0;
    let count = 0;

    for (let i = 0; i < data.length; i += 4) {
      const alpha = data[i + 3];
      if (alpha > 0) {
        // ignore fully transparent pixels
        r += data[i];
        g += data[i + 1];
        b += data[i + 2];
        count++;
      }
    }

    r = Math.round(r / count);
    g = Math.round(g / count);
    b = Math.round(b / count);

    bgColor = `radial-gradient(circle at 50% 50%, rgba(${r}, ${g}, ${b}, 0.2), rgba(${r}, ${g}, ${b}, 0.1))`;
  }
</script>

<div class="bg" style:background={bgColor} style:height={`${height}px`}>
  {#if icon}
    <img
      class="icon"
      bind:this={imgEl}
      src={icon}
      alt="{name} Plugin Icon"
      crossorigin="anonymous"
    />
  {:else}
    <img
      class="icon"
      bind:this={imgEl}
      src={missingLogo}
      alt="{name} Plugin Icon"
      crossorigin="anonymous"
    />
  {/if}
</div>

<style>
  .icon {
    width: 64px;
    height: 64px;
    border-radius: 8px;
  }

  .bg {
    align-self: center;
    display: flex;
    justify-content: center;
    align-items: center;
    aspect-ratio: 1/1;
    border-radius: var(--tp-radius-md);
  }
</style>
