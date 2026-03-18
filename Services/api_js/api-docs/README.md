# API Documentation

This directory contains OpenAPI 3.0 documentation for the Personal Finance API.

## Files

- `openapi.yaml` - OpenAPI specification in YAML format

## Viewing the Documentation

### Option 1: Swagger UI (Online)

Visit [Swagger Editor](https://editor.swagger.io/) and paste the contents of `openapi.yaml`.

### Option 2: Swagger UI (Local)

You can serve the documentation locally using various tools:

#### Using npx (recommended)

```bash
npx @redocly/cli preview-docs api-docs/openapi.yaml
```

Or install Redocly CLI globally:

```bash
npm install -g @redocly/cli
redocly preview-docs api-docs/openapi.yaml
```

#### Using Docker

```bash
docker run -p 8080:8080 -e SWAGGER_JSON=/docs/openapi.yaml -v $(pwd)/api-docs:/docs swaggerapi/swagger-ui
```

Then open http://localhost:8080

### Option 3: ReDoc

```bash
npx @redocly/cli build-docs api-docs/openapi.yaml -o api-docs/index.html
```

Then open `api-docs/index.html` in your browser.

### Option 4: VS Code Extension

Install the "OpenAPI (Swagger) Editor" extension in VS Code and open `openapi.yaml`.

## API Overview

### Base URL

- Development: `http://localhost:8001`
- API endpoints: `http://localhost:8001/api`

### Authentication

All API endpoints (except `/health`) require authentication using an API key:

```bash
curl -H "X-API-Key: your-api-key" http://localhost:8001/api/transactions
```

### Available Endpoints

#### Transactions
- `GET /api/transactions` - List transactions with filters
- `GET /api/transactions/:id` - Get single transaction
- `POST /api/transactions` - Create transaction
- `PATCH /api/transactions/:id` - Update transaction

#### Categories
- `GET /api/categories` - List all categories

#### Accounts
- `GET /api/accounts` - List all accounts
- `GET /api/accounts/:id` - Get single account

#### Budgets
- `GET /api/budgets` - List all budgets

#### Alerts
- `GET /api/alerts` - List all alerts

#### Receipts
- `GET /api/receipts` - List all receipts
- `GET /api/receipts/:id` - Get single receipt
- `GET /api/receipts/:id/pdf` - Download receipt PDF

#### Reports
- `GET /api/reports/by_category` - Get spending by category
- `GET /api/reports/indicators` - Get KPI indicators

## Example Requests

### Get Transactions with Filters

```bash
curl -H "X-API-Key: your-api-key" \
  "http://localhost:8001/api/transactions?date_from=2024-01-01&date_to=2024-01-31&category=GROCERIES"
```

### Create Transaction

```bash
curl -X POST \
  -H "X-API-Key: your-api-key" \
  -H "Content-Type: application/json" \
  -d '{
    "amount_cents": -5499,
    "account_id": 1,
    "category": "GROCERIES",
    "description": "Weekly shopping",
    "creditor_name": "SuperMarket Inc",
    "booking_date": "2024-01-15"
  }' \
  http://localhost:8001/api/transactions
```

### Get Report by Category

```bash
curl -H "X-API-Key: your-api-key" \
  "http://localhost:8001/api/reports/by_category?date_from=2024-01-01&date_to=2024-01-31"
```

## Validation

To validate the OpenAPI specification:

```bash
npx @redocly/cli lint api-docs/openapi.yaml
```

## Notes

- All date parameters should be in `YYYY-MM-DD` format
- All monetary amounts are in cents (e.g., 5499 = 54.99 EUR)
- Negative amounts represent debits/expenses
- Positive amounts represent credits/income
