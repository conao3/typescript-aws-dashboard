# Project Instructions for Claude Code

this file contains project-specific instructions for Claude Code instances working on this project.

## Documentation

before starting any work, read the project documentation in [docs/README.md](./docs/README.md).

key documents:

- [docs/requirements.md](./docs/requirements.md) - project requirements and scope (READ THIS FIRST)
- [docs/coordination.md](./docs/coordination.md) - coordination guidelines for multiple Claude Code instances
- [docs/tasks.md](./docs/tasks.md) - current task tracking and assignments
- [docs/architecture.md](./docs/architecture.md) - system architecture and multi-tenancy design
- [docs/multi-tenancy.md](./docs/multi-tenancy.md) - multi-tenancy implementation guide
- [docs/database.md](./docs/database.md) - database setup and sqldef schema management
- [docs/development.md](./docs/development.md) - development workflow
- [docs/api.md](./docs/api.md) - GraphQL API specification

## Role-Based Work

this project uses role-based collaboration:

- **backend lead**: responsible for Rust backend code
- **frontend lead**: responsible for TypeScript frontend code
- **devops lead**: responsible for infrastructure and integration

check your assigned role in [docs/coordination.md](./docs/coordination.md) before starting work.

## Communication

all task assignments and coordination go through the PdM (product manager). always communicate with PdM about:

- task selection
- progress updates
- blockers
- major design decisions

## Git Worktree Setup

**CRITICAL**: to avoid staging area conflicts, each Claude Code instance must work in a separate git worktree.

### setup your worktree (one time only)

```bash
# backend lead
git worktree add -b work/backend worktree-backend master
cd worktree-backend

# frontend lead
git worktree add -b work/frontend worktree-frontend master
cd worktree-frontend

# devops lead
git worktree add -b work/devops worktree-devops master
cd worktree-devops
```

### daily workflow in worktree

1. navigate to your worktree:
   ```bash
   cd worktree-backend  # or worktree-frontend, worktree-devops
   ```

2. sync with master:
   ```bash
   git rebase master
   ```

3. make changes in your responsibility area

4. stage and commit:
   ```bash
   git add <your-files>
   git commit -m "message"
   ```

5. push your branch:
   ```bash
   git push -u origin work/backend
   ```

6. integrate to master (after work is complete):
   ```bash
   # rebase work branch onto master
   git rebase master

   # merge to master
   cd ../
   git merge work/backend
   git push
   ```

see [docs/worktree.md](./docs/worktree.md) for complete guide.

## Development Server Restrictions

**IMPORTANT**: do NOT start backend or frontend development servers. the user is already running them in watch mode:

- backend server (port 17231) is already running
- frontend server (port 17232) is already running

Claude Code instances should never execute:

- `make dev-backend`
- `make dev`
- any command that starts these servers

the servers automatically reload when you make code changes (including in worktrees).

## Commit Policy

**CRITICAL**: only commit changes related to your role and assigned tasks.

### what to commit

each Claude Code instance should ONLY commit:

- files within their responsibility area:
  - **backend lead**: `backend/` directory files
  - **frontend lead**: `frontend/` directory files
  - **devops lead**: root-level configuration files (Makefile, docker-compose.yml, CI/CD configs)
- documentation updates that you created or are assigned to maintain

### what NOT to commit

DO NOT commit:

- changes made by other Claude Code instances
- changes outside your responsibility area
- documentation created/maintained by other roles (check docs/README.md for ownership)
- temporary files or build artifacts
- configuration changes you didn't make

### before committing

**ensure you are in your worktree**:

```bash
pwd  # must show worktree-backend, worktree-frontend, or worktree-devops
git branch --show-current  # must show work/backend, work/frontend, or work/devops
```

then:

1. run `git status` to see all changes
2. run `git diff` to review each change
3. only stage files (`git add`) that are within your responsibility
4. verify staged changes with `git diff --cached`
5. create commit only when instructed by user

### example workflow

```bash
# 0. ensure in correct worktree
cd worktree-backend
pwd  # verify location

# 1. check all changes
git status

# 2. only add your files
git add backend/src/auth.rs backend/src/middleware.rs

# 3. verify staged changes
git diff --cached

# 4. wait for user instruction to commit
```

### when in doubt

if unsure whether a file should be committed:

- check if it's in your responsibility area (see docs/coordination.md)
- ask PdM before committing
- prefer to commit less rather than more

## Getting Started

1. read [docs/README.md](./docs/README.md) for documentation overview
2. follow setup instructions in [docs/development.md](./docs/development.md)
3. check your role and responsibilities in [docs/coordination.md](./docs/coordination.md)
4. coordinate with PdM for task assignment
