# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
npm run dev          # Start with hot-reloading (ts-node-dev, port 8001)
npm run build        # Compile TypeScript to dist/
npm run build:prod   # Bundle with esbuild to dist_prod/index.js (minified)
npm start            # Run compiled dist/index.js
npm run start:prod   # Run production bundle
./deploy.sh <dev|pro>  # Build and deploy to /k/pfinance/{dev|pro}/api/api.js
```

There are no test scripts defined in this project.

## Architecture

This is an Express/TypeScript Personal Finance API running on port 8001. It uses raw SQL queries (no ORM) against PostgreSQL, with two database schemas:
- `application.*` — enriched views used for reads
- `raw.*` — raw tables used for writes (INSERT/UPDATE)

### Module Structure

Each domain feature lives in `src/modules/<feature>/` with consistent layers:

```
*.model.ts        — TypeScript interfaces matching DB rows
*.dto.ts          — Response serialization (snake_case for v1)
*.service.ts      — Business logic and raw SQL queries
*.controller.ts   — HTTP handlers, DTO transformations
*.routes.ts       — Express route definitions
v2/               — V2 variants (camelCase responses)
  *.controller.v2.ts
  *.dto.v2.ts
  *.routes.v2.ts
```

### API Versioning

Two API versions exist with **identical business logic** but different response field casing:
- **V1** (`/api/v1/*` and `/api/*` legacy): snake_case fields
- **V2** (`/api/v2/*`): camelCase fields

Routes are mounted in `src/routes/v1.routes.ts` and `src/routes/v2.routes.ts`.

### Authentication

All routes except `GET /health` require an `x-api-key` header validated by `src/middleware/apiKeyAuth.ts` against the `API_KEY` environment variable.

### Database

Connection pool configured in `src/config/database.ts` (max 20 connections). All queries use parameterized statements with `$1, $2` placeholders. Date filters default to current year (Jan 1 – Dec 31) via `src/utils/dateHelpers.ts`.

### External Services

AWS S3 integration (`src/services/awsS3Client.ts`) is used only for downloading receipt PDFs (`GET /api/receipts/:id/pdf`).

### Error Handling

Custom `AppError` class in `src/middleware/errorHandler.ts`. Services throw `AppError` directly; the global error handler middleware formats responses. Database constraint violations → 409, validation → 400, not found → 404.

## Environment Variables

See `.env.example` for all required variables:
- `DATABASE_URL` — PostgreSQL connection string
- `API_KEY` — Secret for `x-api-key` authentication
- `AWS_REGION`, `AWS_S3_BUCKET`, `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` — S3 access
- `PORT` — defaults to 8001
- `LOG_LEVEL` — defaults to `info`
- `ENABLE_FILE_LOGGING` — set to `true` to write to `logs/` in development
