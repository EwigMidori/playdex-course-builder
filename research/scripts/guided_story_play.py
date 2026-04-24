#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any


TERM_TAG_RE = re.compile(r"<term id=\"([a-zA-Z0-9_.-]+)\">(.*?)</term>")


class GuidedStoryPlayError(Exception):
    pass


@dataclass
class ExerciseResult:
    prompt: str
    user_answer: str | None
    is_correct: bool | None


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()

    try:
        step_path = resolve_step_path(args)
        step = load_json(step_path)
        validate_step(step_path, step)
        return play_step(step_path, step, args)
    except GuidedStoryPlayError as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 2


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="guided_story_play",
        description="Headless terminal player for one guided_story step JSON.",
        epilog=(
            "Examples:\n"
            "  guided_story_play pipeline/3-guided_story/L1.step7.json\n"
            "  guided_story_play --step-id step7\n"
            "  guided_story_play --step-id step7 --show-focus-terms\n"
            "  guided_story_play --step-id step7 --auto\n"
        ),
        formatter_class=argparse.RawTextHelpFormatter,
    )
    parser.add_argument(
        "step_path",
        nargs="?",
        help="Path to a step JSON file. Optional if --step-id is used.",
    )
    parser.add_argument(
        "--step-id",
        help="Resolve a step via pipeline/3-guided_story/manifest.json sequence_id.",
    )
    parser.add_argument(
        "--manifest",
        default="pipeline/3-guided_story/manifest.json",
        help="Manifest path used with --step-id. Default: pipeline/3-guided_story/manifest.json",
    )
    parser.add_argument(
        "--show-focus-terms",
        action="store_true",
        help="Print glossary entries for focus_terms after each screen.",
    )
    parser.add_argument(
        "--show-source",
        action="store_true",
        help="Print source metadata before playback.",
    )
    parser.add_argument(
        "--auto",
        action="store_true",
        help="Auto-advance without waiting for navigation input. Exercises still prompt unless --skip-exercises.",
    )
    parser.add_argument(
        "--skip-exercises",
        action="store_true",
        help="Do not prompt for exercise answers; just display the exercise content.",
    )
    parser.add_argument(
        "--plain",
        action="store_true",
        help="Disable ANSI styling.",
    )
    return parser


def resolve_step_path(args: argparse.Namespace) -> Path:
    if args.step_path:
        return Path(args.step_path).resolve()
    if not args.step_id:
        raise GuidedStoryPlayError("provide either a step_path or --step-id")

    manifest_path = Path(args.manifest).resolve()
    manifest = load_json(manifest_path)
    for entry in manifest.get("steps", []):
        if entry.get("sequence_id") == args.step_id:
            return (manifest_path.parent.parent / Path(entry["file"]).relative_to("pipeline")).resolve()
    raise GuidedStoryPlayError(f"step-id not found in manifest: {args.step_id}")


def load_json(path: Path) -> dict[str, Any]:
    if not path.exists():
        raise GuidedStoryPlayError(f"file does not exist: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise GuidedStoryPlayError(f"invalid JSON in {path}: {exc}") from exc


def validate_step(path: Path, step: dict[str, Any]) -> None:
    if step.get("mode") != "guided_story":
        raise GuidedStoryPlayError(f"{path} is not a guided_story step")
    screens = step.get("screens")
    if not isinstance(screens, list):
        raise GuidedStoryPlayError(f"{path} missing screens array")
    if not screens:
        raise GuidedStoryPlayError(f"{path} has no screens")


def style(text: str, *, bold: bool = False, dim: bool = False, plain: bool = False) -> str:
    if plain or not sys.stdout.isatty():
        return text
    codes: list[str] = []
    if bold:
        codes.append("1")
    if dim:
        codes.append("2")
    if not codes:
        return text
    return f"\033[{';'.join(codes)}m{text}\033[0m"


def render_line(line: str, term_catalog: dict[str, Any], plain: bool) -> str:
    def replacer(match: re.Match[str]) -> str:
        term_id = match.group(1)
        display = match.group(2)
        if term_id in term_catalog:
            display = str(term_catalog[term_id].get("display") or display)
        return style(display, bold=True, plain=plain)

    return TERM_TAG_RE.sub(replacer, line)


def print_step_header(step_path: Path, step: dict[str, Any], plain: bool, show_source: bool) -> None:
    print(style(f"{step.get('lesson_id', '?')} / {step.get('sequence_id', '?')}", bold=True, plain=plain))
    print(style(step_path.as_posix(), dim=True, plain=plain))
    if show_source:
        source = step.get("source", {})
        print(style(f"source.plain_text: {source.get('plain_text', '')}", dim=True, plain=plain))
        related = source.get("related") or []
        for rel in related:
            print(style(f"source.related: {rel}", dim=True, plain=plain))
    print("")


def play_step(step_path: Path, step: dict[str, Any], args: argparse.Namespace) -> int:
    plain = args.plain
    term_catalog = step.get("term_catalog") or {}
    screens = step["screens"]
    results: list[ExerciseResult] = []
    index = 0

    print_step_header(step_path, step, plain, args.show_source)

    while 0 <= index < len(screens):
        screen = screens[index]
        print_screen(screen, term_catalog, index, len(screens), plain, args.show_focus_terms)

        if screen.get("type") == "exercise" and not args.skip_exercises:
            result = run_exercise(screen, plain)
            if result:
                results.append(result)

        if args.auto:
            index += 1
            continue

        action = prompt_nav(term_catalog, plain)
        if action == "next":
            index += 1
        elif action == "back":
            index = max(0, index - 1)
        elif action == "quit":
            break
        elif action.startswith("term:"):
            term_id = action.split(":", 1)[1]
            show_term(term_id, term_catalog, plain)

    print("")
    print(style("Playback finished.", bold=True, plain=plain))
    if results:
        summarize_results(results, plain)
    return 0


def print_screen(
    screen: dict[str, Any],
    term_catalog: dict[str, Any],
    index: int,
    total: int,
    plain: bool,
    show_focus_terms: bool,
) -> None:
    header = f"[{index + 1}/{total}] {screen.get('id', '?')} {screen.get('type', 'narration')}"
    print(style(header, dim=True, plain=plain))
    for line in screen.get("lines", []):
        print(render_line(line, term_catalog, plain))

    formula = screen.get("formula")
    if isinstance(formula, dict):
        latex = formula.get("latex")
        spoken = formula.get("spoken")
        if latex:
            print(style(f"formula: {latex}", dim=True, plain=plain))
        if spoken:
            print(style(f"spoken: {spoken}", dim=True, plain=plain))

    if show_focus_terms:
        focus_terms = screen.get("focus_terms") or []
        if focus_terms:
            print(style("focus terms:", dim=True, plain=plain))
            for term_id in focus_terms:
                show_term(term_id, term_catalog, plain, indent="  ", prefix="- ")
    print("")


def run_exercise(screen: dict[str, Any], plain: bool) -> ExerciseResult | None:
    exercise = screen.get("exercise")
    if not isinstance(exercise, dict):
        return None

    kind = exercise.get("kind")
    prompt = str(exercise.get("prompt") or "")
    print(style(prompt, bold=True, plain=plain))

    if kind == "single_choice":
        options = exercise.get("options") or []
        labels = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        for idx, option in enumerate(options):
            label = labels[idx] if idx < len(labels) else str(idx + 1)
            print(f"  {label}. {option}")
        raw = input("answer> ").strip()
        picked = parse_single_choice_answer(raw, len(options))
        answer = exercise.get("answer")
        is_correct = picked == answer if isinstance(answer, int) else None
        print_exercise_feedback(exercise, is_correct, plain)
        return ExerciseResult(prompt=prompt, user_answer=raw or None, is_correct=is_correct)

    if kind == "fill_in_blank":
        raw = input("answer> ").strip()
        answers = [normalize_text(v) for v in exercise.get("answers") or []]
        is_correct = normalize_text(raw) in answers if answers else None
        print_exercise_feedback(exercise, is_correct, plain)
        return ExerciseResult(prompt=prompt, user_answer=raw or None, is_correct=is_correct)

    if kind == "short_reflection":
        raw = input("answer> ").strip()
        explanation = exercise.get("explanation")
        if explanation:
            print(style(f"note: {explanation}", dim=True, plain=plain))
        return ExerciseResult(prompt=prompt, user_answer=raw or None, is_correct=None)

    print(style(f"unsupported exercise kind: {kind}", dim=True, plain=plain))
    return None


def parse_single_choice_answer(raw: str, option_count: int) -> int | None:
    if not raw:
        return None
    raw = raw.strip().upper()
    labels = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    if raw in labels[:option_count]:
        return labels.index(raw)
    if raw.isdigit():
        idx = int(raw) - 1
        if 0 <= idx < option_count:
            return idx
    return None


def normalize_text(value: str) -> str:
    return " ".join(value.strip().lower().split())


def print_exercise_feedback(exercise: dict[str, Any], is_correct: bool | None, plain: bool) -> None:
    if is_correct is True:
        print(style("correct", bold=True, plain=plain))
    elif is_correct is False:
        print(style("incorrect", bold=True, plain=plain))
    explanation = exercise.get("explanation")
    if explanation:
        print(style(f"why: {explanation}", dim=True, plain=plain))
    print("")


def prompt_nav(term_catalog: dict[str, Any], plain: bool) -> str:
    while True:
        raw = input("[Enter=next, b=back, t <id>=term, l=list terms, q=quit] > ").strip()
        if raw == "":
            return "next"
        if raw == "b":
            return "back"
        if raw == "q":
            return "quit"
        if raw == "l":
            print(style("available terms:", dim=True, plain=plain))
            for term_id in sorted(term_catalog):
                print(f"  {term_id}")
            print("")
            continue
        if raw.startswith("t "):
            term_id = raw.split(" ", 1)[1].strip()
            return f"term:{term_id}"
        print("unknown command")


def show_term(
    term_id: str,
    term_catalog: dict[str, Any],
    plain: bool,
    *,
    indent: str = "",
    prefix: str = "",
) -> None:
    term = term_catalog.get(term_id)
    if not isinstance(term, dict):
        print(f"{indent}{prefix}{term_id}: <missing>")
        return
    display = term.get("display") or term_id
    aliases = term.get("aliases") or []
    gloss = term.get("gloss") or ""
    line = f"{display} [{term_id}]"
    if aliases:
        line += f" | aliases: {', '.join(str(v) for v in aliases)}"
    print(f"{indent}{prefix}{style(line, bold=True, plain=plain)}")
    if gloss:
        print(f"{indent}  {gloss}")


def summarize_results(results: list[ExerciseResult], plain: bool) -> None:
    answered = [r for r in results if r.user_answer is not None]
    graded = [r for r in results if r.is_correct is not None]
    correct = [r for r in graded if r.is_correct]
    print(style("Exercise summary", bold=True, plain=plain))
    print(f"  answered: {len(answered)}")
    if graded:
        print(f"  correct: {len(correct)}/{len(graded)}")


if __name__ == "__main__":
    raise SystemExit(main())
