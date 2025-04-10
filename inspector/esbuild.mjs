import * as esbuild from 'esbuild'

await esbuild.build({
    entryPoints: ['./src/inspector'],
    bundle: true,
    outfile: 'dist/inspector.js',
    minify: true,
})