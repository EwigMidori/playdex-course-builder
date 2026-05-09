#!/usr/bin/env python3
from __future__ import annotations

import argparse
import base64
import json
import mimetypes
import sys
from pathlib import Path

try:
    import requests
except ImportError as exc:  # pragma: no cover - runtime guidance only
    raise SystemExit(
        "Missing dependency: requests. Run with `uv run --with requests python tools/image_annotation_mvp.py ...`."
    ) from exc


DEFAULT_MODEL = "gpt-5.4"
DEFAULT_REASONING_EFFORT = "high"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Annotate one course image through an OpenAI-compatible Responses API."
    )
    parser.add_argument(
        "--endpoint",
        required=True,
        help="Responses base URL or full endpoint, e.g. https://co.yes.vg or https://co.yes.vg/v1/responses",
    )
    parser.add_argument("--api-key", required=True, help="API key for the Responses API")
    parser.add_argument("--model", default=DEFAULT_MODEL, help="Vision-capable model name")
    parser.add_argument(
        "--reasoning-effort",
        default=DEFAULT_REASONING_EFFORT,
        help="Responses reasoning.effort value; omit with --reasoning-effort none",
    )
    parser.add_argument(
        "--context-lines",
        type=int,
        default=3,
        help="Number of markdown lines to keep before and after the image reference",
    )
    parser.add_argument("--markdown", help="Path to the source full.md file")
    parser.add_argument(
        "--image-ref",
        help="Image reference as it appears inside markdown, e.g. images/foo.jpg",
    )
    parser.add_argument(
        "--image-url",
        help="Direct remote image URL to send as input_image.image_url",
    )
    return parser.parse_args()


def validate_args(args: argparse.Namespace) -> None:
    has_remote_image = bool(args.image_url)
    has_markdown_pair = bool(args.markdown and args.image_ref)

    if has_remote_image == has_markdown_pair:
        raise ValueError(
            "Provide exactly one image source: either --image-url, or both --markdown and --image-ref."
        )

    if args.markdown and not args.image_ref:
        raise ValueError("--image-ref is required when --markdown is provided.")
    if args.image_ref and not args.markdown:
        raise ValueError("--markdown is required when --image-ref is provided.")


def resolve_responses_url(endpoint: str) -> str:
    stripped = endpoint.rstrip("/")
    if stripped.endswith("/responses"):
        return stripped
    if stripped.endswith("/v1"):
        return f"{stripped}/responses"
    return f"{stripped}/v1/responses"


def load_markdown_context(
    markdown_path: Path, image_ref: str, context_lines: int
) -> tuple[str, Path, int]:
    text = markdown_path.read_text(encoding="utf-8")
    lines = text.splitlines()

    for index, line in enumerate(lines):
        if image_ref in line:
            start = max(0, index - context_lines)
            end = min(len(lines), index + context_lines + 1)
            context = "\n".join(lines[start:end]).strip()
            return context, markdown_path.parent / image_ref, index + 1

    raise ValueError(f"Image reference not found in markdown: {image_ref}")


def encode_image_as_data_url(image_path: Path) -> str:
    mime_type, _ = mimetypes.guess_type(image_path.name)
    if not mime_type:
        mime_type = "image/jpeg"
    encoded = base64.b64encode(image_path.read_bytes()).decode("ascii")
    return f"data:{mime_type};base64,{encoded}"


def build_prompt(source_label: str, image_line: int | None, context: str) -> str:
    location = (
        f"Image appears near markdown line: {image_line}\n\n" if image_line is not None else ""
    )
    return (
        "You are annotating one course slide image for downstream lesson generation.\n"
        "Decide whether this image has real pedagogical value, not whether it merely exists.\n"
        "Use both the image and the nearby text context.\n\n"
        "Return strict JSON with this schema:\n"
        "{\n"
        '  "summary": string,\n'
        '  "kind": "diagram" | "chart" | "table" | "formula_figure" | "illustration" | "photo" | "decorative" | "slide_junk" | "other",\n'
        '  "pedagogical_value": "critical" | "useful" | "optional" | "irrelevant",\n'
        '  "must_keep": boolean,\n'
        '  "reason": string,\n'
        '  "related_concepts": string[],\n'
        '  "suggested_alt": string,\n'
        '  "suggested_usage": string,\n'
        '  "confidence": number\n'
        "}\n\n"
        "Rules:\n"
        "- must_keep should be true only if the learner would be meaningfully worse off without this image.\n"
        "- If the image is decorative, low-signal, redundant, or obvious from text alone, mark it irrelevant or optional.\n"
        "- suggested_alt must be semantic, short, and learner-facing.\n"
        "- suggested_usage should describe how a guided story should use the image if it is kept.\n\n"
        f"Image source: {source_label}\n"
        f"{location}"
        "Nearby text context:\n"
        f"{context}"
    )


def build_request_payload(
    model: str, reasoning_effort: str | None, prompt: str, image_url: str
) -> dict:
    schema = {
        "type": "object",
        "additionalProperties": False,
        "properties": {
            "summary": {"type": "string"},
            "kind": {
                "type": "string",
                "enum": [
                    "diagram",
                    "chart",
                    "table",
                    "formula_figure",
                    "illustration",
                    "photo",
                    "decorative",
                    "slide_junk",
                    "other",
                ],
            },
            "pedagogical_value": {
                "type": "string",
                "enum": ["critical", "useful", "optional", "irrelevant"],
            },
            "must_keep": {"type": "boolean"},
            "reason": {"type": "string"},
            "related_concepts": {"type": "array", "items": {"type": "string"}},
            "suggested_alt": {"type": "string"},
            "suggested_usage": {"type": "string"},
            "confidence": {"type": "number"},
        },
        "required": [
            "summary",
            "kind",
            "pedagogical_value",
            "must_keep",
            "reason",
            "related_concepts",
            "suggested_alt",
            "suggested_usage",
            "confidence",
        ],
    }

    payload = {
        "model": model,
        "input": [
            {
                "role": "user",
                "content": [
                    {"type": "input_text", "text": prompt},
                    {"type": "input_image", "image_url": image_url},
                ],
            }
        ],
        "text": {
            "format": {
                "type": "json_schema",
                "name": "image_annotation",
                "strict": True,
                "schema": schema,
            }
        },
    }
    if reasoning_effort and reasoning_effort.lower() != "none":
        payload["reasoning"] = {"effort": reasoning_effort}
    return payload


def extract_output_text(response_body: dict) -> str:
    output_text = response_body.get("output_text")
    if isinstance(output_text, str) and output_text.strip():
        return output_text

    chunks: list[str] = []
    for item in response_body.get("output", []):
        for content in item.get("content", []):
            text = content.get("text")
            if isinstance(text, str):
                chunks.append(text)

    if chunks:
        return "\n".join(chunks)

    raise ValueError(
        "Could not extract output text from response: "
        f"{json.dumps(response_body, ensure_ascii=False)[:1000]}"
    )


def call_responses_api(endpoint: str, api_key: str, payload: dict) -> dict:
    response = requests.post(
        resolve_responses_url(endpoint),
        headers={
            "Content-Type": "application/json",
            "Authorization": f"Bearer {api_key}",
        },
        data=json.dumps(payload),
        timeout=120,
    )

    if not response.ok:
        raise RuntimeError(
            f"HTTP {response.status_code} {response.reason}: {response.text}"
        )
    return response.json()


def build_annotation_request(
    args: argparse.Namespace,
) -> tuple[dict, dict]:
    if args.image_url:
        prompt = build_prompt(
            source_label=args.image_url,
            image_line=None,
            context="No markdown context was provided. Focus on the image content itself.",
        )
        payload = build_request_payload(
            args.model,
            args.reasoning_effort,
            prompt,
            args.image_url,
        )
        sample = {
            "image_source": "remote_url",
            "image_url": args.image_url,
        }
        return sample, payload

    markdown_path = Path(args.markdown).resolve()
    context, image_path, image_line = load_markdown_context(
        markdown_path, args.image_ref, args.context_lines
    )

    if not image_path.is_file():
        raise FileNotFoundError(f"Resolved image path does not exist: {image_path}")

    image_url = encode_image_as_data_url(image_path)
    prompt = build_prompt(
        source_label=f"{markdown_path} -> {args.image_ref}",
        image_line=image_line,
        context=context,
    )
    payload = build_request_payload(
        args.model,
        args.reasoning_effort,
        prompt,
        image_url,
    )
    sample = {
        "image_source": "local_markdown_ref",
        "markdown": str(markdown_path),
        "image_ref": args.image_ref,
        "image_path": str(image_path),
        "image_line": image_line,
    }
    return sample, payload


def main() -> int:
    args = parse_args()
    validate_args(args)

    sample, payload = build_annotation_request(args)
    response = call_responses_api(args.endpoint, args.api_key, payload)
    output_text = extract_output_text(response)
    annotation = json.loads(output_text)

    result = {
        "sample": sample,
        "model": args.model,
        "reasoning_effort": None
        if args.reasoning_effort.lower() == "none"
        else args.reasoning_effort,
        "response_id": response.get("id"),
        "annotation": annotation,
    }
    print(json.dumps(result, ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        raise
