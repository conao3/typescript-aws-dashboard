# Current Tasks

this document tracks current tasks for the typescript-aws-dashboard project.

## Task Status Legend

- 🔴 **TODO**: not started
- 🟡 **IN PROGRESS**: currently being worked on
- 🟢 **DONE**: completed
- ⚪ **BLOCKED**: waiting for dependencies

## Backend Tasks

### 🔴 setup initial GraphQL schema

- **assigned to**: backend lead
- **priority**: high
- **description**: define basic GraphQL schema with tenant-aware queries and mutations
- **dependencies**: tenant context middleware
- **notes**:

### 🔴 implement AWS credentials management

- **assigned to**: backend lead
- **priority**: high
- **description**: implement CRUD operations for AWS credentials with encryption
- **dependencies**: tenant context middleware, authentication
- **notes**: encrypt access_key_id and secret_access_key before storing

### 🔴 integrate AWS SDK for EC2

- **assigned to**: backend lead
- **priority**: high
- **description**: integrate AWS SDK for Rust, implement EC2 client with tenant credentials
- **dependencies**: AWS credentials management
- **notes**: use tenant's AWS credentials to make API calls

### 🔴 implement EC2 AMI import task sync

- **assigned to**: backend lead
- **priority**: high
- **description**: implement sync functionality to fetch AMI import tasks from AWS API
- **dependencies**: AWS SDK integration
- **notes**: use DescribeImportImageTasks API

### 🔴 implement EC2 GraphQL schema

- **assigned to**: backend lead
- **priority**: high
- **description**: define GraphQL schema for EC2 AMI import tasks queries and mutations
- **dependencies**: EC2 AMI import task sync
- **notes**: include filtering and pagination

### 🔴 implement health check endpoint

- **assigned to**: backend lead
- **priority**: low
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

### 🔴 implement AWS credentials UI

- **assigned to**: frontend lead
- **priority**: high
- **description**: create UI for managing AWS credentials (add, edit, delete)
- **dependencies**: authentication UI
- **notes**: never display secret keys after initial creation

### 🔴 implement EC2 AMI import list view

- **assigned to**: frontend lead
- **priority**: high
- **description**: create list view for EC2 AMI import tasks with filtering and search
- **dependencies**: Apollo Client setup
- **notes**: use React Aria Components Table

### 🔴 implement EC2 AMI import detail view

- **assigned to**: frontend lead
- **priority**: high
- **description**: create detail view showing full import task information
- **dependencies**: EC2 AMI import list view
- **notes**: display all AWS API response fields

### 🔴 implement tenant context display

- **assigned to**: frontend lead
- **priority**: low
- **description**: show current tenant information in UI
- **dependencies**: Apollo Client setup
- **notes**:

## DevOps Tasks

### 🟡 setup CI/CD pipeline

- **assigned to**: devops lead
- **priority**: medium
- **description**: configure GitHub Actions for testing and building
- **dependencies**: none
- **notes**: started 2026-02-08

### 🔴 add Docker configuration

- **assigned to**: devops lead
- **priority**: low
- **description**: create Dockerfile and docker-compose.yml for deployment
- **dependencies**: none
- **notes**:

## Completed Tasks

### 🟢 implement tenant context middleware (2026-02-08)

- **assigned to**: backend lead
- **priority**: high
- **description**: create middleware to extract tenant_id from JWT and inject into request context
- **implementation**:
  - created `backend/src/middleware.rs` with TenantContext struct
  - implemented extract_tenant_context to parse Authorization header and verify JWT
  - integrated middleware into graphql handler in `backend/src/main.rs`
  - added TenantContext injection into GraphQL execution context
  - created currentUser query to demonstrate tenant-aware GraphQL queries
  - added async-graphql uuid and chrono features to support User model in GraphQL
  - User model marked as SimpleObject with password_hash field excluded from GraphQL schema

### 🟢 setup PostgreSQL database connection (2026-02-08)

- **assigned to**: backend lead
- **priority**: high
- **description**: configure PostgreSQL connection pool and basic database setup
- **implementation**:
  - added sqlx 0.8 with PostgreSQL, UUID, and chrono support
  - created `backend/src/db.rs` with connection pool initialization
  - integrated pool into actix-web application state
  - added `/health` endpoint for database connectivity check
  - pool configuration: max 5 connections, DATABASE_URL environment variable support

### 🟢 setup sqldef for schema management (2026-02-08)

- **assigned to**: backend lead
- **priority**: high
- **description**: install psqldef and create initial schema.sql file structure
- **implementation**:
  - added sqldef and postgresql to flake.nix development environment
  - created `backend/schema.sql` with dashboard schema and tenants table
  - added `make db-migrate-dry` for schema change preview
  - added `make db-migrate` for schema application
  - successfully applied initial schema with tenants table, indexes, and constraints

### 🟢 create initial schema with multi-tenancy (2026-02-08)

- **assigned to**: backend lead
- **priority**: high
- **description**: create backend/schema.sql with tenants table and initial schema with tenant_id columns
- **implementation**:
  - created `dashboard.users` table with tenant_id, authentication fields
  - created `dashboard.aws_credentials` table with encrypted credential storage
  - created `dashboard.ec2_ami_import_tasks` table for AMI import tracking
  - all tables include tenant_id with foreign key to tenants(id) on delete cascade
  - added tenant_id indexes on all tenant-specific tables
  - added unique constraints: users(tenant_id, email), ec2_ami_import_tasks(tenant_id, import_task_id)

### 🟢 implement JWT authentication (2026-02-08)

- **assigned to**: backend lead
- **priority**: high
- **description**: implement JWT token generation, validation, and tenant_id extraction
- **implementation**:
  - added jsonwebtoken 9 and bcrypt 0.16 dependencies
  - created `backend/src/auth.rs` with JwtConfig, Claims, and password hashing utilities
  - created `backend/src/models.rs` with User and Tenant database models
  - implemented `login` GraphQL mutation with email/password authentication
  - JWT includes user_id (sub), tenant_id, email, and expiration (24 hours)
  - created `backend/seed.sql` with test tenants and users (password: "password")
  - login mutation validates credentials and returns JWT token

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
