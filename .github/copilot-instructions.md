# GitHub Copilot Instructions

Purpose
-------

This document gives concise, repo-specific guidance for using GitHub Copilot (and AI coding assistants) in this repository.

Guidelines
----------

- Conventional commits required: All commit messages must follow the Conventional Commits specification (examples: `feat:`, `fix:`, `docs:`, `chore:`, `refactor:`, `style:`, `test:`). Use a short subject and optional body explaining the change.
- Follow the repository coding style: Match existing code patterns and formatting. For Rust code, run `cargo fmt` and `cargo clippy` (or equivalent) before opening a PR. Keep changes minimal and consistent with current style.
- Use the Rust MCP when needed: If you need deeper analysis, dependency or build checks, or help running Rust tools, use the repository's Rust MCP tooling (cargo-check, cargo-clippy, cargo-fmt, etc.) and the MCP server available to the project. Prefer MCP-based checks for large or cross-crate changes.

When using Copilot / AI assistants
--------------------------------

- Treat suggestions as draft code: review and adapt to this repo's style and tests before committing.
- Add or update tests for behavior-changing changes.
- Keep commits focused: one logical change per commit.

Quick commands
--------------

Run common Rust checks locally before committing:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

If you need assistance from the Rust MCP, call the MCP tools for build/check/format guidance and share output when requesting help.

Contact / exceptions
--------------------

If a change must diverge from these rules, explain why in the PR description and get maintainer approval.

Thank you for keeping contributions consistent and high-quality.
