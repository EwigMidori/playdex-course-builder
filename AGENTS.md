
## Execution gate — confirm before implementation

Before performing implementation work, behavior-changing edits, refactors,
asset changes, or other non-trivial execution, the agent MUST first state the
proposed solution and obtain explicit user confirmation.

The only exception is when the user has clearly and explicitly asked to skip
confirmation and proceed directly.

This rule applies even if the agent believes the next step is obvious. Do not
start execution first and explain later.

## Debugging — evidence before edits

When fixing bugs, especially frontend behavior, data loading, chart rendering,
strategy parity, or backtest/export mismatches, do not guess-and-patch.
All behavior-changing fixes MUST be based on evidence. Use this mandatory
evidence-first loop:

1. **State the observed failure** with the exact error text, screenshot detail,
   log line, failing URL, failing command, or failing test.

2. **Form one narrow hypothesis** that explains the observation. The hypothesis
   must name the suspected file/function/path and the specific mechanism.

3. **Run the smallest verification** before editing. The verification MUST
   produce concrete evidence such as browser console logs, network
   paths/status/content-type, screenshots, local file existence checks, focused
   tests, or command output. For frontend visual regressions, inspect runtime
   state or capture a screenshot; do not infer the result from code alone.

4. **Do not perform bug fixing until the hypothesis is supported by evidence.**
   If the evidence is not sufficient, add diagnostics/logging first.
   Diagnostic-only changes are allowed when they are needed to get evidence;
   behavior-changing fixes are forbidden until evidence supports the hypothesis.

5. **After the edit, verify the same failure mode** with the relevant build,
   test, request, log, or screenshot. Report what evidence changed.

Do not hide missing files or invalid responses with broad fallback logic. If a
file is part of the expected contract, fail early with a precise path and reason.
Fallbacks are allowed only for explicitly supported legacy formats, and they
MUST log which path was chosen.

When a change affects async loading or staged rendering, keep initial-load errors
separate from background-update errors. A background failure should not erase
already valid UI unless the verified contract says the whole view is invalid.
