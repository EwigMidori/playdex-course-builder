#!/usr/bin/env python3
"""Run one MinerU precision extract job with the raw API.

Usage:
    ./.venv/bin/python scripts/mineru_raw_convert.py \
        pipeline/0-raw/L1.pdf \
        pipeline/1-plain

The script keeps the operational contract narrow:
- Basic inputs are CLI args.
- Tunables are environment variables, optionally loaded from .env.
- Local side effects stay under the current working directory.
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import sys
import time
import zipfile
from pathlib import Path
from typing import Any

import requests


class MineruRawConvertCli:
    def load_dotenv(dotenv_path: Path) -> None:
        if not dotenv_path.exists():
            return
        for raw_line in dotenv_path.read_text(encoding="utf-8").splitlines():
            line = raw_line.strip()
            if not line or line.startswith("#") or "=" not in line:
                continue
            key, value = line.split("=", 1)
            key = key.strip()
            value = value.strip().strip("\"'")
            if key and key not in os.environ:
                os.environ[key] = value


    def env_str(name: str, default: str) -> str:
        return os.environ.get(name, default).strip() or default


    def env_int(name: str, default: int) -> int:
        value = os.environ.get(name)
        if value is None or not value.strip():
            return default
        return int(value)


    def env_bool(name: str, default: bool) -> bool:
        value = os.environ.get(name)
        if value is None or not value.strip():
            return default
        return value.strip().lower() in {"1", "true", "yes", "on"}


    def parse_args() -> argparse.Namespace:
        parser = argparse.ArgumentParser(
            description="Convert one local document through MinerU raw precision API."
        )
        parser.add_argument("input_file", type=Path, help="Local file to convert.")
        parser.add_argument(
            "output_root",
            type=Path,
            help="Root directory that will receive <input_stem>/ as the normalized output folder.",
        )
        parser.add_argument(
            "--output-name",
            default=None,
            help="Override the output directory name. Defaults to the input stem.",
        )
        parser.add_argument(
            "--force",
            action="store_true",
            help="Allow writing into an existing non-empty output directory.",
        )
        return parser.parse_args()


    def read_token() -> str:
        inline = os.environ.get("MINERU_API_TOKEN", "").strip()
        if inline:
            return inline

        token_file = Path(MineruRawConvertCli.env_str("MINERU_TOKEN_FILE", "mineru_token.txt"))
        if token_file.exists():
            token = token_file.read_text(encoding="utf-8").strip()
            if token:
                return token
        raise RuntimeError("Missing API token. Set MINERU_API_TOKEN or MINERU_TOKEN_FILE.")


    def resolve_output_dir(input_file: Path, output_root: Path, output_name: str | None) -> Path:
        leaf = output_name or input_file.stem
        return output_root / leaf


    def assert_paths(input_file: Path, output_dir: Path, force: bool) -> None:
        if not input_file.exists():
            raise FileNotFoundError(f"Input file not found: {input_file}")
        if not input_file.is_file():
            raise ValueError(f"Input path is not a file: {input_file}")
        if output_dir.exists() and any(output_dir.iterdir()) and not force:
            raise FileExistsError(
                f"Output directory already exists and is not empty: {output_dir}. "
                "Use --force to reuse it."
            )
        output_dir.mkdir(parents=True, exist_ok=True)


    def session_with_headers(token: str) -> requests.Session:
        session = requests.Session()
        session.headers.update(
            {
                "Authorization": f"Bearer {token}",
                "Accept": "application/json",
            }
        )
        return session


    def request_upload_url(
        session: requests.Session,
        base_url: str,
        input_file: Path,
    ) -> tuple[str, str]:
        submit_url = f"{base_url.rstrip('/')}/api/v4/file-urls/batch"
        payload = {
            "enable_formula": MineruRawConvertCli.env_bool("MINERU_ENABLE_FORMULA", True),
            "language": MineruRawConvertCli.env_str("MINERU_LANGUAGE", "ch"),
            "enable_table": MineruRawConvertCli.env_bool("MINERU_ENABLE_TABLE", True),
            "model_version": MineruRawConvertCli.env_str("MINERU_MODEL_VERSION", "vlm"),
            "files": [
                {
                    "name": input_file.name,
                    "is_ocr": MineruRawConvertCli.env_bool("MINERU_IS_OCR", True),
                    "data_id": input_file.stem,
                }
            ],
        }
        response = session.post(
            submit_url,
            json=payload,
            timeout=MineruRawConvertCli.env_int("MINERU_REQUEST_TIMEOUT_SECONDS", 120),
            headers={"Content-Type": "application/json"},
        )
        response.raise_for_status()
        result = response.json()
        if result.get("code") != 0:
            raise RuntimeError(f"Create upload URL failed: {result.get('msg', 'unknown error')}")

        data = result.get("data", {})
        batch_id = data.get("batch_id")
        file_urls = data.get("file_urls") or data.get("files") or []
        if not batch_id or not file_urls:
            raise RuntimeError(f"Unexpected response payload: {result}")
        return batch_id, file_urls[0]


    def upload_file(upload_url: str, input_file: Path) -> None:
        with input_file.open("rb") as handle:
            response = requests.put(
                upload_url,
                data=handle,
                timeout=MineruRawConvertCli.env_int("MINERU_UPLOAD_TIMEOUT_SECONDS", 600),
            )
        response.raise_for_status()


    def poll_result(
        session: requests.Session,
        base_url: str,
        batch_id: str,
        input_file: Path,
    ) -> dict[str, Any]:
        poll_url = f"{base_url.rstrip('/')}/api/v4/extract-results/batch/{batch_id}"
        timeout_seconds = MineruRawConvertCli.env_int("MINERU_RESULT_TIMEOUT_SECONDS", 3600)
        poll_interval = MineruRawConvertCli.env_int("MINERU_POLL_INTERVAL_SECONDS", 15)
        deadline = time.time() + timeout_seconds

        while True:
            response = session.get(
                poll_url,
                timeout=MineruRawConvertCli.env_int("MINERU_REQUEST_TIMEOUT_SECONDS", 120),
                headers={"Content-Type": "application/json"},
            )
            response.raise_for_status()
            result = response.json()
            if result.get("code") != 0:
                raise RuntimeError(f"Poll failed: {result.get('msg', 'unknown error')}")

            extract_results = result.get("data", {}).get("extract_result") or []
            if extract_results:
                current = extract_results[0]
                state = current.get("state")
                if state == "done":
                    return current
                if state == "failed":
                    raise RuntimeError(current.get("err_msg") or f"Extract failed for {input_file.name}")

                progress = current.get("extract_progress") or {}
                extracted_pages = progress.get("extracted_pages")
                total_pages = progress.get("total_pages")
                if extracted_pages is not None and total_pages is not None:
                    print(f"[mineru] state={state} pages={extracted_pages}/{total_pages}")
                else:
                    print(f"[mineru] state={state}")

            if time.time() >= deadline:
                raise TimeoutError(f"Timed out waiting for batch {batch_id}")
            time.sleep(poll_interval)


    def download_zip(zip_url: str, archive_path: Path) -> None:
        with requests.get(
            zip_url,
            stream=True,
            timeout=MineruRawConvertCli.env_int("MINERU_DOWNLOAD_TIMEOUT_SECONDS", 600),
        ) as response:
            response.raise_for_status()
            with archive_path.open("wb") as handle:
                for chunk in response.iter_content(chunk_size=1024 * 1024):
                    if chunk:
                        handle.write(chunk)


    def extract_archive(archive_path: Path, extracted_dir: Path) -> None:
        with zipfile.ZipFile(archive_path) as archive:
            archive.extractall(extracted_dir)


    def pick_markdown(extracted_dir: Path) -> Path | None:
        full_md = sorted(extracted_dir.rglob("full.md"))
        if full_md:
            return full_md[0]
        any_md = sorted(extracted_dir.rglob("*.md"))
        if any_md:
            return any_md[0]
        return None


    def write_job_metadata(
        output_dir: Path,
        batch_id: str,
        extract_result: dict[str, Any],
        input_file: Path,
    ) -> None:
        metadata = {
            "input_file": str(input_file),
            "batch_id": batch_id,
            "output_dir": str(output_dir),
            "model_version": MineruRawConvertCli.env_str("MINERU_MODEL_VERSION", "vlm"),
            "language": MineruRawConvertCli.env_str("MINERU_LANGUAGE", "ch"),
            "enable_formula": MineruRawConvertCli.env_bool("MINERU_ENABLE_FORMULA", True),
            "enable_table": MineruRawConvertCli.env_bool("MINERU_ENABLE_TABLE", True),
            "is_ocr": MineruRawConvertCli.env_bool("MINERU_IS_OCR", True),
            "result": extract_result,
        }
        (output_dir / "job.json").write_text(
            json.dumps(metadata, ensure_ascii=False, indent=2),
            encoding="utf-8",
        )


    def main() -> int:
        MineruRawConvertCli.load_dotenv(Path(".env"))
        args = MineruRawConvertCli.parse_args()

        input_file = args.input_file.resolve()
        output_root = args.output_root.resolve()
        output_dir = MineruRawConvertCli.resolve_output_dir(input_file, output_root, args.output_name).resolve()
        MineruRawConvertCli.assert_paths(input_file, output_dir, args.force)

        token = MineruRawConvertCli.read_token()
        base_url = MineruRawConvertCli.env_str("MINERU_API_BASE_URL", "https://mineru.net")
        session = MineruRawConvertCli.session_with_headers(token)

        print(f"[mineru] input={input_file}")
        print(f"[mineru] output_dir={output_dir}")
        print(f"[mineru] model_version={MineruRawConvertCli.env_str('MINERU_MODEL_VERSION', 'vlm')}")

        batch_id, upload_url = MineruRawConvertCli.request_upload_url(session, base_url, input_file)
        print(f"[mineru] batch_id={batch_id}")
        print("[mineru] upload=started")
        MineruRawConvertCli.upload_file(upload_url, input_file)
        print("[mineru] upload=done")

        extract_result = MineruRawConvertCli.poll_result(session, base_url, batch_id, input_file)
        full_zip_url = extract_result.get("full_zip_url")
        if not full_zip_url:
            raise RuntimeError(f"Missing full_zip_url in result: {extract_result}")

        archive_path = output_dir / "result.zip"
        extracted_dir = output_dir / "artifacts"
        if extracted_dir.exists():
            shutil.rmtree(extracted_dir)
        extracted_dir.mkdir(parents=True, exist_ok=True)

        MineruRawConvertCli.download_zip(full_zip_url, archive_path)
        MineruRawConvertCli.extract_archive(archive_path, extracted_dir)
        MineruRawConvertCli.write_job_metadata(output_dir, batch_id, extract_result, input_file)

        markdown_path = MineruRawConvertCli.pick_markdown(extracted_dir)
        plain_path = output_dir / "plain.txt"
        if markdown_path is not None:
            shutil.copyfile(markdown_path, plain_path)
        else:
            plain_path.write_text("", encoding="utf-8")

        plain_size = plain_path.stat().st_size
        print(f"[mineru] archive={archive_path}")
        print(f"[mineru] artifacts={extracted_dir}")
        print(f"[mineru] plain_text={plain_path}")
        print(f"[mineru] plain_text_bytes={plain_size}")

        if MineruRawConvertCli.env_bool("MINERU_FAIL_ON_EMPTY_TEXT", False) and plain_size == 0:
            raise RuntimeError("Conversion completed but no markdown/plain text was found in the zip.")
        return 0


if __name__ == "__main__":
    try:
        raise SystemExit(MineruRawConvertCli.main())
    except KeyboardInterrupt:
        raise SystemExit(130)
    except Exception as exc:
        print(f"[mineru] error: {exc}", file=sys.stderr)
        raise SystemExit(1)
