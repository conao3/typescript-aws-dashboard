# Development Guide

this document describes the project development environment setup and development flow.

## Prerequisites

- Nix (with flakes enabled)
- Git

## Initial Setup

### 1. clone repository

```bash
git clone <repository-url>
cd typescript-aws-dashboard
```

### 2. enter Nix development environment

```bash
nix develop
```

### 3. install dependencies

```bash
make setup
```

this installs frontend dependencies (pnpm install).

## Development Flow

**IMPORTANT**: the user runs backend and frontend servers in watch mode. Claude Code instances should NEVER start these servers.

### Backend Development

#### development server

the backend server is already running at:

- server: http://localhost:17231
- GraphiQL IDE: http://localhost:17231/admin/graphiql

**do NOT execute** `make dev-backend` - the server is already running and will auto-reload on code changes.

#### modify code

edit Rust code in `backend/src/`.

#### build and test

```bash
# build only
cargo build --manifest-path backend/Cargo.toml

# run tests
make test

# lint
make lint
```

### Frontend Development

#### development server

the frontend server is already running at:

- frontend: http://localhost:17232

**do NOT execute** `make dev` - the server is already running and will auto-reload on code changes.

#### modify code

edit TypeScript code in `frontend/src/`.

#### build and test

```bash
# build only
pnpm -C frontend build

# run tests
make test

# lint
make lint
```

## Coding Conventions

### common

- files must end with a newline
- no trailing whitespace at end of lines
- adding comments is prohibited (editing existing comments only)
- avoid using variables unless obviously redundant

### Rust

- (add project-specific conventions if any)

### TypeScript

- use `as const` to narrow type information
- (add project-specific conventions if any)

## Makefile

commands used in the project are defined in the Makefile.

### main commands

```bash
make help        # show available commands
make setup       # project setup
make build       # build project
make build-release  # release build
make test        # run tests
make fmt         # format code
make lint        # run lint
make check       # check Nix flake
make clean       # clean build artifacts
```

note: `make dev` and `make dev-backend` are available but should NOT be used by Claude Code instances as servers are already running in watch mode.

### Makefile rules

- `.PHONY` targets are declared for each target
- use `-C` option
- place Makefile in each directory, keep root Makefile concise

## Git Operations

### branching strategy

- `main`: main branch (protected)
- `feature/*`: feature additions
- `fix/*`: bug fixes

see [coordination.md](./coordination.md) for details.

### commit

```bash
git add <files>
git diff --cached  # check changes
git commit -m "commit message"
```

#### commit rules

- commit messages start with lowercase English
- never add Co-authored-by
- check changes with `git diff --cached` before committing
- Claude Code reports commit message to user

### pull request

```bash
# create PR (using gh CLI)
gh pr create --title "title" --body "description"
```

## Testing

### backend tests

```bash
cargo test --manifest-path backend/Cargo.toml
```

### frontend tests

```bash
pnpm -C frontend test
```

### run all tests

```bash
make test
```

## Format and Lint

### format

```bash
make fmt
```

formats entire codebase using Nix treefmt.

### lint

```bash
make lint
```

- Rust: `cargo clippy`
- TypeScript: `pnpm lint`

## Troubleshooting

### commands not working in Nix environment

on NixOS, prefix with `steam-run`:

```bash
steam-run make build
```

however, do not write `steam-run` in Makefile.

note: Claude Code instances should not start development servers as they are already running.

### dependency errors

```bash
make clean
make setup
```

clean build may resolve issues.

### port already in use

if backend (17231) or frontend (17232) ports are in use:

```bash
# check port usage
lsof -i :17231
lsof -i :17232

# kill process
kill <PID>
```

## Development Tips

### using GraphiQL

during backend development, use GraphiQL IDE (http://localhost:17231/admin/graphiql) to:

- test queries and mutations
- check schema documentation
- debug requests/responses

### hot reload

both development servers support hot reload and are already running in watch mode:

- backend: auto-restart on code changes
- frontend: auto-reload on code changes

Claude Code instances should only modify code files. the servers will automatically detect changes and reload.

## Reference Resources

- [coordination.md](./coordination.md) - coordination guide for multiple Claude Code instances
- [task-management.md](./task-management.md) - task management guide
- [architecture.md](./architecture.md) - architecture documentation
- [api.md](./api.md) - API specification (to be created)
