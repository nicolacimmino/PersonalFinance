# Personal Finance API - Bruno Collection

This Bruno collection provides comprehensive API testing for both V1 (snake_case) and V2 (camelCase) versions of the Personal Finance API.

## Prerequisites

Install Bruno: https://www.usebruno.com/downloads

## Collection Structure

```
bruno/
├── bruno.json                  # Collection configuration
├── environments/               # Environment configurations
│   ├── Local.bru              # Local development (localhost:8001)
│   ├── Development.bru        # Dev environment
│   ├── Staging.bru            # Staging environment
│   └── Production.bru         # Production environment
├── v1/                        # V1 API calls (snake_case)
│   ├── Get Transactions.bru
│   ├── Create Transaction.bru
│   ├── Get Accounts.bru
│   └── ...
├── v2/                        # V2 API calls (camelCase)
│   ├── Get Transactions.bru
│   ├── Create Transaction.bru
│   ├── Get Accounts.bru
│   └── ...
└── Health Check.bru           # Health check endpoint
```

## Getting Started

1. **Open the collection in Bruno:**
   ```bash
   # Open Bruno and select "Open Collection"
   # Navigate to: Services/api_js/bruno
   ```

2. **Select an environment:**
   - Click on the environment dropdown (top right)
   - Choose "Local" for development
   - Other environments: Development, Staging, Production

3. **Configure API Key:**
   - Edit the selected environment file
   - Update the `apiKey` variable with your actual API key
   - For Local: Update `bruno/environments/Local.bru`

## Environment Variables

Each environment uses these variables:

| Variable | Description | Example |
|----------|-------------|---------|
| `baseUrl` | API base URL | `http://localhost:8001` |
| `apiKey` | API authentication key | `your-api-key-here` |

## API Versions

### V1 - snake_case (Legacy)

- **Path**: `/api/v1/*` or `/api/*`
- **Field Naming**: snake_case (e.g., `account_id`, `booking_date`)
- **Query Params**: snake_case (e.g., `date_from`, `date_to`)
- **Folder**: `bruno/v1/`

**Example Request:**
```bash
GET /api/v1/transactions?date_from=2024-01-01&date_to=2024-12-31
```

**Example Response:**
```json
{
  "id": 1,
  "account_id": 1,
  "account_name": "Main Checking",
  "booking_date": "2024-01-15T00:00:00.000Z",
  "amount_cents": -5499,
  "creditor_name": "SuperMarket Inc"
}
```

### V2 - camelCase (Current)

- **Path**: `/api/v2/*`
- **Field Naming**: camelCase (e.g., `accountId`, `bookingDate`)
- **Query Params**: camelCase (e.g., `dateFrom`, `dateTo`)
- **Folder**: `bruno/v2/`

**Example Request:**
```bash
GET /api/v2/transactions?dateFrom=2024-01-01&dateTo=2024-12-31
```

**Example Response:**
```json
{
  "id": 1,
  "accountId": 1,
  "accountName": "Main Checking",
  "bookingDate": "2024-01-15T00:00:00.000Z",
  "amountCents": -5499,
  "creditorName": "SuperMarket Inc"
}
```

## Available Endpoints

### Common

- **Health Check**: `/health` - Server health status (no auth)

### Transactions

| Method | V1 Endpoint | V2 Endpoint | Description |
|--------|-------------|-------------|-------------|
| GET | `/api/v1/transactions` | `/api/v2/transactions` | List transactions with filters |
| GET | `/api/v1/transactions/:id` | `/api/v2/transactions/:id` | Get single transaction |
| POST | `/api/v1/transactions` | `/api/v2/transactions` | Create transaction |
| PATCH | `/api/v1/transactions/:id` | `/api/v2/transactions/:id` | Update transaction |

### Accounts

| Method | V1 Endpoint | V2 Endpoint | Description |
|--------|-------------|-------------|-------------|
| GET | `/api/v1/accounts` | `/api/v2/accounts` | List all accounts |
| GET | `/api/v1/accounts/:id` | `/api/v2/accounts/:id` | Get single account |

### Categories

| Method | V1 Endpoint | V2 Endpoint | Description |
|--------|-------------|-------------|-------------|
| GET | `/api/v1/categories` | `/api/v2/categories` | List all categories |

### Budgets

| Method | V1 Endpoint | V2 Endpoint | Description |
|--------|-------------|-------------|-------------|
| GET | `/api/v1/budgets` | `/api/v2/budgets` | List all budgets |

### Alerts

| Method | V1 Endpoint | V2 Endpoint | Description |
|--------|-------------|-------------|-------------|
| GET | `/api/v1/alerts` | `/api/v2/alerts` | List all alerts |

### Receipts

| Method | V1 Endpoint | V2 Endpoint | Description |
|--------|-------------|-------------|-------------|
| GET | `/api/v1/receipts` | `/api/v2/receipts` | List all receipts |
| GET | `/api/v1/receipts/:id` | `/api/v2/receipts/:id` | Get single receipt |
| GET | `/api/v1/receipts/:id/pdf` | `/api/v2/receipts/:id/pdf` | Download receipt PDF |

### Reports

| Method | V1 Endpoint | V2 Endpoint | Description |
|--------|-------------|-------------|-------------|
| GET | `/api/v1/reports/by_category` | `/api/v2/reports/by_category` | Spending by category |
| GET | `/api/v1/reports/indicators` | `/api/v2/reports/indicators` | KPI indicators |

## Field Mapping Reference (V1 → V2)

### Transactions
- `account_id` → `accountId`
- `account_name` → `accountName`
- `booking_date` → `bookingDate`
- `creditor_name` → `creditorName`
- `amount_cents` → `amountCents`
- `amount_cents_in_ref_currency` → `amountCentsInRefCurrency`
- `ref_currency` → `refCurrency`
- `account_to` → `accountTo`
- `account_to_name` → `accountToName`
- `receipt_id` → `receiptId`
- `value_date` → `valueDate`

### Accounts
- `asset_type` → `assetType`
- `pri_transactions_src` → `primaryTransactionSource`
- `balance_cents` → `balanceCents`
- `balance_eur_cents` → `balanceEurCents`

### Budgets
- `from_date` → `fromDate`
- `to_date` → `toDate`
- `amount_cents` → `amountCents`
- `spent_cents_eur` → `spentCentsEur`
- `spent_cents` → `spentCents`
- `transactions` → `transactionCount`

### Receipts
- `amount_cents` → `amountCents`
- `ext_id` → `externalId`
- `merchant_name` → `merchantName`
- `merchant_address` → `merchantAddress`
- `scan_file_name` → `scanFileName`
- `transaction_id` → `transactionId`
- `transaction_category` → `transactionCategory`
- `transaction_amount_cents` → `transactionAmountCents`
- `transaction_currency` → `transactionCurrency`
- `account_code` → `accountCode`
- `account_description` → `accountDescription`

### Reports
- `date_from` → `dateFrom`
- `date_to` → `dateTo`
- `total_cents` → `totalCents`
- `transactions_count` → `transactionCount`
- `category_description` → `categoryDescription`
- `ref_currency` → `refCurrency`

### Alerts
- `item_id` → `itemId`

## Testing Workflow

### 1. Health Check
Start by testing the health endpoint to ensure the server is running:
- Run: `Health Check.bru`
- Expected: `{"status": "ok"}`

### 2. Authentication Test
Test API key authentication:
- Run any V1 or V2 endpoint with correct API key
- Verify: 200 OK response
- Test with invalid key
- Verify: 401 Unauthorized

### 3. Compare V1 vs V2
Run the same request in both versions to see the difference:
- Run: `v1/Get Transactions.bru`
- Note the snake_case response fields
- Run: `v2/Get Transactions.bru`
- Note the camelCase response fields

### 4. Create and Update Flow
Test CRUD operations:
1. `v2/Create Transaction.bru` - Create a transaction
2. Copy the returned `id`
3. Update `v2/Get Transaction by ID.bru` with the ID
4. Run to verify creation
5. `v2/Update Transaction.bru` - Update the transaction
6. Verify changes

## Tips

1. **Disable Optional Query Params**: In Bruno, query params prefixed with `~` are disabled by default
   ```
   ~category: GROCERIES  # Disabled - remove ~ to enable
   ```

2. **Update IDs**: Replace placeholder IDs (e.g., `/transactions/1`) with actual IDs from your database

3. **Date Ranges**: Update date filters in report endpoints to match your data range

4. **Environment Switching**: Switch between environments to test against different servers without changing requests

5. **Documentation**: Each request includes inline documentation in the `docs` section

## Troubleshooting

### 401 Unauthorized
- Check API key in environment configuration
- Verify `X-API-Key` header is present
- Ensure API key is valid for the target environment

### 404 Not Found
- Verify server is running
- Check `baseUrl` in environment configuration
- Ensure correct API version path (`/api/v1/` or `/api/v2/`)

### 400 Bad Request (Create/Update)
- Verify required fields are present
- V2: Ensure camelCase field names (`amountCents`, not `amount_cents`)
- V1: Ensure snake_case field names (`amount_cents`, not `amountCents`)
- Check date format: `YYYY-MM-DD`

## Migration from V1 to V2

When migrating from V1 to V2:

1. Update base URLs: `/api/v1/` → `/api/v2/`
2. Convert all field names to camelCase
3. Update query parameters to camelCase
4. Test all endpoints thoroughly
5. Keep V1 collection for backward compatibility testing

## Contributing

When adding new endpoints:

1. Add to both `v1/` and `v2/` folders
2. Use appropriate naming convention per version
3. Include documentation in `docs` section
4. Add to this README's endpoint table
5. Update field mapping reference if needed
