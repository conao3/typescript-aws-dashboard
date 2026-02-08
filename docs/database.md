# Database Guide

this document describes database setup and management for the project.

## Database Overview

- **database**: PostgreSQL 17
- **host**: localhost (in Docker container)
- **port**: 5432
- **user**: dashboard
- **password**: dashboard
- **database**: dashboard
- **schema**: dashboard

## Schema Management with sqldef

this project uses [sqldef](https://github.com/sqldef/sqldef) for declarative schema management.

### what is sqldef?

sqldef is a tool that manages database schemas declaratively:

- define the desired schema state in a single SQL file
- sqldef calculates the diff between current and desired state
- automatically generates and applies migration SQL
- idempotent and safe

### installation

```bash
# on macOS
brew install sqldef

# on Linux
# download from https://github.com/sqldef/sqldef/releases
```

or use Nix (already included in flake.nix):

```bash
nix develop
```

### schema file

the complete schema is defined in:

```
backend/schema.sql
```

### applying schema changes

#### dry run (preview changes)

```bash
psqldef -U dashboard -h localhost dashboard --file=backend/schema.sql --dry-run
```

this shows what SQL will be executed without actually applying changes.

#### apply changes

```bash
psqldef -U dashboard -h localhost dashboard --file=backend/schema.sql
```

this applies the changes to make the database match schema.sql.

### workflow

1. **edit schema.sql**: add or modify tables, columns, indexes
2. **dry run**: `psqldef ... --dry-run` to preview changes
3. **review diff**: carefully review the generated SQL
4. **apply**: `psqldef ...` to apply changes
5. **test**: verify the changes work as expected

### schema.sql structure

```sql
-- create schema
create schema if not exists dashboard;

-- tables
create table dashboard.tenants (
  id uuid primary key default gen_random_uuid(),
  name text not null,
  slug text not null unique,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  is_active boolean not null default true
);

-- indexes
create index idx_tenants_slug on dashboard.tenants(slug);
```

## Multi-Tenancy Schema

all tenant-specific tables must include:

- `tenant_id uuid not null` column
- foreign key to `dashboard.tenants(id)`
- index on `tenant_id`

example:

```sql
create table dashboard.users (
  id uuid primary key default gen_random_uuid(),
  tenant_id uuid not null references dashboard.tenants(id) on delete cascade,
  email text not null,
  name text not null,
  password_hash text not null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create index idx_users_tenant on dashboard.users(tenant_id);
create unique index idx_users_tenant_email on dashboard.users(tenant_id, email);
```

see [multi-tenancy.md](./multi-tenancy.md) for detailed multi-tenancy design.

## Seed Data

for development and testing, create:

```
backend/seed.sql
```

example:

```sql
-- test tenants
insert into dashboard.tenants (id, name, slug) values
  ('00000000-0000-0000-0000-000000000001', 'Test Tenant 1', 'test1'),
  ('00000000-0000-0000-0000-000000000002', 'Test Tenant 2', 'test2')
on conflict do nothing;

-- test users
insert into dashboard.users (id, tenant_id, email, name, password_hash) values
  (
    '10000000-0000-0000-0000-000000000001',
    '00000000-0000-0000-0000-000000000001',
    'user1@test1.example.com',
    'User 1',
    '$2b$12$...' -- bcrypt hash
  ),
  (
    '10000000-0000-0000-0000-000000000002',
    '00000000-0000-0000-0000-000000000002',
    'user1@test2.example.com',
    'User 1',
    '$2b$12$...' -- bcrypt hash
  )
on conflict do nothing;
```

apply seed data:

```bash
psql -U dashboard -h localhost dashboard < backend/seed.sql
```

## Common Operations

### connect to database

```bash
psql -U dashboard -h localhost dashboard
```

### list tables

```sql
\dt dashboard.*
```

### describe table

```sql
\d dashboard.users
```

### view schema

```sql
\dn+
```

### reset database (development only)

```bash
# drop and recreate database
psql -U dashboard -h localhost -c "drop database if exists dashboard;"
psql -U dashboard -h localhost -c "create database dashboard;"

# apply schema
psqldef -U dashboard -h localhost dashboard --file=backend/schema.sql

# apply seed data
psql -U dashboard -h localhost dashboard < backend/seed.sql
```

## Best Practices

### schema changes

1. **always use sqldef**: do not manually run ALTER TABLE or CREATE TABLE
2. **review diffs**: always run with `--dry-run` first
3. **test locally**: test schema changes in development first
4. **backup production**: backup before applying to production

### naming conventions

- tables: lowercase, plural (e.g., `users`, `api_keys`)
- columns: lowercase with underscore (e.g., `created_at`, `tenant_id`)
- indexes: `idx_{table}_{columns}` (e.g., `idx_users_tenant`)
- foreign keys: `fk_{table}_{column}` (e.g., `fk_users_tenant`)

### indexes

create indexes for:

- foreign keys (especially `tenant_id`)
- columns used in WHERE clauses
- columns used in JOIN conditions
- unique constraints

### data types

prefer:

- `uuid` for IDs
- `text` for strings (not varchar)
- `timestamptz` for timestamps (not timestamp)
- `boolean` for flags
- `jsonb` for structured data

## Troubleshooting

### sqldef shows unexpected changes

check:

1. schema.sql syntax is correct
2. database connection is to the correct database
3. no manual changes were made to the database

### connection refused

check:

1. PostgreSQL container is running
2. port 5432 is accessible
3. credentials are correct

### permission denied

check:

1. user has correct permissions
2. schema ownership is correct

## Makefile Integration

add these targets to Makefile:

```makefile
.PHONY: db-migrate-dry
db-migrate-dry: ## preview database schema changes
	psqldef -U dashboard -h localhost dashboard --file=backend/schema.sql --dry-run

.PHONY: db-migrate
db-migrate: ## apply database schema changes
	psqldef -U dashboard -h localhost dashboard --file=backend/schema.sql

.PHONY: db-seed
db-seed: ## apply seed data
	psql -U dashboard -h localhost dashboard < backend/seed.sql

.PHONY: db-reset
db-reset: ## reset database (development only)
	psql -U dashboard -h localhost -c "drop database if exists dashboard;"
	psql -U dashboard -h localhost -c "create database dashboard;"
	$(MAKE) db-migrate
	$(MAKE) db-seed
```

usage:

```bash
make db-migrate-dry  # preview changes
make db-migrate      # apply changes
make db-seed         # load test data
make db-reset        # reset database (dev only)
```
