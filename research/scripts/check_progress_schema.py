#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_PROGRESS_PATH = ROOT / "docs" / "progress.json"

TOP_LEVEL_KEYS = {
    "schema_version",
    "project",
    "planning_mode",
    "last_updated",
    "status_model",
    "execution_rules",
    "workstreams",
    "milestones",
    "phases",
}

STATUS_MODEL_KEYS = {"phase_status", "task_status"}
EXECUTION_RULE_KEYS = {"summary", "rules"}
WORKSTREAM_KEYS = {"id", "name", "scope"}
MILESTONE_KEYS = {"id", "name", "phase", "done_when"}
PHASE_KEYS = {
    "id",
    "name",
    "description",
    "status",
    "objective",
    "entry_criteria",
    "exit_criteria",
    "deliverables",
    "parallel_tracks",
    "gate_checks",
    "tasks",
}
TASK_KEYS = {
    "id",
    "title",
    "description",
    "workstream",
    "crate_scope",
    "depends_on",
    "status",
    "done_when",
    "artifacts",
}

PHASE_ID_RE = re.compile(r"^P\d+[A-Z]?$")
TASK_ID_RE = re.compile(r"^P\d+[A-Z]?-T\d+[A-Z]?$")


class ValidationError(Exception):
    pass


class ProgressSchemaValidatorCli:
    def main() -> int:
        parser = argparse.ArgumentParser(
            description=(
                "Validate that a progress roadmap JSON keeps the required execution "
                "protocol fields and structural invariants."
            )
        )
        parser.add_argument(
            "path",
            nargs="?",
            default=str(DEFAULT_PROGRESS_PATH),
            help="Path to the roadmap JSON file. Defaults to docs/progress.json.",
        )
        args = parser.parse_args()

        path = Path(args.path).resolve()

        try:
            payload = json.loads(path.read_text(encoding="utf-8"))
        except FileNotFoundError:
            print(f"error: file not found: {path}", file=sys.stderr)
            return 1
        except json.JSONDecodeError as exc:
            print(
                f"error: invalid JSON at {path}:{exc.lineno}:{exc.colno}: {exc.msg}",
                file=sys.stderr,
            )
            return 1

        errors: list[str] = []
        ProgressSchemaValidatorCli.validate_progress(payload, errors)

        if errors:
            print(f"progress schema check failed for {path}", file=sys.stderr)
            for error in errors:
                print(f"- {error}", file=sys.stderr)
            return 1

        print(f"progress schema check passed for {path}")
        return 0


    def validate_progress(payload: Any, errors: list[str]) -> None:
        if not isinstance(payload, dict):
            errors.append("top-level JSON value must be an object")
            return

        ProgressSchemaValidatorCli.expect_exact_keys("top-level", payload, TOP_LEVEL_KEYS, errors)
        ProgressSchemaValidatorCli.expect_type("schema_version", payload.get("schema_version"), int, errors)
        ProgressSchemaValidatorCli.expect_type("project", payload.get("project"), str, errors)
        ProgressSchemaValidatorCli.expect_type("planning_mode", payload.get("planning_mode"), str, errors)
        ProgressSchemaValidatorCli.expect_type("last_updated", payload.get("last_updated"), str, errors)

        status_model = payload.get("status_model")
        if isinstance(status_model, dict):
            ProgressSchemaValidatorCli.expect_exact_keys("status_model", status_model, STATUS_MODEL_KEYS, errors)
            ProgressSchemaValidatorCli.validate_string_list("status_model.phase_status", status_model.get("phase_status"), errors)
            ProgressSchemaValidatorCli.validate_string_list("status_model.task_status", status_model.get("task_status"), errors)
        else:
            errors.append("status_model must be an object")

        execution_rules = payload.get("execution_rules")
        if isinstance(execution_rules, dict):
            ProgressSchemaValidatorCli.expect_exact_keys("execution_rules", execution_rules, EXECUTION_RULE_KEYS, errors)
            ProgressSchemaValidatorCli.expect_type("execution_rules.summary", execution_rules.get("summary"), str, errors)
            ProgressSchemaValidatorCli.validate_string_list("execution_rules.rules", execution_rules.get("rules"), errors)
        else:
            errors.append("execution_rules must be an object")

        workstreams = payload.get("workstreams")
        workstream_ids = ProgressSchemaValidatorCli.validate_workstreams(workstreams, errors)

        milestones = payload.get("milestones")
        milestone_phase_ids = ProgressSchemaValidatorCli.validate_milestones(milestones, errors)

        phases = payload.get("phases")
        phase_ids, task_ids = ProgressSchemaValidatorCli.validate_phases(phases, workstream_ids, errors)

        for milestone_phase_id in milestone_phase_ids:
            if milestone_phase_id not in phase_ids:
                errors.append(
                    f"milestone.phase references unknown phase id `{milestone_phase_id}`"
                )

        valid_dependency_ids = phase_ids | task_ids
        if isinstance(phases, list):
            for phase_index, phase in enumerate(phases):
                if not isinstance(phase, dict):
                    continue
                phase_id = phase.get("id", f"<phase#{phase_index + 1}>")
                tasks = phase.get("tasks")
                if not isinstance(tasks, list):
                    continue
                for task_index, task in enumerate(tasks):
                    if not isinstance(task, dict):
                        continue
                    task_id = task.get("id", f"{phase_id}-<task#{task_index + 1}>")
                    depends_on = task.get("depends_on")
                    if not isinstance(depends_on, list):
                        continue
                    for dependency in depends_on:
                        if not isinstance(dependency, str):
                            continue
                        if dependency not in valid_dependency_ids:
                            errors.append(
                                f"task `{task_id}` depends_on unknown id `{dependency}`"
                            )


    def validate_workstreams(value: Any, errors: list[str]) -> set[str]:
        ids: set[str] = set()
        if not isinstance(value, list):
            errors.append("workstreams must be an array")
            return ids

        for index, item in enumerate(value):
            label = f"workstreams[{index}]"
            if not isinstance(item, dict):
                errors.append(f"{label} must be an object")
                continue
            ProgressSchemaValidatorCli.expect_exact_keys(label, item, WORKSTREAM_KEYS, errors)
            workstream_id = item.get("id")
            ProgressSchemaValidatorCli.expect_type(f"{label}.id", workstream_id, str, errors)
            ProgressSchemaValidatorCli.expect_type(f"{label}.name", item.get("name"), str, errors)
            ProgressSchemaValidatorCli.validate_string_list(f"{label}.scope", item.get("scope"), errors)
            if isinstance(workstream_id, str):
                if workstream_id in ids:
                    errors.append(f"duplicate workstream id `{workstream_id}`")
                ids.add(workstream_id)
        return ids


    def validate_milestones(value: Any, errors: list[str]) -> set[str]:
        phase_ids: set[str] = set()
        if not isinstance(value, list):
            errors.append("milestones must be an array")
            return phase_ids

        seen_ids: set[str] = set()
        for index, item in enumerate(value):
            label = f"milestones[{index}]"
            if not isinstance(item, dict):
                errors.append(f"{label} must be an object")
                continue
            ProgressSchemaValidatorCli.expect_exact_keys(label, item, MILESTONE_KEYS, errors)
            milestone_id = item.get("id")
            ProgressSchemaValidatorCli.expect_type(f"{label}.id", milestone_id, str, errors)
            ProgressSchemaValidatorCli.expect_type(f"{label}.name", item.get("name"), str, errors)
            phase_id = item.get("phase")
            ProgressSchemaValidatorCli.expect_type(f"{label}.phase", phase_id, str, errors)
            ProgressSchemaValidatorCli.validate_string_list(f"{label}.done_when", item.get("done_when"), errors)
            if isinstance(milestone_id, str):
                if milestone_id in seen_ids:
                    errors.append(f"duplicate milestone id `{milestone_id}`")
                seen_ids.add(milestone_id)
            if isinstance(phase_id, str):
                phase_ids.add(phase_id)
        return phase_ids


    def validate_phases(
        value: Any,
        workstream_ids: set[str],
        errors: list[str],
    ) -> tuple[set[str], set[str]]:
        phase_ids: set[str] = set()
        task_ids: set[str] = set()

        if not isinstance(value, list):
            errors.append("phases must be an array")
            return phase_ids, task_ids

        for phase_index, phase in enumerate(value):
            label = f"phases[{phase_index}]"
            if not isinstance(phase, dict):
                errors.append(f"{label} must be an object")
                continue

            ProgressSchemaValidatorCli.expect_exact_keys(label, phase, PHASE_KEYS, errors)
            phase_id = phase.get("id")
            ProgressSchemaValidatorCli.expect_type(f"{label}.id", phase_id, str, errors)
            if isinstance(phase_id, str):
                if not PHASE_ID_RE.match(phase_id):
                    errors.append(f"{label}.id `{phase_id}` does not match phase id pattern")
                if phase_id in phase_ids:
                    errors.append(f"duplicate phase id `{phase_id}`")
                phase_ids.add(phase_id)

            ProgressSchemaValidatorCli.expect_type(f"{label}.name", phase.get("name"), str, errors)
            ProgressSchemaValidatorCli.expect_type(f"{label}.description", phase.get("description"), str, errors)
            ProgressSchemaValidatorCli.expect_type(f"{label}.status", phase.get("status"), str, errors)
            ProgressSchemaValidatorCli.expect_type(f"{label}.objective", phase.get("objective"), str, errors)
            ProgressSchemaValidatorCli.validate_string_list(f"{label}.entry_criteria", phase.get("entry_criteria"), errors)
            ProgressSchemaValidatorCli.validate_string_list(f"{label}.exit_criteria", phase.get("exit_criteria"), errors)
            ProgressSchemaValidatorCli.validate_string_list(f"{label}.deliverables", phase.get("deliverables"), errors)
            ProgressSchemaValidatorCli.validate_string_list(f"{label}.parallel_tracks", phase.get("parallel_tracks"), errors)
            ProgressSchemaValidatorCli.validate_string_list(f"{label}.gate_checks", phase.get("gate_checks"), errors)

            tasks = phase.get("tasks")
            if not isinstance(tasks, list):
                errors.append(f"{label}.tasks must be an array")
                continue

            for task_index, task in enumerate(tasks):
                task_label = f"{label}.tasks[{task_index}]"
                if not isinstance(task, dict):
                    errors.append(f"{task_label} must be an object")
                    continue

                ProgressSchemaValidatorCli.expect_exact_keys(task_label, task, TASK_KEYS, errors)
                task_id = task.get("id")
                ProgressSchemaValidatorCli.expect_type(f"{task_label}.id", task_id, str, errors)
                if isinstance(task_id, str):
                    if not TASK_ID_RE.match(task_id):
                        errors.append(
                            f"{task_label}.id `{task_id}` does not match task id pattern"
                        )
                    if task_id in task_ids:
                        errors.append(f"duplicate task id `{task_id}`")
                    task_ids.add(task_id)

                ProgressSchemaValidatorCli.expect_type(f"{task_label}.title", task.get("title"), str, errors)
                ProgressSchemaValidatorCli.expect_type(f"{task_label}.description", task.get("description"), str, errors)
                workstream = task.get("workstream")
                ProgressSchemaValidatorCli.expect_type(f"{task_label}.workstream", workstream, str, errors)
                if isinstance(workstream, str) and workstream not in workstream_ids:
                    errors.append(
                        f"{task_label}.workstream references unknown workstream `{workstream}`"
                    )
                ProgressSchemaValidatorCli.validate_string_list(f"{task_label}.crate_scope", task.get("crate_scope"), errors)
                ProgressSchemaValidatorCli.validate_string_list(f"{task_label}.depends_on", task.get("depends_on"), errors)
                ProgressSchemaValidatorCli.expect_type(f"{task_label}.status", task.get("status"), str, errors)
                ProgressSchemaValidatorCli.validate_string_list(f"{task_label}.done_when", task.get("done_when"), errors)
                ProgressSchemaValidatorCli.validate_string_list(f"{task_label}.artifacts", task.get("artifacts"), errors)

        return phase_ids, task_ids


    def expect_exact_keys(
        label: str,
        payload: dict[str, Any],
        required_keys: set[str],
        errors: list[str],
    ) -> None:
        actual_keys = set(payload.keys())
        missing = sorted(required_keys - actual_keys)
        extra = sorted(actual_keys - required_keys)
        if missing:
            errors.append(f"{label} is missing keys: {', '.join(missing)}")
        if extra:
            errors.append(f"{label} has unexpected keys: {', '.join(extra)}")


    def expect_type(label: str, value: Any, expected_type: type, errors: list[str]) -> None:
        if not isinstance(value, expected_type):
            errors.append(
                f"{label} must be {expected_type.__name__}, got {type(value).__name__}"
            )


    def validate_string_list(label: str, value: Any, errors: list[str]) -> None:
        if not isinstance(value, list):
            errors.append(f"{label} must be an array of strings")
            return
        for index, item in enumerate(value):
            if not isinstance(item, str):
                errors.append(
                    f"{label}[{index}] must be string, got {type(item).__name__}"
                )


if __name__ == "__main__":
    raise SystemExit(ProgressSchemaValidatorCli.main())
