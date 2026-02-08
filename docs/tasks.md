# Current Tasks

this document tracks current tasks for the typescript-aws-dashboard project.

## Task Status Legend

- 🔴 **TODO**: not started
- 🟡 **IN PROGRESS**: currently being worked on
- 🟢 **DONE**: completed
- ⚪ **BLOCKED**: waiting for dependencies

## Backend Tasks

### 🔴 setup PostgreSQL database connection

- **assigned to**: backend lead
- **priority**: high
- **description**: configure PostgreSQL connection pool and basic database setup
- **dependencies**: none
- **notes**:

### 🔴 setup sqldef for schema management

- **assigned to**: backend lead
- **priority**: high
- **description**: install psqldef and create initial schema.sql file structure
- **dependencies**: database connection
- **notes**: use sqldef for declarative schema management

### 🔴 create initial schema with multi-tenancy

- **assigned to**: backend lead
- **priority**: high
- **description**: create backend/schema.sql with tenants table and initial schema with tenant_id columns
- **dependencies**: sqldef setup
- **notes**: define complete schema in schema.sql, apply with psqldef

### 🔴 implement JWT authentication

- **assigned to**: backend lead
- **priority**: high
- **description**: implement JWT token generation, validation, and tenant_id extraction
- **dependencies**: none
- **notes**:

### 🔴 implement tenant context middleware

- **assigned to**: backend lead
- **priority**: high
- **description**: create middleware to extract tenant_id from JWT and inject into request context
- **dependencies**: JWT authentication
- **notes**:

### 🔴 setup initial GraphQL schema

- **assigned to**: backend lead
- **priority**: high
- **description**: define basic GraphQL schema with tenant-aware queries and mutations
- **dependencies**: tenant context middleware
- **notes**:

### 🔴 implement health check endpoint

- **assigned to**: backend lead
- **priority**: medium
- **description**: add health check endpoint for monitoring
- **dependencies**: none
- **notes**:

## Frontend Tasks

### 🔴 implement authentication UI

- **assigned to**: frontend lead
- **priority**: high
- **description**: create login/logout pages and authentication flow with React Aria Components
- **dependencies**: backend JWT authentication
- **notes**:

### 🔴 implement token management

- **assigned to**: frontend lead
- **priority**: high
- **description**: store and manage JWT tokens in cookies or localStorage
- **dependencies**: authentication UI
- **notes**:

### 🔴 setup Apollo Client with authentication

- **assigned to**: frontend lead
- **priority**: high
- **description**: configure Apollo Client to include JWT token in requests
- **dependencies**: token management
- **notes**:

### 🔴 create basic layout components

- **assigned to**: frontend lead
- **priority**: medium
- **description**: implement header, sidebar, and main layout components with React Aria Components
- **dependencies**: none
- **notes**:

### 🔴 implement tenant context display

- **assigned to**: frontend lead
- **priority**: low
- **description**: show current tenant information in UI
- **dependencies**: Apollo Client setup
- **notes**:

## DevOps Tasks

### 🔴 setup CI/CD pipeline

- **assigned to**: devops lead
- **priority**: medium
- **description**: configure GitHub Actions for testing and building
- **dependencies**: none
- **notes**:

### 🔴 add Docker configuration

- **assigned to**: devops lead
- **priority**: low
- **description**: create Dockerfile and docker-compose.yml for deployment
- **dependencies**: none
- **notes**:

## Completed Tasks

tasks marked as 🟢 DONE will be moved here with completion date.

---

## How to Use This File

### adding a new task

copy this template:

```markdown
### 🔴 task title

- **assigned to**: role (backend lead / frontend lead / devops lead)
- **priority**: high / medium / low
- **description**: detailed description
- **dependencies**: list any blocking tasks or "none"
- **notes**: additional information
```

### updating task status

change the emoji at the start of the task:

- starting work: 🔴 → 🟡
- completing work: 🟡 → 🟢
- blocked: any → ⚪

### completing a task

when a task is done:

1. change status to 🟢
2. add completion date to notes
3. move to "Completed Tasks" section

### communication

- announce when starting a task (🔴 → 🟡)
- update notes with progress or blockers
- notify PdM when completing a task (🟡 → 🟢)
