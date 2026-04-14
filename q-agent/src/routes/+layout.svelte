<script lang="ts">
  import '../app.css';
  import { settings } from '$lib/stores/ui';

  let { children } = $props();

  // Apply theme & accent class to <html>
  $effect(() => {
    document.documentElement.setAttribute('data-theme', $settings.theme);
    document.documentElement.className =
      `accent-${$settings.accent} font-${$settings.fontSize}`;
  });
</script>

{@render children()}

<style>
  :global(:root.font-sm)  { font-size: 13px; }
  :global(:root.font-md)  { font-size: 15px; }
  :global(:root.font-lg)  { font-size: 17px; }
</style>
