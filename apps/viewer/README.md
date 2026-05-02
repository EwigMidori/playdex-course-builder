# Playdex Pipeline Viewer

Debug viewer for existing `research/pipeline` artifacts. It reads files in place through Vite and does not copy or generate pipeline data.

## Commands

```sh
npm install
npm run dev -- --host 127.0.0.1
npm run build
```

The viewer reads `research/pipeline/course-index.json` and opens the indexed course chapters directly from the in-place pipeline assets.
