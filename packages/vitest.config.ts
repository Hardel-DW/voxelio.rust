import { defineConfig } from 'vitest/config';

export default defineConfig({
    esbuild: {
        target: 'ES2023',
    },
    test: {
        globals: true,
    },
});