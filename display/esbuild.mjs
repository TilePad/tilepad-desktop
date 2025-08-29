import * as esbuild from "esbuild";

await esbuild.build({
  entryPoints: ["./src/display"],
  bundle: true,
  outfile: "dist/display.js",
  minify: true,
});
