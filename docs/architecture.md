# Architecture

this document describes the overall architecture of the typescript-aws-dashboard project.

## System Overview

this is a **multi-tenant application** where multiple organizations can use the same system with isolated data.

```
┌─────────────────────────────────────────────────────┐
│                    Frontend                         │
│          (TypeScript / TanStack Start)              │
│                Port: 17232                          │
│         (includes tenant identification)            │
└────────────────┬────────────────────────────────────┘
                 │ GraphQL Queries/Mutations
                 │ (with tenant context)
                 │
┌────────────────▼────────────────────────────────────┐
│                    Backend                          │
│              (Rust / async-graphql)                 │
│                Port: 17231                          │
│          GraphiQL: /admin/graphiql                  │
│         (tenant-aware data access)                  │
└────────────────┬────────────────────────────────────┘
                 │ SQL Queries (with tenant_id filter)
                 │
┌────────────────▼────────────────────────────────────┐
│                   PostgreSQL                        │
│                Port: 5432                           │
│               Schema: dashboard                     │
│         (row-level tenant isolation)                │
└─────────────────────────────────────────────────────┘
```

## Multi-Tenancy Design

### tenant isolation strategy

**row-level isolation**: all tenants share the same database and tables, with data isolated by `tenant_id` column.

benefits:
- simple to implement and maintain
- cost-effective (single database)
- easy to add new tenants
- efficient resource usage

trade-offs:
- requires careful implementation to prevent data leaks
- all tenants share same database performance characteristics

### tenant identification

**authentication token-based**: tenant context is included in JWT token after authentication.

flow:
1. user logs in with credentials
2. backend validates credentials and determines tenant
3. JWT token includes `tenant_id` claim
4. all subsequent requests use this token
5. backend extracts `tenant_id` from token and filters data

### data model pattern

all tenant-specific tables include `tenant_id`:

```sql
create table dashboard.users (
  id uuid primary key,
  tenant_id uuid not null,
  email text not null,
  name text not null,
  created_at timestamptz not null
);

create index idx_users_tenant on dashboard.users(tenant_id);
```

### security considerations

- **query filtering**: all database queries must filter by `tenant_id`
- **middleware enforcement**: use actix-web middleware to automatically inject tenant context
- **no cross-tenant access**: users cannot access data from other tenants
- **admin operations**: separate admin role for cross-tenant management

## Backend

### tech stack

- **language**: Rust
- **GraphQL**: async-graphql
- **HTTP server**: actix-web
- **database**: PostgreSQL 17
- **build tool**: Cargo

### directory structure

```
backend/
├── Cargo.toml
└── src/
    └── (Rust source code)
```

### responsibilities

- provide GraphQL API endpoints
- implement business logic
- data persistence with PostgreSQL
- authentication and authorization with JWT
- tenant context management with actix-web middleware
- ensure data isolation between tenants

### GraphQL API

GraphiQL IDE is accessible at http://localhost:17231/admin/graphiql

see [api.md](./api.md) for schema details.
see [multi-tenancy.md](./multi-tenancy.md) for tenant implementation details.

## Database

### PostgreSQL

- **version**: PostgreSQL 17
- **host**: localhost
- **port**: 5432
- **user**: dashboard
- **password**: dashboard
- **database**: dashboard
- **schema**: dashboard

the database runs in a Docker container managed by docker compose.

## Frontend

### tech stack

- **language**: TypeScript
- **framework**: TanStack Start (with TanStack Router)
- **styling**: Tailwind CSS v4
- **UI components**: React Aria Components
- **state management**: React Stately
- **GraphQL client**: Apollo Client
- **package manager**: pnpm
- **Node.js**: v22

### directory structure

```
frontend/
├── package.json
├── vite.config.ts
├── tsconfig.json
├── tailwind.config.ts
├── postcss.config.js
├── public/
└── src/
    ├── router.tsx
    ├── routes/
    ├── components/
    ├── lib/
    │   └── apollo.ts
    └── styles.css
```

### responsibilities

- implement UI components with React Aria Components
- implement GraphQL client (Apollo Client) with authentication
- routing with TanStack Router
- state management with React Stately including tenant context
- SSR (Server-Side Rendering) with TanStack Start
- handle authentication flow
- store and manage JWT tokens

## Development Environment

### Nix

the project uses Nix flakes to build the development environment.

```bash
nix develop
```

this provides an environment with all necessary dependencies.

### build and test

all operations are defined in the Makefile. see [development.md](./development.md) for details.

## Deployment

### build

```bash
make build-release
```

builds release binaries and frontend assets.

### artifacts

- backend: executable in `backend/target/release/`
- frontend: static assets in `frontend/.output/`

## Security Considerations

- [ ] authentication and authorization with JWT
- [ ] tenant context validation
- [ ] CORS configuration
- [ ] GraphQL query depth limits
- [ ] rate limiting per tenant
- [ ] input validation
- [ ] SQL injection prevention
- [ ] cross-tenant access prevention

## Performance Considerations

- [ ] GraphQL N+1 problem mitigation (DataLoader, etc.)
- [ ] caching strategy per tenant
- [ ] SSR performance optimization
- [ ] backend asynchronous processing
- [ ] database connection pooling
- [ ] tenant_id index optimization

## Monitoring and Logging

- [ ] log collection
- [ ] metrics collection per tenant
- [ ] error tracking
- [ ] performance monitoring
- [ ] tenant usage tracking

## Multi-Tenancy Implementation Checklist

- [ ] database schema with tenant_id columns
- [ ] database migrations system
- [ ] authentication system with JWT
- [ ] tenant context middleware for actix-web
- [ ] GraphQL context with tenant information
- [ ] row-level security enforcement
- [ ] tenant registration/management
- [ ] admin panel for cross-tenant operations

## Future Enhancements

- [ ] AWS integration (inferred from project name)
- [ ] CI/CD pipeline
- [ ] tenant-specific customization
- [ ] usage metrics per tenant
- [ ] tenant usage limits and quotas
- [ ] database backup strategy per tenant
