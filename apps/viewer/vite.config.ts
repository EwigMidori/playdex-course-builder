import path from "node:path";
import { fileURLToPath } from "node:url";
import fs from "node:fs";
import type { ServerResponse } from "node:http";
import react from "@vitejs/plugin-react";
import { defineConfig, type Plugin } from "vite";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, "../..");
const pipelineRoot = path.join(repoRoot, "research", "pipeline");

const contentTypes: Record<string, string> = {
  ".json": "application/json; charset=utf-8",
  ".md": "text/markdown; charset=utf-8",
  ".mdx": "text/markdown; charset=utf-8",
  ".txt": "text/plain; charset=utf-8",
  ".jpg": "image/jpeg",
  ".jpeg": "image/jpeg",
  ".png": "image/png",
  ".pdf": "application/pdf"
};

function pipelineMiddleware(): Plugin {
  const servePipelineFile = (url: string | undefined, res: ServerResponse) => {
    if (!url?.startsWith("/research/pipeline/")) {
      return false;
    }

    const requestPath = decodeURIComponent(url.split("?")[0] ?? "");
    const relativePath = requestPath.replace(/^\/research\/pipeline\/?/, "");
    const filePath = path.resolve(pipelineRoot, relativePath);

    if (!filePath.startsWith(`${pipelineRoot}${path.sep}`) && filePath !== pipelineRoot) {
      res.statusCode = 403;
      res.end("Forbidden");
      return true;
    }

    if (!fs.existsSync(filePath) || !fs.statSync(filePath).isFile()) {
      res.statusCode = 404;
      res.end("Not found");
      return true;
    }

    res.setHeader("Content-Type", contentTypes[path.extname(filePath).toLowerCase()] ?? "application/octet-stream");
    fs.createReadStream(filePath).pipe(res);
    return true;
  };

  return {
    name: "playdex-pipeline-files",
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        if (!servePipelineFile(req.url, res)) {
          next();
        }
      });
    },
    configurePreviewServer(server) {
      server.middlewares.use((req, res, next) => {
        if (!servePipelineFile(req.url, res)) {
          next();
        }
      });
    }
  };
}

export default defineConfig({
  plugins: [pipelineMiddleware(), react()],
  server: {
    fs: {
      allow: [repoRoot]
    }
  }
});
