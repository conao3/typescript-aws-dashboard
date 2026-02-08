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

### Backend Development

#### start development server

```bash
make dev-backend
```

- server: http://localhost:17231
- GraphiQL IDE: http://localhost:17231/admin/graphiql

#### modify code

edit Rust code in `backend/src/`.

#### build and test

```bash
# build only
nix develop -c cargo build --manifest-path backend/Cargo.toml

# run tests
make test

# lint
make lint
```

### Frontend Development

#### start development server

```bash
make dev
```

- frontend: http://localhost:17232

#### modify code

edit TypeScript code in `frontend/app/`.

#### build and test

```bash
# build only
nix develop -c pnpm build -C frontend

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
make dev         # start frontend dev server
make dev-backend # start backend dev server
```

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
nix develop -c cargo test --manifest-path backend/Cargo.toml
```

### frontend tests

```bash
nix develop -c pnpm test -C frontend
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
steam-run make dev
```

however, do not write `steam-run` in Makefile.

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

both development servers support hot reload:

- backend: auto-restart on code changes (to be confirmed)
- frontend: auto-reload on code changes

### concurrent development

start both servers in different terminals for concurrent development:

```bash
# terminal 1
make dev-backend

# terminal 2
make dev
```

## Reference Resources

- [coordination.md](./coordination.md) - coordination guide for multiple Claude Code instances
- [task-management.md](./task-management.md) - task management guide
- [architecture.md](./architecture.md) - architecture documentation
- [api.md](./api.md) - API specification (to be created)
