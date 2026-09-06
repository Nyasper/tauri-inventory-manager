import { crossfade } from "svelte/transition";

export const [send, receive] = crossfade({
  duration: 240,
  fallback: (node) => {
    const style = getComputedStyle(node);
    const transform = style.transform === "none" ? "" : style.transform;
    return {
      duration: 240,
      easing: (t) => t,
      css: (t) => `
        transform: ${transform} scale(${t});
        opacity: ${t};
      `,
    };
  },
});