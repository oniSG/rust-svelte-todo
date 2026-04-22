import { defineConfig } from 'orval';

export default defineConfig({
    myApi: {
        input: {
            target: 'http://localhost:3000/openapi.json', // your API's openapi.json URL
        },
        output: {
            mode: 'tags-split',         // one file per tag/resource
            target: 'src/lib/api/generated',      // where generated files go
            client: 'svelte-query',     // generates TanStack svelte-query hooks
            httpClient: 'fetch',        // use native fetch
            clean: true,                // clean output dir before regenerating
            override: {
                mutator: {
                    path: 'src/lib/api/mutator.ts',
                    name: 'customFetch',
                },
            },
        },
    },
});