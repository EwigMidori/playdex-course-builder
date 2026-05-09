#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import sys

import requests


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Minimal Responses API image-url test using a requests-style payload."
    )
    parser.add_argument("--endpoint", required=True, help="Responses endpoint, e.g. https://co.yes.vg/v1/responses")
    parser.add_argument("--api-key", required=True, help="Bearer API key")
    parser.add_argument("--model", default="gpt-5.4", help="Model name")
    parser.add_argument("--prompt", default="描述这张图片中的内容", help="Prompt text")
    parser.add_argument("--image-url", required=True, help="Public image URL")
    parser.add_argument("--reasoning-effort", help="Optional reasoning.effort, e.g. high")
    return parser.parse_args()


def build_payload(args: argparse.Namespace) -> dict:
    payload = {
        "model": args.model,
        "input": [
            {
                "role": "user",
                "content": [
                    {"type": "input_text", "text": args.prompt},
                    {"type": "input_image", "image_url": args.image_url},
                ],
            }
        ],
    }
    if args.reasoning_effort:
        payload["reasoning"] = {"effort": args.reasoning_effort}
    return payload


def main() -> int:
    args = parse_args()
    payload = build_payload(args)
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {args.api_key}",
    }

    response = requests.post(
        args.endpoint,
        headers=headers,
        data=json.dumps(payload),
        timeout=120,
    )

    print(f"HTTP {response.status_code}")
    print(response.text)
    response.raise_for_status()
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except requests.RequestException as exc:
        print(f"REQUEST_ERROR: {exc}", file=sys.stderr)
        raise
