# typescript-aws-dashboard

Rust GraphQL backend and TypeScript frontend project.

## Setup

```bash
nix develop
make setup
```

## Development

Start PostgreSQL database:

```bash
make db-up
```

Start backend server:

```bash
make dev-backend
```

GraphiQL IDE: http://localhost:17231/admin/graphiql

Start frontend development server:

```bash
make dev
```

Frontend: http://localhost:17232

### Database Commands

```bash
make db-up      # start postgres
make db-down    # stop postgres
make db-logs    # show logs
make db-reset   # reset database
```

Database connection:
- Host: localhost
- Port: 5432
- User: dashboard
- Password: dashboard
- Database: dashboard
- Schema: dashboard

## Build

```bash
make build
```

## Test

```bash
make test
```

## Format

```bash
make fmt
```

## Lint

```bash
make lint
```

## Documentation

For detailed documentation, see the [docs](./docs) directory:

- [Claude Code Coordination Guide](./docs/coordination.md) - guidelines for multiple Claude Code instances
- [Architecture](./docs/architecture.md) - system architecture
- [Development Guide](./docs/development.md) - development environment and workflow
- [API Specification](./docs/api.md) - GraphQL API specification
