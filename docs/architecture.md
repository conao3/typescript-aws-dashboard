# Architecture

this document describes the overall architecture of the typescript-aws-dashboard project.

## System Overview

```
┌─────────────────────────────────────────────────────┐
│                    Frontend                         │
│          (TypeScript / TanStack Start)              │
│                Port: 17232                          │
└────────────────┬────────────────────────────────────┘
                 │ GraphQL Queries/Mutations
                 │
┌────────────────▼────────────────────────────────────┐
│                    Backend                          │
│              (Rust / async-graphql)                 │
│                Port: 17231                          │
│          GraphiQL: /admin/graphiql                  │
└─────────────────────────────────────────────────────┘
```

## Backend

### tech stack

- **language**: Rust
- **GraphQL**: async-graphql
- **HTTP server**: (to be confirmed)
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
- data persistence (database connection, depending on implementation)
- authentication and authorization (depending on implementation)

### GraphQL API

GraphiQL IDE is accessible at http://localhost:17231/admin/graphiql

see [api.md](./api.md) for schema details.

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

- implement UI components
- implement GraphQL client
- routing
- state management
- SSR (Server-Side Rendering)

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

- [ ] authentication and authorization implementation status
- [ ] CORS configuration
- [ ] GraphQL query depth limits
- [ ] rate limiting
- [ ] input validation

## Performance Considerations

- [ ] GraphQL N+1 problem mitigation (DataLoader, etc.)
- [ ] caching strategy
- [ ] SSR performance optimization
- [ ] backend asynchronous processing

## Monitoring and Logging

- [ ] log collection
- [ ] metrics collection
- [ ] error tracking
- [ ] performance monitoring

## Future Enhancements

- [ ] database integration
- [ ] authentication system
- [ ] AWS integration (inferred from project name)
- [ ] CI/CD pipeline
- [ ] containerization (Docker)
