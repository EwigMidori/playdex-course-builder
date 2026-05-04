#!/usr/bin/env python3
from __future__ import annotations

import argparse
import datetime as dt
import difflib
import json
import os
import re
import shutil
import sys
from dataclasses import dataclass
from pathlib import Path, PurePosixPath
from typing import Any, Iterable


SCRIPT_PATH = Path(__file__).resolve()
PROJECT_ROOT = SCRIPT_PATH.parents[1]
DEFAULT_SOURCE_DIR = Path("pipeline/3-guided_story")
TX_ROOT = PROJECT_ROOT / ".guided_story_tx"
TX_TRANSACTIONS_DIR = TX_ROOT / "transactions"
TX_CURRENT_FILE = TX_ROOT / "current"
TX_NAME_RE = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._-]{0,127}$")


class GuidedStoryTxError(Exception):
    pass


@dataclass
class ValidationResult:
    errors: list[str]
    warnings: list[str]


@dataclass
class Transaction:
    name: str
    root: Path
    work_root: Path
    base_root: Path
    source_rel: Path
    source_abs: Path

    @property
    def work_source_dir(self) -> Path:
        return self.work_root / self.source_rel

    @property
    def base_source_dir(self) -> Path:
        return self.base_root / self.source_rel

    @property
    def meta_path(self) -> Path:
        return self.root / "meta.json"


class GuidedStoryTxCli:
    def main() -> int:
        parser = GuidedStoryTxCli.build_parser()
        args = parser.parse_args()
        if not hasattr(args, "handler"):
            parser.print_help()
            return 1

        try:
            return args.handler(args)
        except GuidedStoryTxError as exc:
            print(f"error: {exc}", file=sys.stderr)
            return 2


    def build_parser() -> argparse.ArgumentParser:
        parser = argparse.ArgumentParser(
            prog="guided_story_tx",
            description=(
                "Transactional CLI editor for pipeline/3-guided_story JSON assets. "
                "All edits stay inside a project-local transaction until save."
            ),
            formatter_class=argparse.RawTextHelpFormatter,
            epilog=(
                "Examples:\n"
                "  guided_story_tx tx-start draft-step5 --set-current\n"
                "  guided_story_tx tx-use draft-step5\n"
                "  guided_story_tx list tx\n"
                "  guided_story_tx list steps --tx draft-step5\n"
                "  guided_story_tx new step --tx draft-step5 --sequence-id step5 --lesson-id L1 --manifest-concept \"供需与均衡\"\n"
                "  guided_story_tx new screen --tx draft-step5 --step-id step1 --after s032 --type narration --line \"补一屏\" --renumber\n"
                "  guided_story_tx edit screen --tx draft-step5 --step-id step1 --screen-id s003 --line \"新的第一行\" --line \"新的第二行\"\n"
                "  guided_story_tx new term --tx draft-step5 --step-id step1 --term-id spread --display 点差 --alias Spread --gloss \"买价和卖价之间的差。\"\n"
                "  guided_story_tx edit manifest-step --tx draft-step5 --sequence-id step1 --concept \"为什么算法交易重要\"\n"
                "  guided_story_tx diff --tx draft-step5\n"
                "  guided_story_tx save --tx draft-step5\n"
            ),
        )
        subparsers = parser.add_subparsers(dest="command")

        GuidedStoryTxCli.add_tx_start_parser(subparsers)
        GuidedStoryTxCli.add_tx_use_parser(subparsers)
        GuidedStoryTxCli.add_new_parser(subparsers)
        GuidedStoryTxCli.add_edit_parser(subparsers)
        GuidedStoryTxCli.add_list_parser(subparsers)
        GuidedStoryTxCli.add_delete_parser(subparsers)
        GuidedStoryTxCli.add_diff_parser(subparsers)
        GuidedStoryTxCli.add_save_parser(subparsers)

        return parser


    def add_tx_selector_args(parser: argparse.ArgumentParser) -> None:
        parser.add_argument(
            "--tx",
            help="Explicit transaction name.",
        )
        parser.add_argument(
            "--current",
            action="store_true",
            help="Use the shared current transaction pointer explicitly.",
        )


    def add_set_args(parser: argparse.ArgumentParser) -> None:
        parser.add_argument(
            "--set-json",
            action="append",
            default=[],
            metavar="PATH=JSON",
            help=(
                "Set an object field from JSON. Example: "
                "--set-json 'exercise={\"kind\":\"single_choice\"}'"
            ),
        )
        parser.add_argument(
            "--set-str",
            action="append",
            default=[],
            metavar="PATH=TEXT",
            help="Set an object field to a plain string without JSON quoting.",
        )
        parser.add_argument(
            "--unset",
            action="append",
            default=[],
            metavar="PATH",
            help="Remove an object field.",
        )


    def add_tx_start_parser(subparsers: argparse._SubParsersAction[argparse.ArgumentParser]) -> None:
        parser = subparsers.add_parser(
            "tx-start",
            help="Create a transaction snapshot from pipeline/3-guided_story.",
        )
        parser.add_argument("name", help="Transaction name.")
        parser.add_argument(
            "--source-dir",
            default=str(DEFAULT_SOURCE_DIR),
            help="Source directory relative to project root. Default: pipeline/3-guided_story",
        )
        parser.add_argument(
            "--force",
            action="store_true",
            help="Replace an existing transaction with the same name.",
        )
        parser.add_argument(
            "--set-current",
            action="store_true",
            help="Also switch the shared current transaction pointer to this transaction.",
        )
        parser.set_defaults(handler=GuidedStoryTxCli.cmd_tx_start)


    def add_tx_use_parser(subparsers: argparse._SubParsersAction[argparse.ArgumentParser]) -> None:
        parser = subparsers.add_parser(
            "tx-use",
            help="Set the shared current transaction pointer explicitly.",
        )
        parser.add_argument("name", help="Transaction name to mark current.")
        parser.set_defaults(handler=GuidedStoryTxCli.cmd_tx_use)


    def add_new_parser(subparsers: argparse._SubParsersAction[argparse.ArgumentParser]) -> None:
        parser = subparsers.add_parser("new", help="Create a new object inside a transaction.")
        sub = parser.add_subparsers(dest="kind", required=True)

        step = sub.add_parser("step", help="Create a step JSON file, optionally with a manifest entry.")
        GuidedStoryTxCli.add_tx_selector_args(step)
        step.add_argument("--sequence-id", required=True, help="Step sequence id, e.g. step5.")
        step.add_argument("--lesson-id", help="Lesson id. Defaults to manifest lesson_id.")
        step.add_argument("--target-language", help="Optional target_language for the step.")
        step.add_argument("--plain-text", help="source.plain_text value.")
        step.add_argument(
            "--related",
            action="append",
            default=[],
            help="Append a source.related path. Repeatable.",
        )
        step.add_argument(
            "--file",
            help="Manifest file path relative to project root. Default: pipeline/3-guided_story/<lesson_id>.<sequence_id>.json",
        )
        step.add_argument(
            "--manifest-concept",
            help="Concept text for the manifest entry. Defaults to the sequence id.",
        )
        step.add_argument(
            "--skip-manifest",
            action="store_true",
            help="Create only the step file and do not create a manifest entry.",
        )
        step.add_argument("--before", help="Insert manifest entry before this sequence id.")
        step.add_argument("--after", help="Insert manifest entry after this sequence id.")
        step.add_argument("--index", type=int, help="Insert manifest entry at zero-based index.")
        GuidedStoryTxCli.add_set_args(step)
        step.set_defaults(handler=GuidedStoryTxCli.cmd_new_step)

        screen = sub.add_parser("screen", help="Create a screen inside a step.")
        GuidedStoryTxCli.add_tx_selector_args(screen)
        screen.add_argument("--step-id", required=True, help="Parent step sequence id.")
        screen.add_argument("--screen-id", help="Screen id. Defaults to the next available sNNN.")
        screen.add_argument("--type", default="narration", help="Screen type. Default: narration")
        screen.add_argument(
            "--line",
            action="append",
            default=[],
            help="Screen line. Repeat for multiple lines.",
        )
        screen.add_argument(
            "--introduced-term",
            action="append",
            default=[],
            help="Append to introduced_terms. Repeatable.",
        )
        screen.add_argument(
            "--focus-term",
            action="append",
            default=[],
            help="Append to focus_terms. Repeatable.",
        )
        screen.add_argument("--before", help="Insert before this screen id.")
        screen.add_argument("--after", help="Insert after this screen id.")
        screen.add_argument("--index", type=int, help="Insert at zero-based screen index.")
        screen.add_argument(
            "--renumber",
            action="store_true",
            help="Renumber all screen ids in the step after insertion.",
        )
        GuidedStoryTxCli.add_set_args(screen)
        screen.set_defaults(handler=GuidedStoryTxCli.cmd_new_screen)

        term = sub.add_parser("term", help="Create a term_catalog entry inside a step.")
        GuidedStoryTxCli.add_tx_selector_args(term)
        term.add_argument("--step-id", required=True, help="Parent step sequence id.")
        term.add_argument("--term-id", required=True, help="Term id, e.g. algorithmic_trading.")
        term.add_argument("--display", required=True, help="term_catalog.<id>.display")
        term.add_argument(
            "--alias",
            action="append",
            default=[],
            help="Append an alias. Repeatable.",
        )
        term.add_argument("--gloss", required=True, help="term_catalog.<id>.gloss")
        GuidedStoryTxCli.add_set_args(term)
        term.set_defaults(handler=GuidedStoryTxCli.cmd_new_term)

        manifest_step = sub.add_parser("manifest-step", help="Create a manifest entry.")
        GuidedStoryTxCli.add_tx_selector_args(manifest_step)
        manifest_step.add_argument("--sequence-id", required=True, help="Manifest sequence id.")
        manifest_step.add_argument(
            "--file",
            required=True,
            help="Step file path relative to project root.",
        )
        manifest_step.add_argument("--concept", required=True, help="Manifest concept text.")
        manifest_step.add_argument("--before", help="Insert before this sequence id.")
        manifest_step.add_argument("--after", help="Insert after this sequence id.")
        manifest_step.add_argument("--index", type=int, help="Insert at zero-based index.")
        GuidedStoryTxCli.add_set_args(manifest_step)
        manifest_step.set_defaults(handler=GuidedStoryTxCli.cmd_new_manifest_step)


    def add_edit_parser(subparsers: argparse._SubParsersAction[argparse.ArgumentParser]) -> None:
        parser = subparsers.add_parser("edit", help="Edit an existing object inside a transaction.")
        sub = parser.add_subparsers(dest="kind", required=True)

        step = sub.add_parser("step", help="Edit a step JSON file.")
        GuidedStoryTxCli.add_tx_selector_args(step)
        step.add_argument("--sequence-id", required=True, help="Target step sequence id.")
        step.add_argument(
            "--rename-sequence-id",
            help="Rename the step sequence_id and update the manifest entry and file path if possible.",
        )
        step.add_argument("--lesson-id", help="Replace lesson_id.")
        step.add_argument("--target-language", help="Replace target_language.")
        step.add_argument("--plain-text", help="Replace source.plain_text.")
        step.add_argument(
            "--related",
            action="append",
            default=[],
            help="Replace source.related with these values. Repeatable.",
        )
        step.add_argument(
            "--clear-related",
            action="store_true",
            help="Clear source.related.",
        )
        GuidedStoryTxCli.add_set_args(step)
        step.set_defaults(handler=GuidedStoryTxCli.cmd_edit_step)

        screen = sub.add_parser("screen", help="Edit a screen inside a step.")
        GuidedStoryTxCli.add_tx_selector_args(screen)
        screen.add_argument("--step-id", required=True, help="Parent step sequence id.")
        screen.add_argument("--screen-id", required=True, help="Target screen id.")
        screen.add_argument("--new-screen-id", help="Rename the screen id.")
        screen.add_argument("--type", help="Replace screen type.")
        screen.add_argument(
            "--line",
            action="append",
            default=[],
            help="Replace lines with these values. Repeatable.",
        )
        screen.add_argument(
            "--clear-lines",
            action="store_true",
            help="Clear the lines array before applying other updates.",
        )
        screen.add_argument(
            "--introduced-term",
            action="append",
            default=[],
            help="Replace introduced_terms with these values. Repeatable.",
        )
        screen.add_argument(
            "--focus-term",
            action="append",
            default=[],
            help="Replace focus_terms with these values. Repeatable.",
        )
        screen.add_argument("--before", help="Move before this screen id.")
        screen.add_argument("--after", help="Move after this screen id.")
        screen.add_argument("--index", type=int, help="Move to zero-based screen index.")
        screen.add_argument(
            "--renumber",
            action="store_true",
            help="Renumber all screen ids in the step after editing.",
        )
        GuidedStoryTxCli.add_set_args(screen)
        screen.set_defaults(handler=GuidedStoryTxCli.cmd_edit_screen)

        term = sub.add_parser("term", help="Edit a term_catalog entry.")
        GuidedStoryTxCli.add_tx_selector_args(term)
        term.add_argument("--step-id", required=True, help="Parent step sequence id.")
        term.add_argument("--term-id", required=True, help="Target term id.")
        term.add_argument("--rename-term-id", help="Rename the term id and update references in the step.")
        term.add_argument("--display", help="Replace display.")
        term.add_argument(
            "--alias",
            action="append",
            default=[],
            help="Replace aliases with these values. Repeatable.",
        )
        term.add_argument(
            "--add-alias",
            action="append",
            default=[],
            help="Append aliases without replacing existing aliases.",
        )
        term.add_argument(
            "--clear-aliases",
            action="store_true",
            help="Clear aliases before applying add/replace options.",
        )
        term.add_argument("--gloss", help="Replace gloss.")
        GuidedStoryTxCli.add_set_args(term)
        term.set_defaults(handler=GuidedStoryTxCli.cmd_edit_term)

        manifest_step = sub.add_parser("manifest-step", help="Edit a manifest entry.")
        GuidedStoryTxCli.add_tx_selector_args(manifest_step)
        manifest_step.add_argument("--sequence-id", required=True, help="Target manifest sequence id.")
        manifest_step.add_argument("--concept", help="Replace concept.")
        manifest_step.add_argument("--file", help="Replace file path relative to project root.")
        manifest_step.add_argument("--before", help="Move before this sequence id.")
        manifest_step.add_argument("--after", help="Move after this sequence id.")
        manifest_step.add_argument("--index", type=int, help="Move to zero-based index.")
        GuidedStoryTxCli.add_set_args(manifest_step)
        manifest_step.set_defaults(handler=GuidedStoryTxCli.cmd_edit_manifest_step)


    def add_list_parser(subparsers: argparse._SubParsersAction[argparse.ArgumentParser]) -> None:
        parser = subparsers.add_parser("list", help="List transactions or guided_story objects.")
        GuidedStoryTxCli.add_tx_selector_args(parser)
        parser.add_argument(
            "kind",
            choices=["tx", "steps", "screens", "terms", "manifest"],
            help="What to list.",
        )
        parser.add_argument("--step-id", help="Required for list screens|terms.")
        parser.set_defaults(handler=GuidedStoryTxCli.cmd_list)


    def add_delete_parser(subparsers: argparse._SubParsersAction[argparse.ArgumentParser]) -> None:
        parser = subparsers.add_parser("delete", help="Delete an object from a transaction.")
        sub = parser.add_subparsers(dest="kind", required=True)

        tx = sub.add_parser("tx", help="Delete a transaction directory.")
        tx.add_argument("name", help="Transaction name.")
        tx.add_argument(
            "--force",
            action="store_true",
            help="Allow deleting the current transaction and clear the current pointer.",
        )
        tx.set_defaults(handler=GuidedStoryTxCli.cmd_delete_tx)

        step = sub.add_parser("step", help="Delete a step file and its manifest entry.")
        GuidedStoryTxCli.add_tx_selector_args(step)
        step.add_argument("--sequence-id", required=True, help="Step sequence id.")
        step.add_argument(
            "--keep-manifest-entry",
            action="store_true",
            help="Delete only the step file.",
        )
        step.set_defaults(handler=GuidedStoryTxCli.cmd_delete_step)

        screen = sub.add_parser("screen", help="Delete a screen from a step.")
        GuidedStoryTxCli.add_tx_selector_args(screen)
        screen.add_argument("--step-id", required=True, help="Parent step sequence id.")
        screen.add_argument("--screen-id", required=True, help="Screen id.")
        screen.add_argument(
            "--renumber",
            action="store_true",
            help="Renumber all screen ids in the step after deletion.",
        )
        screen.set_defaults(handler=GuidedStoryTxCli.cmd_delete_screen)

        term = sub.add_parser("term", help="Delete a term_catalog entry.")
        GuidedStoryTxCli.add_tx_selector_args(term)
        term.add_argument("--step-id", required=True, help="Parent step sequence id.")
        term.add_argument("--term-id", required=True, help="Term id.")
        term.add_argument(
            "--drop-references",
            action="store_true",
            help="Also remove line tags and screen term references for this term.",
        )
        term.set_defaults(handler=GuidedStoryTxCli.cmd_delete_term)

        manifest_step = sub.add_parser("manifest-step", help="Delete a manifest entry.")
        GuidedStoryTxCli.add_tx_selector_args(manifest_step)
        manifest_step.add_argument("--sequence-id", required=True, help="Manifest sequence id.")
        manifest_step.add_argument(
            "--delete-step-file",
            action="store_true",
            help="Also delete the referenced step file if it exists.",
        )
        manifest_step.set_defaults(handler=GuidedStoryTxCli.cmd_delete_manifest_step)


    def add_diff_parser(subparsers: argparse._SubParsersAction[argparse.ArgumentParser]) -> None:
        parser = subparsers.add_parser("diff", help="Show differences between a transaction and the source directory.")
        GuidedStoryTxCli.add_tx_selector_args(parser)
        parser.add_argument(
            "--against",
            choices=["base", "live"],
            default="base",
            help="Diff transaction worktree against the base snapshot or the live source directory. Default: base",
        )
        parser.add_argument(
            "--name-only",
            action="store_true",
            help="Only print changed file names.",
        )
        parser.set_defaults(handler=GuidedStoryTxCli.cmd_diff)


    def add_save_parser(subparsers: argparse._SubParsersAction[argparse.ArgumentParser]) -> None:
        parser = subparsers.add_parser(
            "save",
            help="Validate and write the transaction back into pipeline/3-guided_story.",
        )
        GuidedStoryTxCli.add_tx_selector_args(parser)
        parser.add_argument(
            "--force",
            action="store_true",
            help="Allow save even if the live source directory diverged from the transaction base snapshot.",
        )
        parser.set_defaults(handler=GuidedStoryTxCli.cmd_save)


    def cmd_tx_start(args: argparse.Namespace) -> int:
        tx_name = GuidedStoryTxCli.validate_tx_name(args.name)
        source_rel = GuidedStoryTxCli.normalize_project_relpath(args.source_dir)
        source_abs = PROJECT_ROOT / source_rel
        if not source_abs.is_dir():
            raise GuidedStoryTxError(f"source directory does not exist: {source_rel.as_posix()}")

        GuidedStoryTxCli.ensure_tx_root()
        tx_dir = TX_TRANSACTIONS_DIR / tx_name
        if tx_dir.exists():
            if not args.force:
                raise GuidedStoryTxError(
                    f"transaction already exists: {tx_name}. Use --force to replace it."
                )
            shutil.rmtree(tx_dir)

        work_root = tx_dir / "work"
        base_root = tx_dir / "base"
        GuidedStoryTxCli.copy_source_tree(source_abs, work_root / source_rel)
        GuidedStoryTxCli.copy_source_tree(source_abs, base_root / source_rel)
        GuidedStoryTxCli.write_json(
            tx_dir / "meta.json",
            {
                "name": tx_name,
                "source_dir": source_rel.as_posix(),
                "created_at": dt.datetime.now(dt.timezone.utc).isoformat(),
            },
        )
        if args.set_current:
            GuidedStoryTxCli.set_current_tx(tx_name)

        print(f"started transaction {tx_name}")
        print(f"source: {source_rel.as_posix()}")
        if args.set_current:
            print(f"current: {tx_name}")
        else:
            print("current: unchanged (use tx-use NAME or re-run tx-start --set-current)")
        return 0


    def cmd_tx_use(args: argparse.Namespace) -> int:
        tx_name = GuidedStoryTxCli.validate_tx_name(args.name)
        tx_dir = TX_TRANSACTIONS_DIR / tx_name
        if not tx_dir.is_dir():
            raise GuidedStoryTxError(f"transaction not found: {tx_name}")
        meta = GuidedStoryTxCli.read_json(tx_dir / "meta.json")
        if not isinstance(meta, dict):
            raise GuidedStoryTxError(f"transaction metadata is invalid: {tx_name}")
        GuidedStoryTxCli.set_current_tx(tx_name)
        print(f"current: {tx_name}")
        return 0


    def cmd_new_step(args: argparse.Namespace) -> int:
        tx = GuidedStoryTxCli.resolve_tx_from_args(args)
        manifest = GuidedStoryTxCli.load_manifest(tx)
        sequence_id = args.sequence_id
        existing_entry = GuidedStoryTxCli.find_manifest_entry(manifest, sequence_id)
        lesson_id = args.lesson_id or manifest.get("lesson_id")
        if not isinstance(lesson_id, str) or not lesson_id:
            raise GuidedStoryTxError("step lesson_id is required and could not be inferred from manifest.json")

        target_rel = (
            GuidedStoryTxCli.normalize_source_file_relpath(tx, args.file)
            if args.file
            else GuidedStoryTxCli.default_step_relpath(tx.source_rel, lesson_id, sequence_id)
        )
        if existing_entry:
            target_rel = GuidedStoryTxCli.normalize_source_file_relpath(tx, existing_entry["file"])
        target_path = tx.work_root / target_rel
        if target_path.exists():
            raise GuidedStoryTxError(f"step file already exists: {target_rel.as_posix()}")

        step_obj: dict[str, Any] = {
            "lesson_id": lesson_id,
            "sequence_id": sequence_id,
            "mode": "guided_story",
            "source": {"plain_text": args.plain_text or "", "related": list(args.related)},
            "screens": [],
        }
        if args.target_language:
            step_obj["target_language"] = args.target_language

        GuidedStoryTxCli.apply_object_mutations(step_obj, args.set_json, args.set_str, args.unset)
        GuidedStoryTxCli.ensure_parent(target_path)
        GuidedStoryTxCli.write_json(target_path, step_obj)

        if not args.skip_manifest:
            if existing_entry:
                if args.manifest_concept:
                    existing_entry["concept"] = args.manifest_concept
                if args.file:
                    existing_entry["file"] = target_rel.as_posix()
            else:
                entry = {
                    "sequence_id": sequence_id,
                    "file": target_rel.as_posix(),
                    "concept": args.manifest_concept or sequence_id,
                }
                insert_at = GuidedStoryTxCli.choose_insert_index(
                    items=manifest["steps"],
                    before_id=args.before,
                    after_id=args.after,
                    index=args.index,
                    id_key="sequence_id",
                )
                manifest["steps"].insert(insert_at, entry)
            GuidedStoryTxCli.save_manifest(tx, manifest)

        print(f"created step {sequence_id} in transaction {tx.name}")
        return 0


    def cmd_new_screen(args: argparse.Namespace) -> int:
        tx = GuidedStoryTxCli.resolve_tx_from_args(args)
        step, step_path = GuidedStoryTxCli.load_step_by_sequence(tx, args.step_id)
        screens = step.setdefault("screens", [])
        if not isinstance(screens, list):
            raise GuidedStoryTxError(f"{args.step_id} has invalid screens data")

        screen_id = args.screen_id or GuidedStoryTxCli.next_screen_id(screens)
        if any(screen.get("id") == screen_id for screen in screens):
            raise GuidedStoryTxError(f"screen id already exists in {args.step_id}: {screen_id}")

        screen_obj: dict[str, Any] = {
            "id": screen_id,
            "type": args.type,
            "lines": list(args.line),
        }
        if args.introduced_term:
            screen_obj["introduced_terms"] = list(args.introduced_term)
        if args.focus_term:
            screen_obj["focus_terms"] = list(args.focus_term)
        GuidedStoryTxCli.apply_object_mutations(screen_obj, args.set_json, args.set_str, args.unset)

        insert_at = GuidedStoryTxCli.choose_insert_index(
            items=screens,
            before_id=args.before,
            after_id=args.after,
            index=args.index,
            id_key="id",
        )
        screens.insert(insert_at, screen_obj)
        if args.renumber:
            GuidedStoryTxCli.renumber_screens(screens)
        GuidedStoryTxCli.write_json(step_path, step)

        print(f"created screen {screen_obj['id']} in {args.step_id} for transaction {tx.name}")
        return 0


    def cmd_new_term(args: argparse.Namespace) -> int:
        tx = GuidedStoryTxCli.resolve_tx_from_args(args)
        step, step_path = GuidedStoryTxCli.load_step_by_sequence(tx, args.step_id)
        term_catalog = step.setdefault("term_catalog", {})
        if not isinstance(term_catalog, dict):
            raise GuidedStoryTxError(f"{args.step_id} has invalid term_catalog data")
        if args.term_id in term_catalog:
            raise GuidedStoryTxError(f"term already exists in {args.step_id}: {args.term_id}")

        term_obj: dict[str, Any] = {
            "display": args.display,
            "aliases": list(args.alias),
            "gloss": args.gloss,
        }
        GuidedStoryTxCli.apply_object_mutations(term_obj, args.set_json, args.set_str, args.unset)
        term_catalog[args.term_id] = term_obj
        GuidedStoryTxCli.write_json(step_path, step)

        print(f"created term {args.term_id} in {args.step_id} for transaction {tx.name}")
        return 0


    def cmd_new_manifest_step(args: argparse.Namespace) -> int:
        tx = GuidedStoryTxCli.resolve_tx_from_args(args)
        manifest = GuidedStoryTxCli.load_manifest(tx)
        if GuidedStoryTxCli.find_manifest_entry(manifest, args.sequence_id):
            raise GuidedStoryTxError(f"manifest entry already exists: {args.sequence_id}")

        entry: dict[str, Any] = {
            "sequence_id": args.sequence_id,
            "file": GuidedStoryTxCli.normalize_source_file_relpath(tx, args.file).as_posix(),
            "concept": args.concept,
        }
        GuidedStoryTxCli.apply_object_mutations(entry, args.set_json, args.set_str, args.unset)
        GuidedStoryTxCli.normalize_manifest_entry_file(tx, entry)
        insert_at = GuidedStoryTxCli.choose_insert_index(
            items=manifest["steps"],
            before_id=args.before,
            after_id=args.after,
            index=args.index,
            id_key="sequence_id",
        )
        manifest["steps"].insert(insert_at, entry)
        GuidedStoryTxCli.save_manifest(tx, manifest)

        print(f"created manifest entry {args.sequence_id} in transaction {tx.name}")
        return 0


    def cmd_edit_step(args: argparse.Namespace) -> int:
        tx = GuidedStoryTxCli.resolve_tx_from_args(args)
        step, step_path = GuidedStoryTxCli.load_step_by_sequence(tx, args.sequence_id)
        manifest = GuidedStoryTxCli.load_manifest(tx)
        manifest_entry = GuidedStoryTxCli.find_manifest_entry(manifest, args.sequence_id)

        old_sequence = step.get("sequence_id")
        if args.lesson_id:
            step["lesson_id"] = args.lesson_id
        if args.target_language:
            step["target_language"] = args.target_language
        if args.plain_text:
            source = GuidedStoryTxCli.ensure_dict(step, "source")
            source["plain_text"] = args.plain_text
        if args.clear_related:
            GuidedStoryTxCli.ensure_dict(step, "source")["related"] = []
        if args.related:
            GuidedStoryTxCli.ensure_dict(step, "source")["related"] = list(args.related)

        GuidedStoryTxCli.apply_object_mutations(step, args.set_json, args.set_str, args.unset)

        new_sequence = args.rename_sequence_id or old_sequence
        new_step_path = step_path
        if args.rename_sequence_id:
            if not isinstance(old_sequence, str) or not old_sequence:
                raise GuidedStoryTxError("existing step sequence_id is invalid")
            other_manifest_entry = GuidedStoryTxCli.find_manifest_entry(manifest, new_sequence)
            if other_manifest_entry and other_manifest_entry is not manifest_entry:
                raise GuidedStoryTxError(f"manifest already contains sequence_id {new_sequence}")
            other_step = GuidedStoryTxCli.find_step_path_by_sequence(tx, new_sequence)
            if other_step and other_step != step_path:
                raise GuidedStoryTxError(f"another step file already uses sequence_id {new_sequence}")

            step["sequence_id"] = new_sequence
            if manifest_entry:
                manifest_entry["sequence_id"] = new_sequence
                lesson_id = step.get("lesson_id") or manifest.get("lesson_id")
                current_rel = GuidedStoryTxCli.normalize_source_file_relpath(tx, manifest_entry["file"])
                expected_old = GuidedStoryTxCli.default_step_relpath(tx.source_rel, lesson_id, old_sequence)
                if current_rel == expected_old or current_rel.name.endswith(f".{old_sequence}.json"):
                    renamed_rel = GuidedStoryTxCli.default_step_relpath(tx.source_rel, lesson_id, new_sequence)
                    manifest_entry["file"] = renamed_rel.as_posix()
                    new_step_path = tx.work_root / renamed_rel
                else:
                    new_step_path = step_path
            else:
                new_step_path = step_path

        if new_step_path != step_path and new_step_path.exists():
            raise GuidedStoryTxError(f"target step file already exists: {GuidedStoryTxCli.path_in_project(new_step_path)}")
        GuidedStoryTxCli.write_json(step_path, step)
        if new_step_path != step_path:
            GuidedStoryTxCli.ensure_parent(new_step_path)
            shutil.move(step_path, new_step_path)
        if manifest_entry:
            GuidedStoryTxCli.save_manifest(tx, manifest)

        print(f"edited step {new_sequence} in transaction {tx.name}")
        return 0


    def cmd_edit_screen(args: argparse.Namespace) -> int:
        tx = GuidedStoryTxCli.resolve_tx_from_args(args)
        step, step_path = GuidedStoryTxCli.load_step_by_sequence(tx, args.step_id)
        screen, screen_index = GuidedStoryTxCli.find_screen(step, args.screen_id)

        if args.new_screen_id:
            if any(
                idx != screen_index and candidate.get("id") == args.new_screen_id
                for idx, candidate in enumerate(step["screens"])
            ):
                raise GuidedStoryTxError(f"screen id already exists in {args.step_id}: {args.new_screen_id}")
            screen["id"] = args.new_screen_id
        if args.type:
            screen["type"] = args.type
        if args.clear_lines:
            screen["lines"] = []
        if args.line:
            screen["lines"] = list(args.line)
        if args.introduced_term:
            screen["introduced_terms"] = list(args.introduced_term)
        if args.focus_term:
            screen["focus_terms"] = list(args.focus_term)

        GuidedStoryTxCli.apply_object_mutations(screen, args.set_json, args.set_str, args.unset)

        target_index = GuidedStoryTxCli.choose_insert_index(
            items=step["screens"],
            before_id=args.before,
            after_id=args.after,
            index=args.index,
            id_key="id",
            current_index=screen_index,
        )
        if target_index != screen_index:
            moved = step["screens"].pop(screen_index)
            step["screens"].insert(target_index, moved)
        if args.renumber:
            GuidedStoryTxCli.renumber_screens(step["screens"])
        GuidedStoryTxCli.write_json(step_path, step)

        print(f"edited screen {args.screen_id} in {args.step_id} for transaction {tx.name}")
        return 0


    def cmd_edit_term(args: argparse.Namespace) -> int:
        tx = GuidedStoryTxCli.resolve_tx_from_args(args)
        step, step_path = GuidedStoryTxCli.load_step_by_sequence(tx, args.step_id)
        term_catalog = step.setdefault("term_catalog", {})
        if not isinstance(term_catalog, dict) or args.term_id not in term_catalog:
            raise GuidedStoryTxError(f"term not found in {args.step_id}: {args.term_id}")

        term_obj = term_catalog[args.term_id]
        if args.display:
            term_obj["display"] = args.display
        if args.gloss:
            term_obj["gloss"] = args.gloss
        if args.clear_aliases:
            term_obj["aliases"] = []
        if args.alias:
            term_obj["aliases"] = list(args.alias)
        if args.add_alias:
            aliases = term_obj.setdefault("aliases", [])
            if not isinstance(aliases, list):
                raise GuidedStoryTxError(f"term aliases is not a list for {args.term_id}")
            aliases.extend(args.add_alias)

        GuidedStoryTxCli.apply_object_mutations(term_obj, args.set_json, args.set_str, args.unset)

        if args.rename_term_id:
            if args.rename_term_id in term_catalog and args.rename_term_id != args.term_id:
                raise GuidedStoryTxError(f"term id already exists in {args.step_id}: {args.rename_term_id}")
            GuidedStoryTxCli.rename_term_references(step, args.term_id, args.rename_term_id)
            term_catalog[args.rename_term_id] = term_catalog.pop(args.term_id)

        GuidedStoryTxCli.write_json(step_path, step)
        print(f"edited term {args.rename_term_id or args.term_id} in {args.step_id} for transaction {tx.name}")
        return 0


    def cmd_edit_manifest_step(args: argparse.Namespace) -> int:
        tx = GuidedStoryTxCli.resolve_tx_from_args(args)
        manifest = GuidedStoryTxCli.load_manifest(tx)
        entry = GuidedStoryTxCli.find_manifest_entry(manifest, args.sequence_id)
        if not entry:
            raise GuidedStoryTxError(f"manifest entry not found: {args.sequence_id}")

        if args.concept:
            entry["concept"] = args.concept
        if args.file:
            entry["file"] = GuidedStoryTxCli.normalize_source_file_relpath(tx, args.file).as_posix()

        GuidedStoryTxCli.apply_object_mutations(entry, args.set_json, args.set_str, args.unset)
        GuidedStoryTxCli.normalize_manifest_entry_file(tx, entry)

        current_index = manifest["steps"].index(entry)
        target_index = GuidedStoryTxCli.choose_insert_index(
            items=manifest["steps"],
            before_id=args.before,
            after_id=args.after,
            index=args.index,
            id_key="sequence_id",
            current_index=current_index,
        )
        if target_index != current_index:
            moved = manifest["steps"].pop(current_index)
            manifest["steps"].insert(target_index, moved)

        GuidedStoryTxCli.save_manifest(tx, manifest)
        print(f"edited manifest entry {args.sequence_id} in transaction {tx.name}")
        return 0


    def cmd_list(args: argparse.Namespace) -> int:
        if args.kind == "tx":
            return GuidedStoryTxCli.list_transactions()

        tx = GuidedStoryTxCli.resolve_tx_from_args(args)
        if args.kind == "steps":
            return GuidedStoryTxCli.list_steps(tx)
        if args.kind == "manifest":
            return GuidedStoryTxCli.list_manifest(tx)
        if args.kind == "screens":
            if not args.step_id:
                raise GuidedStoryTxError("--step-id is required for list screens")
            return GuidedStoryTxCli.list_screens(tx, args.step_id)
        if args.kind == "terms":
            if not args.step_id:
                raise GuidedStoryTxError("--step-id is required for list terms")
            return GuidedStoryTxCli.list_terms(tx, args.step_id)
        raise GuidedStoryTxError(f"unsupported list kind: {args.kind}")


    def cmd_delete_tx(args: argparse.Namespace) -> int:
        tx_name = GuidedStoryTxCli.validate_tx_name(args.name)
        tx_dir = TX_TRANSACTIONS_DIR / tx_name
        if not tx_dir.exists():
            raise GuidedStoryTxError(f"transaction not found: {tx_name}")
        current = GuidedStoryTxCli.read_current_tx_name()
        if current == tx_name and not args.force:
            raise GuidedStoryTxError(
                f"{tx_name} is current. Re-run with --force to delete it and clear the current pointer."
            )
        shutil.rmtree(tx_dir)
        if current == tx_name:
            GuidedStoryTxCli.clear_current_tx()
        print(f"deleted transaction {tx_name}")
        return 0


    def cmd_delete_step(args: argparse.Namespace) -> int:
        tx = GuidedStoryTxCli.resolve_tx_from_args(args)
        manifest = GuidedStoryTxCli.load_manifest(tx)
        step, step_path = GuidedStoryTxCli.load_step_by_sequence(tx, args.sequence_id)
        del step  # used for existence check only
        if step_path.exists():
            step_path.unlink()

        if not args.keep_manifest_entry:
            GuidedStoryTxCli.remove_manifest_entry(manifest, args.sequence_id)
            GuidedStoryTxCli.save_manifest(tx, manifest)

        print(f"deleted step {args.sequence_id} from transaction {tx.name}")
        return 0


    def cmd_delete_screen(args: argparse.Namespace) -> int:
        tx = GuidedStoryTxCli.resolve_tx_from_args(args)
        step, step_path = GuidedStoryTxCli.load_step_by_sequence(tx, args.step_id)
        _, screen_index = GuidedStoryTxCli.find_screen(step, args.screen_id)
        step["screens"].pop(screen_index)
        if args.renumber:
            GuidedStoryTxCli.renumber_screens(step["screens"])
        GuidedStoryTxCli.write_json(step_path, step)

        print(f"deleted screen {args.screen_id} from {args.step_id} in transaction {tx.name}")
        return 0


    def cmd_delete_term(args: argparse.Namespace) -> int:
        tx = GuidedStoryTxCli.resolve_tx_from_args(args)
        step, step_path = GuidedStoryTxCli.load_step_by_sequence(tx, args.step_id)
        term_catalog = step.setdefault("term_catalog", {})
        if not isinstance(term_catalog, dict) or args.term_id not in term_catalog:
            raise GuidedStoryTxError(f"term not found in {args.step_id}: {args.term_id}")

        if GuidedStoryTxCli.term_references_exist(step, args.term_id) and not args.drop_references:
            raise GuidedStoryTxError(
                f"term {args.term_id} is still referenced. Use --drop-references to strip tags and screen references."
            )
        if args.drop_references:
            GuidedStoryTxCli.remove_term_references(step, args.term_id)
        term_catalog.pop(args.term_id)
        GuidedStoryTxCli.write_json(step_path, step)

        print(f"deleted term {args.term_id} from {args.step_id} in transaction {tx.name}")
        return 0


    def cmd_delete_manifest_step(args: argparse.Namespace) -> int:
        tx = GuidedStoryTxCli.resolve_tx_from_args(args)
        manifest = GuidedStoryTxCli.load_manifest(tx)
        entry = GuidedStoryTxCli.find_manifest_entry(manifest, args.sequence_id)
        if not entry:
            raise GuidedStoryTxError(f"manifest entry not found: {args.sequence_id}")

        step_path = tx.work_root / GuidedStoryTxCli.manifest_entry_file_relpath(tx, entry)
        manifest["steps"].remove(entry)
        GuidedStoryTxCli.save_manifest(tx, manifest)
        if args.delete_step_file and step_path.exists():
            step_path.unlink()

        print(f"deleted manifest entry {args.sequence_id} from transaction {tx.name}")
        return 0


    def cmd_diff(args: argparse.Namespace) -> int:
        tx = GuidedStoryTxCli.resolve_tx_from_args(args)
        left_dir = tx.base_source_dir if args.against == "base" else tx.source_abs
        changes = GuidedStoryTxCli.compute_directory_changes(left_dir, tx.work_source_dir)
        if not changes:
            baseline_name = "base snapshot" if args.against == "base" else "live source"
            print(f"no changes between transaction {tx.name} and {baseline_name}")
            return 0

        if args.name_only:
            for change in changes:
                print(change)
            return 0

        for idx, rel_path in enumerate(changes):
            if idx:
                print()
            source_path = left_dir / rel_path
            tx_path = tx.work_source_dir / rel_path
            source_lines = GuidedStoryTxCli.read_text_lines(source_path)
            tx_lines = GuidedStoryTxCli.read_text_lines(tx_path)
            diff = difflib.unified_diff(
                source_lines,
                tx_lines,
                fromfile=f"{args.against}/{rel_path.as_posix()}",
                tofile=f"tx/{rel_path.as_posix()}",
                lineterm="",
            )
            printed = False
            for line in diff:
                print(line)
                printed = True
            if not printed:
                print(f"changed: {rel_path.as_posix()}")
        return 0


    def cmd_save(args: argparse.Namespace) -> int:
        tx = GuidedStoryTxCli.resolve_tx_from_args(args)
        validation = GuidedStoryTxCli.validate_transaction(tx)
        if validation.errors:
            print(f"validation failed for transaction {tx.name}:", file=sys.stderr)
            for item in validation.errors:
                print(f"  - {item}", file=sys.stderr)
            if validation.warnings:
                for item in validation.warnings:
                    print(f"  warning: {item}", file=sys.stderr)
            return 2

        if validation.warnings:
            for item in validation.warnings:
                print(f"warning: {item}", file=sys.stderr)

        diverged = GuidedStoryTxCli.compute_directory_changes(tx.base_source_dir, tx.source_abs)
        if diverged and not args.force:
            print(
                f"refusing to save {tx.name}: live source changed since tx-start. Re-run with --force if you want to overwrite.",
                file=sys.stderr,
            )
            for rel_path in diverged:
                print(f"  - {rel_path.as_posix()}", file=sys.stderr)
            return 2

        GuidedStoryTxCli.sync_directory(tx.work_source_dir, tx.source_abs)
        GuidedStoryTxCli.replace_directory(tx.work_source_dir, tx.base_source_dir)
        print(f"saved transaction {tx.name} into {GuidedStoryTxCli.path_in_project(tx.source_abs)}")
        return 0


    def list_transactions() -> int:
        GuidedStoryTxCli.ensure_tx_root()
        current = GuidedStoryTxCli.read_current_tx_name()
        rows = []
        for tx_dir in sorted(TX_TRANSACTIONS_DIR.iterdir()) if TX_TRANSACTIONS_DIR.exists() else []:
            if tx_dir.is_dir():
                marker = "*" if tx_dir.name == current else " "
                rows.append(f"{marker} {tx_dir.name}")
        if not rows:
            print("no transactions")
        else:
            print("\n".join(rows))
        return 0


    def list_steps(tx: Transaction) -> int:
        manifest = GuidedStoryTxCli.load_manifest(tx)
        file_map = GuidedStoryTxCli.load_step_file_map(tx)
        for idx, entry in enumerate(manifest.get("steps", [])):
            rel_path = GuidedStoryTxCli.manifest_entry_file_relpath(tx, entry)
            status = "ok" if rel_path in file_map else "missing"
            concept = entry.get("concept", "")
            print(f"{idx:02d} {entry['sequence_id']} [{status}] {concept} -> {rel_path.as_posix()}")
        return 0


    def list_manifest(tx: Transaction) -> int:
        manifest = GuidedStoryTxCli.load_manifest(tx)
        print(json.dumps(manifest, ensure_ascii=False, indent=2))
        return 0


    def list_screens(tx: Transaction, step_id: str) -> int:
        step, _ = GuidedStoryTxCli.load_step_by_sequence(tx, step_id)
        for idx, screen in enumerate(step.get("screens", [])):
            preview = " / ".join(screen.get("lines", [])[:2])
            print(f"{idx:02d} {screen.get('id')} [{screen.get('type')}] {preview}")
        return 0


    def list_terms(tx: Transaction, step_id: str) -> int:
        step, _ = GuidedStoryTxCli.load_step_by_sequence(tx, step_id)
        term_catalog = step.get("term_catalog", {})
        if not term_catalog:
            print(f"{step_id} has no term_catalog entries")
            return 0
        if not isinstance(term_catalog, dict):
            raise GuidedStoryTxError(f"{step_id} has invalid term_catalog data")
        for term_id, term_obj in sorted(term_catalog.items()):
            display = term_obj.get("display", "")
            gloss = term_obj.get("gloss", "")
            print(f"{term_id}: {display} | {gloss}")
        return 0


    def resolve_tx_from_args(args: argparse.Namespace) -> Transaction:
        tx_name = GuidedStoryTxCli.select_tx_name(args.tx, getattr(args, "current", False))
        return GuidedStoryTxCli.resolve_tx(tx_name)


    def select_tx_name(name: str | None, use_current: bool) -> str:
        if name and use_current:
            raise GuidedStoryTxError("use either --tx or --current, not both")
        if name:
            return GuidedStoryTxCli.validate_tx_name(name)
        if use_current:
            current = GuidedStoryTxCli.read_current_tx_name()
            if not current:
                raise GuidedStoryTxError("current transaction is not set. Use tx-use NAME first.")
            return current
        raise GuidedStoryTxError("select a transaction explicitly with --tx NAME or --current")


    def resolve_tx(name: str) -> Transaction:
        GuidedStoryTxCli.ensure_tx_root()
        tx_name = GuidedStoryTxCli.validate_tx_name(name)
        tx_dir = TX_TRANSACTIONS_DIR / tx_name
        if not tx_dir.is_dir():
            raise GuidedStoryTxError(f"transaction not found: {tx_name}")
        meta = GuidedStoryTxCli.read_json(tx_dir / "meta.json")
        if not isinstance(meta, dict):
            raise GuidedStoryTxError(f"transaction metadata is invalid: {tx_name}")
        meta_name = meta.get("name")
        if not isinstance(meta_name, str) or GuidedStoryTxCli.validate_tx_name(meta_name) != tx_name:
            raise GuidedStoryTxError(f"transaction metadata name mismatch: {tx_name}")
        source_rel_value = meta.get("source_dir")
        if not isinstance(source_rel_value, str) or not source_rel_value:
            raise GuidedStoryTxError(f"transaction metadata missing source_dir: {tx_name}")
        source_rel = GuidedStoryTxCli.normalize_project_relpath(source_rel_value)
        return Transaction(
            name=tx_name,
            root=tx_dir,
            work_root=tx_dir / "work",
            base_root=tx_dir / "base",
            source_rel=source_rel,
            source_abs=PROJECT_ROOT / source_rel,
        )


    def ensure_tx_root() -> None:
        TX_TRANSACTIONS_DIR.mkdir(parents=True, exist_ok=True)


    def set_current_tx(name: str) -> None:
        safe_name = GuidedStoryTxCli.validate_tx_name(name)
        GuidedStoryTxCli.ensure_parent(TX_CURRENT_FILE)
        tmp_path = TX_CURRENT_FILE.with_suffix(".tmp")
        tmp_path.write_text(safe_name + "\n", encoding="utf-8")
        os.replace(tmp_path, TX_CURRENT_FILE)


    def clear_current_tx() -> None:
        if TX_CURRENT_FILE.exists():
            TX_CURRENT_FILE.unlink()


    def read_current_tx_name() -> str | None:
        if not TX_CURRENT_FILE.exists():
            return None
        value = TX_CURRENT_FILE.read_text(encoding="utf-8").strip()
        if not value:
            return None
        return GuidedStoryTxCli.validate_tx_name(value)


    def validate_tx_name(name: str) -> str:
        if not isinstance(name, str):
            raise GuidedStoryTxError("transaction name must be a string")
        candidate = name.strip()
        if not TX_NAME_RE.fullmatch(candidate):
            raise GuidedStoryTxError(
                "transaction name must match [A-Za-z0-9][A-Za-z0-9._-]{0,127}"
            )
        return candidate


    def read_json(path: Path) -> Any:
        try:
            return json.loads(path.read_text(encoding="utf-8"))
        except FileNotFoundError as exc:
            raise GuidedStoryTxError(f"missing JSON file: {GuidedStoryTxCli.path_in_project(path)}") from exc
        except json.JSONDecodeError as exc:
            raise GuidedStoryTxError(
                f"invalid JSON in {GuidedStoryTxCli.path_in_project(path)} at line {exc.lineno} column {exc.colno}: {exc.msg}"
            ) from exc


    def write_json(path: Path, data: Any) -> None:
        GuidedStoryTxCli.ensure_parent(path)
        path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


    def ensure_parent(path: Path) -> None:
        path.parent.mkdir(parents=True, exist_ok=True)


    def normalize_project_relpath(path_value: str | Path) -> Path:
        candidate = Path(path_value)
        if candidate.is_absolute():
            try:
                candidate = candidate.relative_to(PROJECT_ROOT)
            except ValueError as exc:
                raise GuidedStoryTxError(f"path must stay inside project root: {candidate}") from exc
        normalized = PurePosixPath(candidate.as_posix())
        if normalized.is_absolute() or ".." in normalized.parts:
            raise GuidedStoryTxError(f"path must stay inside project root: {candidate}")
        return Path(normalized.as_posix())


    def path_in_project(path: Path) -> str:
        try:
            return path.relative_to(PROJECT_ROOT).as_posix()
        except ValueError:
            return str(path)


    def copy_source_tree(source_abs: Path, dest_abs: Path) -> None:
        GuidedStoryTxCli.ensure_parent(dest_abs)
        shutil.copytree(source_abs, dest_abs)


    def replace_directory(source_abs: Path, dest_abs: Path) -> None:
        if dest_abs.exists():
            shutil.rmtree(dest_abs)
        GuidedStoryTxCli.copy_source_tree(source_abs, dest_abs)


    def read_text_lines(path: Path) -> list[str]:
        if not path.exists():
            return []
        return path.read_text(encoding="utf-8").splitlines()


    def load_manifest(tx: Transaction) -> dict[str, Any]:
        manifest_path = tx.work_source_dir / "manifest.json"
        data = GuidedStoryTxCli.read_json(manifest_path)
        if not isinstance(data, dict):
            raise GuidedStoryTxError("manifest.json must be a JSON object")
        if "steps" not in data or not isinstance(data["steps"], list):
            raise GuidedStoryTxError("manifest.json must contain a steps array")
        return data


    def save_manifest(tx: Transaction, manifest: dict[str, Any]) -> None:
        GuidedStoryTxCli.write_json(tx.work_source_dir / "manifest.json", manifest)


    def find_manifest_entry(manifest: dict[str, Any], sequence_id: str) -> dict[str, Any] | None:
        for entry in manifest.get("steps", []):
            if entry.get("sequence_id") == sequence_id:
                return entry
        return None


    def remove_manifest_entry(manifest: dict[str, Any], sequence_id: str) -> None:
        entry = GuidedStoryTxCli.find_manifest_entry(manifest, sequence_id)
        if not entry:
            raise GuidedStoryTxError(f"manifest entry not found: {sequence_id}")
        manifest["steps"].remove(entry)


    def default_step_relpath(source_rel: Path, lesson_id: str, sequence_id: str) -> Path:
        return source_rel / f"{lesson_id}.{sequence_id}.json"


    def manifest_entry_file_relpath(tx: Transaction, entry: dict[str, Any]) -> Path:
        file_value = entry.get("file")
        if not isinstance(file_value, str) or not file_value:
            raise GuidedStoryTxError("manifest entry file must be a non-empty string")
        return GuidedStoryTxCli.normalize_source_file_relpath(tx, file_value)


    def normalize_manifest_entry_file(tx: Transaction, entry: dict[str, Any]) -> None:
        entry["file"] = GuidedStoryTxCli.manifest_entry_file_relpath(tx, entry).as_posix()


    def normalize_source_file_relpath(tx: Transaction, path_value: str | Path) -> Path:
        rel_path = GuidedStoryTxCli.normalize_project_relpath(path_value)
        try:
            rel_path.relative_to(tx.source_rel)
        except ValueError as exc:
            raise GuidedStoryTxError(
                f"step file path must stay inside source dir {tx.source_rel.as_posix()}: {rel_path.as_posix()}"
            ) from exc
        if rel_path.name == "manifest.json":
            raise GuidedStoryTxError("step file path cannot point to manifest.json")
        if rel_path.suffix != ".json":
            raise GuidedStoryTxError(f"step file path must end with .json: {rel_path.as_posix()}")
        return rel_path


    def load_step_file_map(tx: Transaction) -> dict[Path, Path]:
        mapping: dict[Path, Path] = {}
        for json_path in sorted((tx.work_root / tx.source_rel).rglob("*.json")):
            if json_path.name == "manifest.json":
                continue
            rel_path = GuidedStoryTxCli.normalize_source_file_relpath(tx, json_path.relative_to(tx.work_root))
            mapping[rel_path] = json_path
        return mapping


    def load_step_by_sequence(tx: Transaction, sequence_id: str) -> tuple[dict[str, Any], Path]:
        step_path = GuidedStoryTxCli.find_step_path_by_sequence(tx, sequence_id)
        if not step_path:
            raise GuidedStoryTxError(f"step not found in transaction {tx.name}: {sequence_id}")
        step = GuidedStoryTxCli.read_json(step_path)
        if not isinstance(step, dict):
            raise GuidedStoryTxError(f"step file is not a JSON object: {GuidedStoryTxCli.path_in_project(step_path)}")
        return step, step_path


    def find_step_path_by_sequence(tx: Transaction, sequence_id: str) -> Path | None:
        manifest = GuidedStoryTxCli.load_manifest(tx)
        matches: list[Path] = []
        entry = GuidedStoryTxCli.find_manifest_entry(manifest, sequence_id)
        if entry:
            candidate = tx.work_root / GuidedStoryTxCli.manifest_entry_file_relpath(tx, entry)
            if candidate.exists():
                matches.append(candidate)
        for json_path in sorted(tx.work_source_dir.rglob("*.json")):
            if json_path.name == "manifest.json":
                continue
            try:
                data = GuidedStoryTxCli.read_json(json_path)
            except GuidedStoryTxError:
                continue
            if isinstance(data, dict) and data.get("sequence_id") == sequence_id:
                if json_path not in matches:
                    matches.append(json_path)
        if not matches:
            return None
        if len(matches) > 1:
            rendered = ", ".join(GuidedStoryTxCli.path_in_project(path) for path in matches)
            raise GuidedStoryTxError(f"multiple step files share sequence_id {sequence_id}: {rendered}")
        return matches[0]


    def find_screen(step: dict[str, Any], screen_id: str) -> tuple[dict[str, Any], int]:
        screens = step.get("screens")
        if not isinstance(screens, list):
            raise GuidedStoryTxError("step screens must be a list")
        for idx, screen in enumerate(screens):
            if isinstance(screen, dict) and screen.get("id") == screen_id:
                return screen, idx
        raise GuidedStoryTxError(f"screen not found: {screen_id}")


    def next_screen_id(screens: list[dict[str, Any]]) -> str:
        numbers = []
        for screen in screens:
            screen_id = screen.get("id")
            if isinstance(screen_id, str):
                match = re.fullmatch(r"s(\d+)", screen_id)
                if match:
                    numbers.append(int(match.group(1)))
        next_value = max(numbers, default=0) + 1
        return f"s{next_value:03d}"


    def renumber_screens(screens: list[dict[str, Any]]) -> None:
        for idx, screen in enumerate(screens, start=1):
            if isinstance(screen, dict):
                screen["id"] = f"s{idx:03d}"


    def choose_insert_index(
        *,
        items: list[dict[str, Any]],
        before_id: str | None,
        after_id: str | None,
        index: int | None,
        id_key: str,
        current_index: int | None = None,
    ) -> int:
        modes = [before_id is not None, after_id is not None, index is not None]
        if sum(modes) > 1:
            raise GuidedStoryTxError("use only one of --before, --after, --index")

        adjusted_items = items
        if current_index is not None:
            adjusted_items = [item for idx, item in enumerate(items) if idx != current_index]

        if before_id is not None:
            for idx, item in enumerate(adjusted_items):
                if item.get(id_key) == before_id:
                    return idx
            raise GuidedStoryTxError(f"{id_key} not found for --before: {before_id}")

        if after_id is not None:
            for idx, item in enumerate(adjusted_items):
                if item.get(id_key) == after_id:
                    return idx + 1
            raise GuidedStoryTxError(f"{id_key} not found for --after: {after_id}")

        if index is not None:
            if index < 0 or index > len(adjusted_items):
                raise GuidedStoryTxError(f"--index out of range: {index}")
            return index

        if current_index is not None:
            return current_index
        return len(adjusted_items)


    def ensure_dict(parent: dict[str, Any], key: str) -> dict[str, Any]:
        value = parent.setdefault(key, {})
        if not isinstance(value, dict):
            raise GuidedStoryTxError(f"{key} must be a JSON object")
        return value


    def parse_json_assignment(value: str) -> tuple[list[str], Any]:
        if "=" not in value:
            raise GuidedStoryTxError(f"expected PATH=JSON, got: {value}")
        path_str, payload = value.split("=", 1)
        if not path_str:
            raise GuidedStoryTxError(f"missing assignment path in: {value}")
        try:
            parsed = json.loads(payload)
        except json.JSONDecodeError as exc:
            raise GuidedStoryTxError(
                f"invalid JSON for assignment {path_str}: line {exc.lineno} column {exc.colno}: {exc.msg}"
            ) from exc
        return GuidedStoryTxCli.parse_object_path(path_str), parsed


    def parse_string_assignment(value: str) -> tuple[list[str], str]:
        if "=" not in value:
            raise GuidedStoryTxError(f"expected PATH=TEXT, got: {value}")
        path_str, payload = value.split("=", 1)
        if not path_str:
            raise GuidedStoryTxError(f"missing assignment path in: {value}")
        return GuidedStoryTxCli.parse_object_path(path_str), payload


    def parse_object_path(path_str: str) -> list[str]:
        raw_parts = path_str.split(".")
        if not path_str or any(part == "" for part in raw_parts):
            raise GuidedStoryTxError(f"invalid object path: {path_str}")
        return raw_parts


    def apply_object_mutations(
        obj: dict[str, Any],
        set_json_items: Iterable[str],
        set_str_items: Iterable[str],
        unset_items: Iterable[str],
    ) -> None:
        for item in set_json_items:
            path, value = GuidedStoryTxCli.parse_json_assignment(item)
            GuidedStoryTxCli.set_path_value(obj, path, value)
        for item in set_str_items:
            path, value = GuidedStoryTxCli.parse_string_assignment(item)
            GuidedStoryTxCli.set_path_value(obj, path, value)
        for item in unset_items:
            GuidedStoryTxCli.remove_path_value(obj, GuidedStoryTxCli.parse_object_path(item))


    def set_path_value(container: dict[str, Any], path: list[str], value: Any) -> None:
        current: Any = container
        for offset, key in enumerate(path[:-1]):
            if isinstance(current, dict):
                if key not in current:
                    raise GuidedStoryTxError(f"path does not exist: {'.'.join(path[: offset + 1])}")
                next_value = current[key]
                if not isinstance(next_value, (dict, list)):
                    raise GuidedStoryTxError(
                        f"cannot descend into non-container at {'.'.join(path[: offset + 1])}"
                    )
                current = next_value
                continue
            if isinstance(current, list):
                idx = GuidedStoryTxCli.parse_index_token(key, len(current), allow_end=False)
                next_value = current[idx]
                if not isinstance(next_value, (dict, list)):
                    raise GuidedStoryTxError(
                        f"cannot descend into non-container at {'.'.join(path[: offset + 1])}"
                    )
                current = next_value
                continue
            raise GuidedStoryTxError(f"cannot descend into non-container at {'.'.join(path[:-1])}")

        last = path[-1]
        if isinstance(current, dict):
            current[last] = value
            return
        if isinstance(current, list):
            idx = GuidedStoryTxCli.parse_index_token(last, len(current), allow_end=False)
            current[idx] = value
            return
        raise GuidedStoryTxError(f"cannot set path on non-container: {'.'.join(path[:-1])}")


    def remove_path_value(container: dict[str, Any], path: list[str]) -> None:
        current: Any = container
        for offset, key in enumerate(path[:-1]):
            if isinstance(current, dict):
                if key not in current:
                    raise GuidedStoryTxError(f"path does not exist: {'.'.join(path[: offset + 1])}")
                next_value = current[key]
                if not isinstance(next_value, (dict, list)):
                    raise GuidedStoryTxError(
                        f"cannot descend into non-container at {'.'.join(path[: offset + 1])}"
                    )
                current = next_value
                continue
            if isinstance(current, list):
                idx = GuidedStoryTxCli.parse_index_token(key, len(current), allow_end=False)
                next_value = current[idx]
                if not isinstance(next_value, (dict, list)):
                    raise GuidedStoryTxError(
                        f"cannot descend into non-container at {'.'.join(path[: offset + 1])}"
                    )
                current = next_value
                continue
            raise GuidedStoryTxError(f"cannot descend into non-container at {'.'.join(path[:-1])}")
        if isinstance(current, dict):
            if path[-1] not in current:
                raise GuidedStoryTxError(f"path does not exist: {'.'.join(path)}")
            current.pop(path[-1])
        elif isinstance(current, list):
            idx = GuidedStoryTxCli.parse_index_token(path[-1], len(current), allow_end=False)
            current.pop(idx)
        else:
            raise GuidedStoryTxError(f"cannot unset path on non-container: {'.'.join(path[:-1])}")


    def is_index_token(token: str) -> bool:
        return token.isdigit()


    def parse_index_token(token: str, size: int, *, allow_end: bool) -> int:
        if not GuidedStoryTxCli.is_index_token(token):
            raise GuidedStoryTxError(f"expected list index, got: {token}")
        idx = int(token)
        upper = size if allow_end else size - 1
        if idx < 0 or idx > upper:
            raise GuidedStoryTxError(f"list index out of range: {token}")
        return idx


    def rename_term_references(step: dict[str, Any], old_id: str, new_id: str) -> None:
        for screen in step.get("screens", []):
            if not isinstance(screen, dict):
                continue
            for key in ("introduced_terms", "focus_terms"):
                values = screen.get(key)
                if isinstance(values, list):
                    screen[key] = [new_id if value == old_id else value for value in values]
            lines = screen.get("lines")
            if isinstance(lines, list):
                screen["lines"] = [GuidedStoryTxCli.replace_term_id_tag(line, old_id, new_id) for line in lines]


    def term_references_exist(step: dict[str, Any], term_id: str) -> bool:
        pattern = re.compile(rf"<term\s+id=['\"]{re.escape(term_id)}['\"]>")
        for screen in step.get("screens", []):
            if not isinstance(screen, dict):
                continue
            for key in ("introduced_terms", "focus_terms"):
                values = screen.get(key)
                if isinstance(values, list) and term_id in values:
                    return True
            lines = screen.get("lines")
            if isinstance(lines, list) and any(isinstance(line, str) and pattern.search(line) for line in lines):
                return True
        return False


    def remove_term_references(step: dict[str, Any], term_id: str) -> None:
        open_tag = re.compile(rf"<term\s+id=['\"]{re.escape(term_id)}['\"]>(.*?)</term>")
        for screen in step.get("screens", []):
            if not isinstance(screen, dict):
                continue
            for key in ("introduced_terms", "focus_terms"):
                values = screen.get(key)
                if isinstance(values, list):
                    screen[key] = [value for value in values if value != term_id]
            lines = screen.get("lines")
            if isinstance(lines, list):
                cleaned = []
                for line in lines:
                    if isinstance(line, str):
                        line = open_tag.sub(r"\1", line)
                        line = GuidedStoryTxCli.replace_term_id_tag(line, term_id, term_id, drop_tag=True)
                    cleaned.append(line)
                screen["lines"] = cleaned


    def replace_term_id_tag(line: str, old_id: str, new_id: str, drop_tag: bool = False) -> str:
        if drop_tag:
            return re.sub(rf"<term\s+id=['\"]{re.escape(old_id)}['\"]>(.*?)</term>", r"\1", line)
        return re.sub(
            rf"(<term\s+id=['\"]){re.escape(old_id)}(['\"])",
            rf"\1{new_id}\2",
            line,
        )


    def validate_transaction(tx: Transaction) -> ValidationResult:
        errors: list[str] = []
        warnings: list[str] = []
        manifest_path = tx.work_source_dir / "manifest.json"

        try:
            manifest = GuidedStoryTxCli.read_json(manifest_path)
        except GuidedStoryTxError as exc:
            return ValidationResult(errors=[str(exc)], warnings=[])

        if not isinstance(manifest, dict):
            return ValidationResult(errors=["manifest.json must be a JSON object"], warnings=[])

        if not isinstance(manifest.get("lesson_id"), str) or not manifest.get("lesson_id"):
            errors.append("manifest.json missing non-empty lesson_id")
        if manifest.get("mode") != "guided_story":
            errors.append("manifest.json mode must be 'guided_story'")
        steps = manifest.get("steps")
        if not isinstance(steps, list):
            errors.append("manifest.json steps must be a list")
            return ValidationResult(errors=errors, warnings=warnings)

        seen_sequence_ids: set[str] = set()
        seen_files: set[str] = set()
        referenced_files: set[Path] = set()
        for idx, entry in enumerate(steps):
            prefix = f"manifest.steps[{idx}]"
            if not isinstance(entry, dict):
                errors.append(f"{prefix} must be an object")
                continue
            sequence_id = entry.get("sequence_id")
            file_value = entry.get("file")
            concept = entry.get("concept")
            if not isinstance(sequence_id, str) or not sequence_id:
                errors.append(f"{prefix}.sequence_id must be a non-empty string")
                continue
            if sequence_id in seen_sequence_ids:
                errors.append(f"duplicate manifest sequence_id: {sequence_id}")
            seen_sequence_ids.add(sequence_id)
            if not isinstance(file_value, str) or not file_value:
                errors.append(f"{prefix}.file must be a non-empty string")
                continue
            if file_value in seen_files:
                errors.append(f"duplicate manifest file: {file_value}")
            seen_files.add(file_value)
            if not isinstance(concept, str) or not concept:
                errors.append(f"{prefix}.concept must be a non-empty string")

            try:
                step_rel = GuidedStoryTxCli.normalize_source_file_relpath(tx, file_value)
            except GuidedStoryTxError as exc:
                errors.append(f"{prefix}.file invalid: {exc}")
                continue
            referenced_files.add(step_rel)
            step_path = tx.work_root / step_rel
            if not step_path.exists():
                errors.append(f"missing step file for {sequence_id}: {step_rel.as_posix()}")
                continue
            GuidedStoryTxCli.validate_step_file(
                tx=tx,
                manifest=manifest,
                manifest_entry=entry,
                step_path=step_path,
                errors=errors,
                warnings=warnings,
            )

        for json_path in sorted(tx.work_source_dir.rglob("*.json")):
            if json_path.name == "manifest.json":
                continue
            rel_path = GuidedStoryTxCli.normalize_source_file_relpath(tx, json_path.relative_to(tx.work_root))
            if rel_path not in referenced_files:
                warnings.append(f"step file not referenced by manifest: {rel_path.as_posix()}")

        return ValidationResult(errors=errors, warnings=warnings)


    def validate_step_file(
        *,
        tx: Transaction,
        manifest: dict[str, Any],
        manifest_entry: dict[str, Any],
        step_path: Path,
        errors: list[str],
        warnings: list[str],
    ) -> None:
        rel_path = GuidedStoryTxCli.path_in_project(step_path)
        try:
            step = GuidedStoryTxCli.read_json(step_path)
        except GuidedStoryTxError as exc:
            errors.append(str(exc))
            return
        if not isinstance(step, dict):
            errors.append(f"{rel_path} must be a JSON object")
            return

        if not isinstance(step.get("lesson_id"), str) or not step.get("lesson_id"):
            errors.append(f"{rel_path} missing non-empty lesson_id")
        elif step.get("lesson_id") != manifest.get("lesson_id"):
            warnings.append(
                f"{rel_path} lesson_id {step.get('lesson_id')} differs from manifest {manifest.get('lesson_id')}"
            )

        if step.get("mode") != "guided_story":
            errors.append(f"{rel_path} mode must be 'guided_story'")
        if step.get("sequence_id") != manifest_entry.get("sequence_id"):
            errors.append(
                f"{rel_path} sequence_id {step.get('sequence_id')} does not match manifest {manifest_entry.get('sequence_id')}"
            )
        if "target_language" in step and not isinstance(step.get("target_language"), str):
            errors.append(f"{rel_path} target_language must be a string when present")

        source = step.get("source")
        if not isinstance(source, dict):
            errors.append(f"{rel_path} source must be an object")
        else:
            if not isinstance(source.get("plain_text"), str):
                errors.append(f"{rel_path} source.plain_text must be a string")
            related = source.get("related", [])
            if not isinstance(related, list) or any(not isinstance(item, str) for item in related):
                errors.append(f"{rel_path} source.related must be a string array")

        term_catalog = step.get("term_catalog", {})
        if term_catalog is not None and not isinstance(term_catalog, dict):
            errors.append(f"{rel_path} term_catalog must be an object when present")
            term_catalog = {}
        if isinstance(term_catalog, dict):
            for term_id, term_obj in term_catalog.items():
                if not isinstance(term_obj, dict):
                    errors.append(f"{rel_path} term_catalog.{term_id} must be an object")
                    continue
                if not isinstance(term_obj.get("display"), str) or not term_obj.get("display"):
                    errors.append(f"{rel_path} term_catalog.{term_id}.display must be a non-empty string")
                aliases = term_obj.get("aliases", [])
                if not isinstance(aliases, list) or any(not isinstance(alias, str) for alias in aliases):
                    errors.append(f"{rel_path} term_catalog.{term_id}.aliases must be a string array")
                if not isinstance(term_obj.get("gloss"), str) or not term_obj.get("gloss"):
                    errors.append(f"{rel_path} term_catalog.{term_id}.gloss must be a non-empty string")

        screens = step.get("screens")
        if not isinstance(screens, list):
            errors.append(f"{rel_path} screens must be a list")
            return
        seen_screen_ids: set[str] = set()
        for idx, screen in enumerate(screens):
            prefix = f"{rel_path} screens[{idx}]"
            if not isinstance(screen, dict):
                errors.append(f"{prefix} must be an object")
                continue
            screen_id = screen.get("id")
            if not isinstance(screen_id, str) or not screen_id:
                errors.append(f"{prefix}.id must be a non-empty string")
            elif screen_id in seen_screen_ids:
                errors.append(f"{rel_path} duplicate screen id: {screen_id}")
            else:
                seen_screen_ids.add(screen_id)
            if not isinstance(screen.get("type"), str) or not screen.get("type"):
                errors.append(f"{prefix}.type must be a non-empty string")
            lines = screen.get("lines")
            if not isinstance(lines, list) or any(not isinstance(line, str) for line in lines):
                errors.append(f"{prefix}.lines must be a string array")
            for ref_key in ("introduced_terms", "focus_terms"):
                ref_values = screen.get(ref_key)
                if ref_values is not None:
                    if not isinstance(ref_values, list) or any(not isinstance(item, str) for item in ref_values):
                        errors.append(f"{prefix}.{ref_key} must be a string array when present")
                    elif isinstance(term_catalog, dict):
                        for item in ref_values:
                            if item not in term_catalog:
                                warnings.append(f"{prefix}.{ref_key} references missing term_catalog id {item}")

            if screen.get("type") == "exercise":
                exercise = screen.get("exercise")
                if not isinstance(exercise, dict):
                    errors.append(f"{prefix}.exercise must be an object for exercise screens")
                elif not isinstance(exercise.get("kind"), str) or not exercise.get("kind"):
                    errors.append(f"{prefix}.exercise.kind must be a non-empty string")

            formula = screen.get("formula")
            if formula is not None and not isinstance(formula, dict):
                errors.append(f"{prefix}.formula must be an object when present")
            media = screen.get("media")
            if media is not None and not isinstance(media, dict):
                errors.append(f"{prefix}.media must be an object when present")


    def compute_directory_changes(left_dir: Path, right_dir: Path) -> list[Path]:
        left_files = GuidedStoryTxCli.collect_files(left_dir)
        right_files = GuidedStoryTxCli.collect_files(right_dir)
        changed = []
        for rel_path in sorted(set(left_files) | set(right_files)):
            left_path = left_dir / rel_path
            right_path = right_dir / rel_path
            if not left_path.exists() or not right_path.exists():
                changed.append(rel_path)
                continue
            if left_path.read_bytes() != right_path.read_bytes():
                changed.append(rel_path)
        return changed


    def collect_files(root: Path) -> set[Path]:
        if not root.exists():
            return set()
        files = set()
        for path in root.rglob("*"):
            if path.is_file():
                files.add(path.relative_to(root))
        return files


    def sync_directory(source_dir: Path, dest_dir: Path) -> None:
        dest_dir.mkdir(parents=True, exist_ok=True)
        source_files = GuidedStoryTxCli.collect_files(source_dir)
        dest_files = GuidedStoryTxCli.collect_files(dest_dir)
        for rel_path in sorted(dest_files - source_files):
            (dest_dir / rel_path).unlink()
        for rel_path in sorted(source_files):
            source_path = source_dir / rel_path
            dest_path = dest_dir / rel_path
            GuidedStoryTxCli.ensure_parent(dest_path)
            shutil.copy2(source_path, dest_path)


if __name__ == "__main__":
    raise SystemExit(GuidedStoryTxCli.main())
