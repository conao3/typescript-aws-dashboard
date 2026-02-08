#!/usr/bin/env bash
set -euo pipefail

echo "Setting up git worktrees for Claude Code instances..."
echo ""

# Check if worktrees already exist
if [ -d "worktree-backend" ] || [ -d "worktree-frontend" ] || [ -d "worktree-devops" ]; then
    echo "Warning: Some worktrees already exist."
    echo "Current worktrees:"
    git worktree list
    echo ""
    read -p "Remove and recreate all worktrees? (y/N): " confirm
    if [[ $confirm == [yY] ]]; then
        echo "Removing existing worktrees..."
        git worktree remove worktree-backend 2>/dev/null || true
        git worktree remove worktree-frontend 2>/dev/null || true
        git worktree remove worktree-devops 2>/dev/null || true
        git worktree prune
        git branch -D work/backend 2>/dev/null || true
        git branch -D work/frontend 2>/dev/null || true
        git branch -D work/devops 2>/dev/null || true
    else
        echo "Cancelled."
        exit 0
    fi
fi

# Create worktrees
echo "Creating worktree-backend (branch: work/backend)..."
git worktree add -b work/backend worktree-backend main

echo "Creating worktree-frontend (branch: work/frontend)..."
git worktree add -b work/frontend worktree-frontend main

echo "Creating worktree-devops (branch: work/devops)..."
git worktree add -b work/devops worktree-devops main

echo ""
echo "Worktrees created successfully!"
echo ""
echo "List of worktrees:"
git worktree list
echo ""
echo "Usage:"
echo "  Backend lead:  cd worktree-backend"
echo "  Frontend lead: cd worktree-frontend"
echo "  DevOps lead:   cd worktree-devops"
echo "  PdM:           (stay in main directory)"
echo ""
echo "See docs/worktree.md for detailed usage guide."
