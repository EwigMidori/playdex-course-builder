import path from "node:path";
import { fileURLToPath } from "node:url";
import fs from "node:fs";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
var __dirname = path.dirname(fileURLToPath(import.meta.url));
var repoRoot = path.resolve(__dirname, "../..");
var assetUrlPrefixes = ["/research/", "/users/"];
var contentTypes = {
    ".json": "application/json; charset=utf-8",
    ".md": "text/markdown; charset=utf-8",
    ".mdx": "text/markdown; charset=utf-8",
    ".txt": "text/plain; charset=utf-8",
    ".jpg": "image/jpeg",
    ".jpeg": "image/jpeg",
    ".png": "image/png",
    ".pdf": "application/pdf"
};
function pipelineMiddleware() {
    var servePipelineFile = function (url, res) {
        var _a, _b;
        var requestPath = decodeURIComponent((_a = url === null || url === void 0 ? void 0 : url.split("?")[0]) !== null && _a !== void 0 ? _a : "");
        if (!assetUrlPrefixes.some(function (prefix) { return requestPath.startsWith(prefix); })) {
            return false;
        }
        var relativePath = requestPath.replace(/^\/+/, "");
        var filePath = path.resolve(repoRoot, relativePath);
        if (!filePath.startsWith("".concat(repoRoot).concat(path.sep)) && filePath !== repoRoot) {
            res.statusCode = 403;
            res.end("Forbidden");
            return true;
        }
        if (!fs.existsSync(filePath) || !fs.statSync(filePath).isFile()) {
            res.statusCode = 404;
            res.end("Not found");
            return true;
        }
        res.setHeader("Content-Type", (_b = contentTypes[path.extname(filePath).toLowerCase()]) !== null && _b !== void 0 ? _b : "application/octet-stream");
        fs.createReadStream(filePath).pipe(res);
        return true;
    };
    return {
        name: "playdex-pipeline-files",
        configureServer: function (server) {
            server.middlewares.use(function (req, res, next) {
                if (!servePipelineFile(req.url, res)) {
                    next();
                }
            });
        },
        configurePreviewServer: function (server) {
            server.middlewares.use(function (req, res, next) {
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
