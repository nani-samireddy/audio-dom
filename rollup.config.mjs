import typescript from 'rollup-plugin-typescript2';
import wasm from 'rollup-plugin-wasm';
import path from 'path';

export default [
  // ESM build (for browser/CDN)
  {
    input: 'src/audio-dom-wrapper.ts',
    output: {
      file: 'dist/audio-dom.esm.js',
      format: 'es',
      sourcemap: true,
    },
    plugins: [
      wasm({ maxFileSize: 0 }), // Lazy load (fetch `.wasm` file)
      typescript({
        tsconfig: './tsconfig.json',
        useTsconfigDeclarationDir: true
      }),
    ],
  },

  // CJS build (for Node)
  {
    input: 'src/audio-dom-wrapper.ts',
    output: {
      file: 'dist/audio-dom.cjs.js',
      format: 'cjs',
      sourcemap: true,
    },
    plugins: [
      wasm({ maxFileSize: 0 }),
      typescript({
        tsconfig: './tsconfig.json',
        useTsconfigDeclarationDir: true
      }),
    ],
  }
];
