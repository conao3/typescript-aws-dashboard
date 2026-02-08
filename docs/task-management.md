# Task Management Guide

this project uses vibe_kanban MCP server for task management.

## vibe_kanban Overview

vibe_kanban is an MCP server for managing tasks and projects. multiple Claude Code instances can access tasks in the same project and work collaboratively.

## Basic Operations

### check projects

```
list_projects
```

lists all projects. check the `project_id` for this project.

### list tasks

```
list_tasks project_id=<project-id>
```

optional parameters:
- `status`: filter (todo, inprogress, inreview, done, cancelled)
- `limit`: maximum number of results (default: 50)

### get task details

```
get_task task_id=<task-id>
```

retrieves complete task description and execution status.

### create task

```
create_task project_id=<project-id> title="..." description="..."
```

creates a new task. `project_id` is required.

### update task

```
update_task task_id=<task-id> status=inprogress
```

updatable fields:
- `title`: title
- `description`: description
- `status`: status (todo, inprogress, inreview, done, cancelled)

### delete task

```
delete_task task_id=<task-id>
```

deletes unnecessary tasks.

## Workflow

### 1. select a task to work on

```
list_tasks project_id=<project-id> status=todo
```

check unstarted tasks corresponding to your role.

### 2. start the task

```
update_task task_id=<task-id> status=inprogress
```

always update status when starting work to notify other Claude Code instances.

### 3. perform the work

implement according to task description.

### 4. update status when completed

```
update_task task_id=<task-id> status=done
```

update status to `done` when work is completed.

### 5. if review is needed

```
update_task task_id=<task-id> status=inreview
```

set to `inreview` when review is needed, such as after creating a PR.

## Task Priorities

select tasks in the following priority order:

1. **blocker tasks**: tasks that other tasks depend on
2. **role-specific tasks**: backend/frontend/devops
3. **high priority tasks**: specified by PdM
4. **old tasks**: tasks that have been in todo state for a long time

## Repositories and Workspaces

### list repositories

```
list_repos project_id=<project-id>
```

check repositories related to the project.

### get repository details

```
get_repo repo_id=<repo-id>
```

check setup scripts, cleanup scripts, dev server scripts, etc.

### start workspace session

```
start_workspace_session
  task_id=<task-id>
  executor=CLAUDE_CODE
  repos=[{repo_id: "<repo-id>", base_branch: "main"}]
```

creates a workspace environment for the task and starts work.

## Best Practices

### task granularity

- ideal task size: completable in 1-3 hours
- split tasks that are too large into multiple subtasks
- merge tasks that are too small with related tasks

### task descriptions

write clear task descriptions:
- **purpose**: why this task is needed
- **deliverables**: what to create
- **acceptance criteria**: when is it done
- **technical constraints**: constraints to consider

example:
```
title: implement user authentication API
description: |
  ## purpose
  implement GraphQL API to provide user authentication

  ## deliverables
  - login mutation
  - logout mutation
  - currentUser query
  - JWT token generation and verification

  ## acceptance criteria
  - can log in with correct credentials
  - returns error with invalid credentials
  - maintains authentication state with token
  - unit tests pass

  ## technical constraints
  - JWT expiration: 24 hours
  - passwords hashed with bcrypt
```

### regular checks

regularly check task list before and during work:
```
list_tasks project_id=<project-id> status=inprogress
```

understand what other Claude Code instances are working on.

### task blocking

when a task needs to wait for another task to complete, note this in the task description or manage dependencies.

## Troubleshooting

### cannot find tasks

```
list_projects
```

verify you are using the correct `project_id`.

### cannot update task

- verify `task_id` is correct
- verify required parameters are provided
- verify status value is correct (todo, inprogress, inreview, done, cancelled)

### resolving conflicts

when multiple Claude Code instances select the same task:
1. the one that set `inprogress` first takes priority
2. the other selects a different task
3. consult PdM if necessary
