# Git Worktree Guide

this document explains how to use git worktrees to avoid staging area conflicts between multiple Claude Code instances.

## Problem

when multiple Claude Code instances work in the same repository, they share a single git staging area. this causes conflicts when:
- instance A stages files for commit
- instance B stages different files
- both try to commit at the same time

## Solution: Git Worktrees

git worktrees allow multiple working directories from the same repository, each with its own:
- working directory
- staging area (`git add`)
- HEAD (current branch/commit)

all worktrees share the same `.git` directory (history, branches, remotes).

## Worktree Structure

```
typescript-aws-dashboard/          # main directory (for PdM)
├── worktree-backend/             # backend lead workspace
├── worktree-frontend/            # frontend lead workspace
└── worktree-devops/              # devops lead workspace
```

each worktree has its own branch:
- main directory: `main` branch
- worktree-backend: `work/backend` branch
- worktree-frontend: `work/frontend` branch
- worktree-devops: `work/devops` branch

## Initial Setup

**IMPORTANT**: only set up your assigned worktree once.

### backend lead

```bash
cd typescript-aws-dashboard
git worktree add -b work/backend worktree-backend main
cd worktree-backend
```

### frontend lead

```bash
cd typescript-aws-dashboard
git worktree add -b work/frontend worktree-frontend main
cd worktree-frontend
```

### devops lead

```bash
cd typescript-aws-dashboard
git worktree add -b work/devops worktree-devops main
cd worktree-devops
```

### PdM

stays in the main directory (no worktree needed).

## Daily Workflow

### 1. navigate to your worktree

```bash
# backend lead
cd typescript-aws-dashboard/worktree-backend

# frontend lead
cd typescript-aws-dashboard/worktree-frontend

# devops lead
cd typescript-aws-dashboard/worktree-devops
```

### 2. sync with main branch

before starting work, sync your branch with main:

```bash
git fetch origin
git rebase origin/main
```

if there are conflicts:

```bash
# resolve conflicts in files
git add <resolved-files>
git rebase --continue
```

### 3. make changes

edit files in your responsibility area:
- backend lead: `backend/` directory
- frontend lead: `frontend/` directory
- devops lead: root-level config files

the changes are isolated to your worktree.

### 4. stage and commit

```bash
# check changes
git status
git diff

# stage your files only
git add backend/src/auth.rs  # example

# verify staged changes
git diff --cached

# commit (when instructed by user)
git commit -m "implement authentication"
```

### 5. push your work branch

```bash
# first time
git push -u origin work/backend

# subsequent pushes
git push
```

### 6. integrate to main

option 1: create pull request

```bash
gh pr create --base main --head work/backend --title "..." --body "..."
```

option 2: merge directly (after approval)

```bash
cd typescript-aws-dashboard  # go to main directory
git checkout main
git merge work/backend
git push origin main
```

### 7. sync after merge

after your changes are merged to main:

```bash
cd worktree-backend  # or your worktree
git fetch origin
git rebase origin/main
```

## Worktree Commands

### list all worktrees

```bash
git worktree list
```

output example:

```
/home/user/typescript-aws-dashboard                    abc1234 [main]
/home/user/typescript-aws-dashboard/worktree-backend  def5678 [work/backend]
/home/user/typescript-aws-dashboard/worktree-frontend ghi9012 [work/frontend]
```

### check current worktree

```bash
pwd  # shows which worktree you're in
git branch --show-current  # shows current branch
```

### remove worktree

if you need to remove a worktree:

```bash
cd typescript-aws-dashboard  # go to main directory
git worktree remove worktree-backend
git branch -d work/backend  # delete the branch if needed
```

### prune stale worktrees

if worktrees are manually deleted:

```bash
git worktree prune
```

## Development Server Notes

the user runs development servers from the main directory. they watch all files including those in worktrees.

when you edit files in your worktree:
- backend changes: backend server auto-reloads
- frontend changes: frontend server auto-reloads

you should NEVER start servers yourself.

## Advantages

1. **isolated staging**: each worktree has its own staging area
2. **no conflicts**: multiple Claude Code instances can work simultaneously
3. **independent commits**: commit whenever ready without blocking others
4. **same repository**: all share history, branches, and remotes
5. **efficient**: no need to clone repository multiple times

## Troubleshooting

### "worktree already exists"

```bash
git worktree list  # check existing worktrees
```

if the worktree directory exists but git doesn't know about it:

```bash
git worktree prune
rm -rf worktree-backend  # or the problematic directory
git worktree add -b work/backend worktree-backend main
```

### "branch already exists"

if the branch already exists:

```bash
# use existing branch
git worktree add worktree-backend work/backend

# or delete and recreate
git branch -d work/backend
git worktree add -b work/backend worktree-backend main
```

### accidentally committed in wrong worktree

if you committed changes outside your responsibility:

```bash
# undo last commit (keeps changes)
git reset HEAD~1

# check you're in correct worktree
pwd
git branch --show-current

# move to correct worktree
cd ../worktree-correct

# apply changes
git cherry-pick <commit-hash>
```

### worktree out of sync

if your worktree is far behind main:

```bash
git fetch origin
git rebase origin/main
```

if rebase is too complex:

```bash
# create backup branch
git branch backup-work

# reset to main
git reset --hard origin/main

# cherry-pick your commits
git cherry-pick <your-commits>
```

## Best Practices

1. **always work in your assigned worktree**
2. **sync with main before starting new work**
3. **commit small and frequently**
4. **push regularly to avoid losing work**
5. **never edit files outside your responsibility area**
6. **verify current worktree before committing** (`pwd` and `git branch`)

## Reference

- official git documentation: https://git-scm.com/docs/git-worktree
- worktree tutorial: https://git-scm.com/book/en/v2/Git-Tools-Advanced-Merging
