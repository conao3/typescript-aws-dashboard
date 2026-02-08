# Multi-Tenancy Implementation Guide

this document provides detailed guidelines for implementing multi-tenancy in this project.

## Overview

this application uses **row-level isolation** for multi-tenancy:

- single database for all tenants
- data isolated by `tenant_id` column
- tenant identification via JWT token

## Database Design

### schema management with sqldef

this project uses [sqldef](https://github.com/sqldef/sqldef) for database schema management.

sqldef is a declarative schema management tool:

- define desired schema in `schema.sql`
- sqldef calculates diff and applies changes
- idempotent and safe migrations
- use `psqldef` command for PostgreSQL

### tenants table

```sql
create table dashboard.tenants (
  id uuid primary key default gen_random_uuid(),
  name text not null,
  slug text not null unique,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  is_active boolean not null default true
);
```

### tenant-specific tables

all tables that contain tenant-specific data must:

1. include `tenant_id uuid not null` column
2. have foreign key to `tenants(id)`
3. have index on `tenant_id`

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

## Authentication Flow

### login

1. user submits email and password
2. backend finds user by email (considering tenant)
3. backend validates password
4. backend generates JWT with claims:
   ```json
   {
     "sub": "user_id",
     "tenant_id": "tenant_id",
     "email": "user@example.com",
     "exp": 1234567890
   }
   ```
5. frontend stores JWT token

### authenticated requests

1. frontend includes JWT token in Authorization header
2. backend middleware validates JWT
3. backend extracts `tenant_id` from token
4. backend injects tenant context into request
5. all database queries automatically filter by `tenant_id`

## Backend Implementation

### tenant context

```rust
// example structure
pub struct TenantContext {
    pub tenant_id: Uuid,
    pub user_id: Uuid,
}
```

### middleware

create middleware to:

1. extract JWT from Authorization header
2. validate JWT signature
3. extract `tenant_id` and `user_id` from claims
4. inject `TenantContext` into request

### GraphQL context

```rust
// example
pub struct GraphQLContext {
    pub tenant: TenantContext,
    pub db: DatabasePool,
}
```

### query filtering

always filter by tenant_id:

```rust
// example
sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE tenant_id = $1 AND id = $2",
    context.tenant.tenant_id,
    user_id
)
```

### repository pattern

create repository layer that automatically injects tenant context:

```rust
// example
impl UserRepository {
    pub async fn find_by_id(
        &self,
        ctx: &TenantContext,
        id: Uuid,
    ) -> Result<User> {
        // automatically filters by ctx.tenant_id
    }
}
```

## Frontend Implementation

### token storage

store JWT token in:

- **httpOnly cookie** (recommended for security)
- or localStorage (simpler but less secure)

### GraphQL client setup

```typescript
// example using urql
const client = createClient({
  url: "http://localhost:17231/graphql",
  fetchOptions: () => {
    const token = getToken();
    return {
      headers: {
        authorization: token ? `Bearer ${token}` : "",
      },
    };
  },
});
```

### authentication state

manage authentication state:

- current user information
- current tenant information
- token validity

## Security Best Practices

### critical rules

1. **never trust client-provided tenant_id**: always use tenant_id from JWT
2. **always filter by tenant_id**: every query must include tenant filter
3. **validate tenant access**: ensure user belongs to tenant before issuing JWT
4. **use prepared statements**: prevent SQL injection
5. **audit logging**: log all tenant-specific operations

### middleware enforcement

create a middleware that:

- runs before all GraphQL resolvers
- automatically injects tenant context
- prevents queries without tenant context

### testing

write tests to verify:

- users cannot access other tenants' data
- tenant isolation is maintained
- authentication is required for all operations

## Admin Operations

### super admin

create separate admin role for cross-tenant operations:

- tenant management (create, update, delete)
- user management across tenants
- system monitoring

admin endpoints should:

- use separate authentication
- not require tenant_id in JWT
- explicitly handle tenant selection

## Database Migrations with sqldef

### schema file location

```
backend/schema.sql
```

this file contains the complete desired schema state.

### applying migrations

```bash
# dry run (show what would be changed)
psqldef -U dashboard -h localhost dashboard --file=backend/schema.sql --dry-run

# apply changes
psqldef -U dashboard -h localhost dashboard --file=backend/schema.sql
```

### schema guidelines

1. always add `tenant_id` to new tables
2. create indexes on `tenant_id`
3. add foreign key constraints
4. use `dashboard` schema for all tables
5. include constraints and defaults

### example schema.sql structure

```sql
-- create schema
create schema if not exists dashboard;

-- tenants table
create table dashboard.tenants (
  id uuid primary key default gen_random_uuid(),
  name text not null,
  slug text not null unique,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  is_active boolean not null default true
);

-- users table with tenant isolation
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

### workflow

1. edit `backend/schema.sql` with desired schema changes
2. run `psqldef` with `--dry-run` to preview changes
3. review the diff carefully
4. apply changes by running `psqldef` without `--dry-run`
5. test the changes

### test data

for development, create a separate `backend/seed.sql` file:

```sql
-- test tenants
insert into dashboard.tenants (id, name, slug) values
  ('00000000-0000-0000-0000-000000000001', 'Test Tenant 1', 'test1'),
  ('00000000-0000-0000-0000-000000000002', 'Test Tenant 2', 'test2')
on conflict do nothing;
```

apply seed data:

```bash
psql -U dashboard -h localhost dashboard < backend/seed.sql
```

## Error Handling

### tenant-related errors

- **invalid tenant**: return 401 Unauthorized
- **tenant not found**: return 404 Not Found
- **tenant inactive**: return 403 Forbidden
- **cross-tenant access attempt**: return 403 Forbidden and log incident

## Monitoring

### metrics to track

- queries per tenant
- storage usage per tenant
- API requests per tenant
- error rates per tenant
- slow queries per tenant

## Testing Strategy

### unit tests

test individual components with mock tenant context.

### integration tests

test full flow with multiple tenants:

1. create test tenants
2. create test users for each tenant
3. verify data isolation
4. verify cross-tenant access prevention

### test data

create test data in migrations:

```sql
-- test tenants
insert into tenants (id, name, slug) values
  ('00000000-0000-0000-0000-000000000001', 'Test Tenant 1', 'test1'),
  ('00000000-0000-0000-0000-000000000002', 'Test Tenant 2', 'test2');
```

## Performance Considerations

### indexing

ensure all tenant_id columns are indexed:

```sql
create index idx_table_tenant on table_name(tenant_id);
```

### query optimization

- use compound indexes: `(tenant_id, other_column)`
- avoid full table scans
- use connection pooling
- consider partitioning for large datasets

## Backup and Recovery

### backup strategy

- full database backups
- option to restore individual tenant data
- tenant-specific data export

## Future Enhancements

- [ ] tenant-specific customization (themes, logos)
- [ ] tenant-specific feature flags
- [ ] tenant usage limits and quotas
- [ ] multi-database support (one DB per tenant)
- [ ] tenant data export and import
- [ ] tenant-specific backups
