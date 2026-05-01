# Playdex Pipeline Viewer

Debug viewer for existing `research/pipeline` artifacts. It reads files in place through Vite and does not copy or generate pipeline data.

## Commands

```sh
npm install
npm run dev -- --host 127.0.0.1
npm run build
```

The default lesson is `L2`. `L1` is available with graceful fallback for the older guided story layout.
Use the `Stress` data mode in the app to synthesize 30+ rows per list for scrolling and pagination testing.
