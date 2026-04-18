# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
npm run dev          # Start dev server (Vite, hot reload)
npm run build        # Production build
npm run type-check   # TypeScript type checking (vue-tsc)
npm run lint         # ESLint with auto-fix
npm run format       # Prettier formatting for src/
```

There are no automated tests in this project.

## Architecture

This is a Vue 3 + TypeScript personal finance dashboard. It connects to a backend API running at `localhost:8001` (local) or `pfinance-api.kumo.home` (production K8).

### Data Flow

```
Views → Composables (TanStack Vue Query) → services/api/ (Axios) → Backend REST API
```

- **`src/views/`** — Page-level route components (Home, Transactions, Budgets, Categories, Accounts, Indicators)
- **`src/components/`** — Reusable UI components scoped to domain (account, budget, category, transaction, alert)
- **`src/composables/`** — All data fetching and mutation logic lives here using TanStack Vue Query (`useQuery`, `useMutation`). Each composable owns one domain (e.g., `useTransactions`, `useBudgets`).
- **`src/services/api/`** — Thin Axios wrappers per domain. The shared Axios instance in `src/config/index.ts` injects the API key from `localStorage` on every request and handles HTTP error responses globally.

### Key Conventions

- **API key auth**: stored in `localStorage`, injected via Axios request interceptor — no login screen.
- **Year filter**: a global year filter (also in `localStorage`) is passed as a query param to most API calls; it drives what data is shown across views.
- **Privacy mode / compact mode**: UI toggles stored in `localStorage`, read via `src/config/index.ts` constants.
- **Charts**: Chart.js via `vue-chartjs` — used in dashboard and indicator views.
- **Path alias**: `@/` maps to `src/` — use it for all imports.
- **Environment**: controlled via `.env` files and `VITE_HOST` / `VITE_ENV` variables. The `build-devk8` script targets a separate `dist_devk8/` output.
