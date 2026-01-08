# Personal Finance API - Node.js

A Node.js/TypeScript implementation of the Personal Finance API that replicates the functionality of the Rust API server.

## Features

- **Full API Compatibility**: Implements all endpoints from the Rust API
- **PostgreSQL Database**: Connects to the same database using raw SQL queries
- **TypeScript**: Strong typing for better maintainability
- **Express.js**: Fast and minimal web framework
- **AWS S3 Integration**: PDF receipt downloads from S3
- **Authentication**: API key-based authentication via `x-api-key` header
- **Logging**: Winston-based logging to console and file
- **Error Handling**: Comprehensive error handling and validation

## Tech Stack

- **Runtime**: Node.js
- **Language**: TypeScript
- **Framework**: Express.js
- **Database**: PostgreSQL (via `pg` driver)
- **Cloud**: AWS S3 (via `@aws-sdk/client-s3`)
- **Logging**: Winston
- **Dev Tools**: ts-node-dev for hot-reloading

## Prerequisites

- Node.js 18+ and npm
- PostgreSQL database (same as used by Rust API)
- AWS credentials for S3 access (for receipt PDFs)

## Installation

1. **Install dependencies**:
   ```bash
   npm install
   ```

2. **Configure environment variables**:

   Copy `.env.example` to `.env` and configure:
   ```bash
   cp .env.example .env
   ```

   Edit `.env` with your settings:
   ```env
   # Database
   DATABASE_URL=postgresql://username:password@localhost:5432/personal_finance

   # API Security
   API_KEY=your-secret-api-key-here

   # AWS Configuration
   AWS_REGION=eu-central-1
   AWS_S3_BUCKET=pfinance-receipts
   AWS_ACCESS_KEY_ID=your-access-key
   AWS_SECRET_ACCESS_KEY=your-secret-key

   # Server Configuration
   PORT=8001
   NODE_ENV=development

   # Logging
   LOG_LEVEL=info
   ```

## Usage

### Development Mode

Start the server with hot-reloading:
```bash
npm run dev
```

The API will be available at `http://localhost:8001`

### Production Build

1. **Build the TypeScript code**:
   ```bash
   npm run build
   ```

2. **Start the production server**:
   ```bash
   npm start
   ```

## API Endpoints

All endpoints require the `x-api-key` header for authentication.

### Categories

- `GET /api/categories` - Get all transaction categories

### Accounts

- `GET /api/accounts` - Get all accounts
- `GET /api/accounts/:id` - Get specific account by ID

### Transactions

- `GET /api/transactions?category&account&date_from&date_to` - Get transactions with optional filters
  - `category` (optional) - Filter by category code
  - `account` (optional) - Filter by account ID
  - `date_from` (optional) - Start date (YYYY-MM-DD), defaults to Jan 1 of current year
  - `date_to` (optional) - End date (YYYY-MM-DD), defaults to Dec 31 of current year
- `GET /api/transactions/:id` - Get specific transaction
- `POST /api/transactions` - Create new transaction
- `PATCH /api/transactions/:id` - Update transaction

### Reports

- `GET /api/reports/by_category?date_from&date_to` - Get spending report grouped by category
- `GET /api/reports/indicators?date_from&date_to` - Get KPI indicators

### Budgets

- `GET /api/budgets` - Get all budgets

### Alerts

- `GET /api/alerts` - Get all current alerts

### Receipts

- `GET /api/receipts` - Get all receipts
- `GET /api/receipts/:id` - Get specific receipt
- `GET /api/receipts/:id/pdf` - Download receipt PDF from S3

### Health Check

- `GET /health` - Health check endpoint (no authentication required)

## Example Requests

### Get all transactions for current year

```bash
curl -H "x-api-key: your-api-key" http://localhost:8001/api/transactions
```

### Get transactions filtered by date range

```bash
curl -H "x-api-key: your-api-key" \
  "http://localhost:8001/api/transactions?date_from=2024-01-01&date_to=2024-12-31"
```

### Create a new transaction

```bash
curl -X POST \
  -H "x-api-key: your-api-key" \
  -H "Content-Type: application/json" \
  -d '{
    "amount_cents": 5000,
    "category": "FOOD",
    "description": "Grocery shopping",
    "account_id": 1,
    "booking_date": "2024-01-15"
  }' \
  http://localhost:8001/api/transactions
```

### Get report by category

```bash
curl -H "x-api-key: your-api-key" \
  "http://localhost:8001/api/reports/by_category?date_from=2024-01-01&date_to=2024-12-31"
```

## Project Structure

```
api_js/
├── src/
│   ├── index.ts                      # Application entry point
│   ├── config/
│   │   ├── database.ts               # PostgreSQL connection pool
│   │   └── logger.ts                 # Winston logger configuration
│   ├── middleware/
│   │   ├── apiKeyAuth.ts             # API key authentication
│   │   └── errorHandler.ts           # Global error handling
│   ├── services/
│   │   └── awsS3Client.ts            # S3 client for receipt PDFs
│   ├── utils/
│   │   └── dateHelpers.ts            # Date parsing utilities
│   └── modules/
│       ├── categories/               # Categories module
│       ├── accounts/                 # Accounts module
│       ├── transactions/             # Transactions module
│       ├── reports/                  # Reports module
│       ├── budgets/                  # Budgets module
│       ├── alerts/                   # Alerts module
│       └── receipts/                 # Receipts module
├── logs/                             # Log files directory
├── .env.example                      # Environment variables template
├── .gitignore
├── tsconfig.json                     # TypeScript configuration
├── package.json
└── README.md
```

Each module follows a consistent structure:
- `*.model.ts` - Database models (TypeScript interfaces)
- `*.dto.ts` - Data Transfer Objects for requests/responses
- `*.service.ts` - Business logic and database queries
- `*.controller.ts` - HTTP request handlers
- `*.routes.ts` - Express route definitions

## Database Schema

The API connects to PostgreSQL and uses two schemas:

- **`application.*`** - Views with enriched data (used for most GET operations)
- **`raw.*`** - Raw tables (used for INSERT/UPDATE operations)

Key tables/views:
- `application.accounts` - Account data with balances
- `application.transactions` - Enriched transaction view
- `raw.transactions` - Raw transaction table (for writes)
- `application.categories` - Transaction categories
- `application.budgets` - Budget tracking
- `application.alerts` - System alerts
- `application.receipts` - Receipt documents

## Logging

Logs are written to:
- **Console**: All log levels with color coding
- **File**: `logs/run.log` (in production or when `ENABLE_FILE_LOGGING=true`)
- **Exceptions**: `logs/exceptions.log`
- **Rejections**: `logs/rejections.log`

Log format: `[YYYY-MM-DD HH:mm:ss] LEVEL: message {metadata}`

## Error Handling

The API provides consistent error responses:

```json
{
  "error": "Error message",
  "details": "Detailed error (development only)",
  "stack": "Stack trace (development only)"
}
```

Common HTTP status codes:
- `200` - Success
- `201` - Created (for POST requests)
- `400` - Bad Request (validation errors)
- `401` - Unauthorized (missing/invalid API key)
- `404` - Not Found (resource doesn't exist)
- `409` - Conflict (database constraint violations)
- `500` - Internal Server Error

## Running Both APIs in Parallel

You can run both the Rust API (port 8000) and Node.js API (port 8001) simultaneously for testing:

1. Start Rust API: `cd ../api && cargo run`
2. Start Node.js API: `cd ../api_js && npm run dev`

Both APIs connect to the same database and provide identical functionality.

## Differences from Rust API

While functionally identical, there are some implementation differences:

1. **Performance**: Node.js is single-threaded, 20-30% slower than Rust but adequate for typical loads
2. **Type Safety**: TypeScript provides good type safety but not as strict as Rust at compile time
3. **Memory**: JavaScript garbage collection vs Rust's ownership model
4. **Deployment**: Requires Node.js runtime vs Rust's single binary

## Development

### Type Checking

```bash
npx tsc --noEmit
```

### Code Formatting

The project uses ESLint and Prettier (if configured):
```bash
npm run lint
```

## Troubleshooting

### Database Connection Errors

- Verify `DATABASE_URL` in `.env` is correct
- Ensure PostgreSQL is running and accessible
- Check that the database schema exists

### Authentication Errors

- Verify `API_KEY` is set in `.env`
- Ensure requests include `x-api-key` header

### AWS S3 Errors

- Verify AWS credentials in `.env`
- Ensure S3 bucket exists and is accessible
- Check IAM permissions for S3 access

## License

ISC
