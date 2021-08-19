import svelte from "rollup-plugin-svelte";
import commonjs from "@rollup/plugin-commonjs";
import resolve from "@rollup/plugin-node-resolve";
import css from "rollup-plugin-css-only";
import autoPreprocess from "svelte-preprocess";
import typescript from "@rollup/plugin-typescript";

export default {
  input: "web/svelte/main.js",
  output: {
    sourcemap: true,
    format: "iife",
    name: "app",
    file: "web/compiled_public/bundle.js",
  },
  plugins: [
    svelte({
      preprocess: autoPreprocess(),
    }),
    typescript({ 
      module: "ESNext",
      include: ["web/svelte/main.js"]
    }),
    css({ output: "bundle.css" }),
    resolve({
      browser: true,
      dedupe: ["svelte"],
    }),
    commonjs(),
  ],
};
