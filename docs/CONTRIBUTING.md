# Contributing

## Set Up Environment

```bash
git clone https://github.com/EthanDGee/kannab.git
cd kannab
```

The project requires **Rust 1.95.0 or later** (edition 2024).

```bash
# Build the project
cargo build

# Run the project
cargo run

# Run tests
cargo test

# Run lints (must pass before pushing)
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt
```

### Prek

This project uses [prek](https://prek.j178.dev/) instead of pre-commit for git hooks.
It is a significantly faster alternative written in Rust, with a single
binary and no Python dependency.

**Installation:**

```bash
cargo install --locked prek
```

Or use the standalone installer script — see the [install guide](https://prek.j178.dev/#installation).

**Setup:**

```bash
# Install git shims (runs hooks automatically on commit/push)
prek install
```

**Useful commands:**

```bash
# Run all hooks against staged files
prek run

# Run all hooks against all files
prek run --all-files

# Run a specific hook (e.g. typos)
prek run typos

# Run hooks for files changed in the last commit
prek run --last-commit

# List configured hooks
prek list
```

**What the hooks check (pre-commit stage):**

| Hook | Description |
| ---- | ----------- |
| `trailing-whitespace` | Strips trailing whitespace |
| `end-of-file-fixer` | Ensures files end with a newline |
| `check-toml` | Validates TOML files |
| `check-yaml` | Validates YAML files |
| `check-added-large-files` | Blocks files over 500 KB |
| `typos` | Spell checker |
| `markdownlint-fix` | Auto-fixes most markdown style issues |
| `cargo fmt` | Formats Rust source code |

**Pre-push stage:**

| Hook | Description |
| ---- | ----------- |
| `cargo clippy` | Runs clippy lints (`-D warnings`) |
| `cargo test` | Runs the full test suite |

## Commit Guidelines

It is recommended to use [lazygit](https://github.com/jesseduffield/lazygit) for
composing commits, this is the tool as it is far easier to follow standards.

Commits follow a conventional format:

```text
<type>: <brief description>

<body with details (This is required)>
```

The `!` suffix denotes a **breaking change** (e.g. `feat!:`, `refactor!:`).

| Type | Description |
| ---- | ----------- |
| `feat` | A new feature |
| `refactor` | Code change that is not a feature or bug fix |
| `bug` | A bug fix |
| `docs` | Documentation changes |
| `repo` | Repository / infrastructure / CI changes |
| `test` | Adding or updating tests |
| `release` | Release squashes used when merging into `main` |

**Examples from the repository:**

```text
feat: added ability to toggle checklist items
refactor!: moved state specific modals to their respective files
bug: fixed prek clippy warnings
docs: updated docs for new/unlabeled function and structs
repo: added prek hooks
test: Added board state tests
release: version 0.3.1
```

## Testing

- **Non-UI code** — use standard Rust testing with `cargo test`.
  Place unit tests in a `tests` module at the bottom of each source file.
- **UI / rendering code** — test manually by running the application and
  verifying the terminal output behaves as expected.

## Pull Requests

1. Open an issue first to discuss the proposed change.
2. Keep pull requests small and focused on a single concern.
3. PRs should the `development` branch not into `main`.
4. The PR title and description should follow the commit standards above and
   provide a clear summary of the changes.

## AI Policy

The use of AI-assisted tooling is permitted but discouraged.
If AI is used,
all generated code must receive a high level of scrutiny and review before
being committed.

**AI generated commits and pull requests are not allowed.
If you can't explain the changes on your own you have not had an active enough
role in development.**
