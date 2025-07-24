<script lang="ts">
  type Props = {
    /** Size in pixels of the spinner */
    size?: number;
  };

  const { size = 20 }: Props = $props();
</script>

<span
  class="loading-spinner"
  style:width={`${size}px`}
  style:height={`${size}px`}
></span>

<style>
  .loading-spinner {
    display: inline-block;
    border: 2px solid #777;
    border-radius: 50%;
    border-top-color: white;
    animation: spin 1s ease-in-out infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
