# Code Review Guidelines

When reviewing pull requests in this repository, please follow these guidelines to ensure our reviews are actionable and focused on correctness.

## When Not to Review
Please skip reviewing pull requests if:
- The changes are exclusively in `monitoring-manifests`.
- The pull request description includes the phrase "AI, PLEASE DO NOT REVIEW THIS PR".

## Primary Goals
Reviews should focus on finding real, actionable issues that could lead to:
- Bugs or logic errors
- Security vulnerabilities
- Data inconsistencies
- Incorrect behavior

## What to Avoid
Please refrain from commenting on:
- **Style and Naming:** Leave style, naming, documentation, and minor refactoring alone unless they clearly prevent a bug. Automated linters and formatters are already in place to handle these.
- **Speculation:** Do not assume behavior in other files, systems, or runtime environments. Base your review strictly on the information visible within the diff.
- **Nitpicks:** Prefer making no comments over adding low-confidence or low-value remarks. Do not guess the author's intent or suggest optional "might be" or "consider" improvements unless there is a provable problem.

## Review Constraints and Rules
- Only comment on lines that were explicitly changed in the pull request.
- Ensure all comments have a high or medium confidence level. Do not leave low-confidence comments.

## Comment Format
For consistency, structure every review comment as follows:

1. **Quote:** Include the exact line or lines from the diff.
2. **Issue:** State the problem clearly in a single sentence.
3. **Fix:** Propose a concrete fix or a safer alternative in 1-3 sentences.
4. **Severity:** Tag the issue with one of the following severities: 🔴 `blocker`, 🟠 `major`, or 🟡 `minor`.
5. **Confidence:** Tag the comment with ✅ `high` or ⚠️ `medium`.
