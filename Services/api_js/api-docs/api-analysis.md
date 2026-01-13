# API RESTful Violations and Naming Inconsistencies Analysis

## Overview
Comprehensive analysis of the Personal Finance API identifying RESTful violations and naming inconsistencies across all endpoints and DTOs.

**Purpose**: Documentation and reference for technical debt tracking. No immediate implementation planned.

**Scope**: Analysis covers all 7 API modules (Transactions, Categories, Accounts, Budgets, Alerts, Receipts, Reports)

---

## 1. RESTful Violations

### 1.1 Missing CRUD Operations (Incomplete REST Resources)

**Accounts Module** (`src/modules/accounts/account.routes.ts`)
- ❌ Missing: `POST /api/accounts` (create)
- ❌ Missing: `PATCH /api/accounts/:id` (update)
- ❌ Missing: `DELETE /api/accounts/:id` (delete)
- ✅ Has: `GET /api/accounts` and `GET /api/accounts/:id`

**Alerts Module** (`src/modules/alerts/alert.routes.ts`)
- ❌ Missing: `GET /api/alerts/:id` (get single)
- ❌ Missing: `POST /api/alerts` (create)
- ❌ Missing: `PATCH /api/alerts/:id` (update)
- ❌ Missing: `DELETE /api/alerts/:id` (delete)
- ✅ Has: Only `GET /api/alerts` (list)

**Budgets Module** (`src/modules/budgets/budget.routes.ts`)
- ❌ Missing: `GET /api/budgets/:id` (get single)
- ❌ Missing: `POST /api/budgets` (create)
- ❌ Missing: `PATCH /api/budgets/:id` (update)
- ❌ Missing: `DELETE /api/budgets/:id` (delete)
- ✅ Has: Only `GET /api/budgets` (list)

**Categories Module** (`src/modules/categories/category.routes.ts`)
- ❌ Missing: `GET /api/categories/:id` (get single)
- ❌ Missing: `POST /api/categories` (create)
- ❌ Missing: `PATCH /api/categories/:id` (update)
- ❌ Missing: `DELETE /api/categories/:id` (delete)
- ✅ Has: Only `GET /api/categories` (list)

**Receipts Module** (`src/modules/receipts/receipt.routes.ts`)
- ❌ Missing: `POST /api/receipts` (create)
- ❌ Missing: `PATCH /api/receipts/:id` (update)
- ❌ Missing: `DELETE /api/receipts/:id` (delete)
- ✅ Has: `GET /api/receipts`, `GET /api/receipts/:id`, `GET /api/receipts/:id/pdf`

**Transactions Module** (`src/modules/transactions/transaction.routes.ts`)
- ❌ Missing: `DELETE /api/transactions/:id` (delete)
- ✅ Has: Complete CRUD except delete (GET, POST, PATCH)

### 1.2 Non-RESTful URL Patterns

**Reports Module** (`src/modules/reports/report.routes.ts`)
- ❌ `GET /api/reports/by_category` - Contains action phrase "by_category"
  - **Issue**: Action-based URL instead of resource-oriented
  - **Better**: `GET /api/reports?type=category&date_from=...&date_to=...`
  - **Alternative**: `GET /api/category-reports?date_from=...&date_to=...`

- ⚠️ `GET /api/reports/indicators` - Multiple concerns
  - **Issue**: "indicators" might be its own resource, not a report type
  - **Better**: `GET /api/indicators?date_from=...&date_to=...`
  - **Or**: `GET /api/reports?type=indicators&date_from=...&date_to=...`

### 1.3 Inconsistent Filtering Patterns

**Transactions Module** - Uses query parameters (✅ Correct REST pattern)
```
GET /api/transactions?category=GROCERIES&account=1&date_from=2024-01-01&date_to=2024-01-31
```

**Reports Module** - Uses fixed paths instead of query params (❌ Inconsistent)
```
GET /api/reports/by_category?date_from=...&date_to=...
GET /api/reports/indicators?date_from=...&date_to=...
```

**Recommendation**: Standardize on query parameter filtering across all modules.

---

## 2. Naming Inconsistencies

### 2.1 Snake_case in JSON APIs (Database Naming Leak)

**Critical Issue**: All DTOs expose database field names (snake_case) instead of JavaScript/JSON standard (camelCase).

#### Affected Files and Field Count:

| Module | DTO File | Fields with snake_case | Total Fields |
|--------|----------|------------------------|--------------|
| Accounts | `account.dto.ts` | 4 of 10 | 40% |
| Alerts | `alert.dto.ts` | 1 of 5 | 20% |
| Budgets | `budget.dto.ts` | 5 of 11 | 45% |
| Categories | `category.dto.ts` | 0 of 4 | 0% ✅ |
| Receipts | `receipt.dto.ts` | 11 of 15 | 73% |
| Reports | `report.dto.ts` | 4 of 6 | 67% |
| Transactions | `transaction.dto.ts` | 10 of 13 | 77% |

#### Detailed Field Mappings:

**Accounts** (`src/modules/accounts/account.dto.ts`)
```typescript
// Current → Should be
asset_type → assetType
pri_transactions_src → primaryTransactionSource
balance_cents → balanceCents
balance_eur_cents → balanceEurCents
```

**Alerts** (`src/modules/alerts/alert.dto.ts`)
```typescript
item_id → itemId
```

**Budgets** (`src/modules/budgets/budget.dto.ts`)
```typescript
from_date → fromDate
to_date → toDate
amount_cents → amountCents
spent_cents_eur → spentCentsEur
spent_cents → spentCents
transactions → transactionCount  // Also ambiguous naming
```

**Receipts** (`src/modules/receipts/receipt.dto.ts`)
```typescript
amount_cents → amountCents
ext_id → externalId
merchant_name → merchantName
merchant_address → merchantAddress
scan_file_name → scanFileName
transaction_id → transactionId
transaction_category → transactionCategory
transaction_amount_cents → transactionAmountCents
transaction_currency → transactionCurrency
account_code → accountCode
account_description → accountDescription
```

**Reports** (`src/modules/reports/report.dto.ts`)
```typescript
total_cents → totalCents
transactions_count → transactionCount
category_description → categoryDescription
ref_currency → refCurrency
date_from → dateFrom  // In ReportByCategoryDto
date_to → dateTo      // In ReportByCategoryDto
```

**Transactions** (`src/modules/transactions/transaction.dto.ts`)
```typescript
account_id → accountId
account_name → accountName
booking_date → bookingDate
creditor_name → creditorName
amount_cents → amountCents
amount_cents_in_ref_currency → amountCentsInRefCurrency
ref_currency → refCurrency
account_to → accountTo
account_to_name → accountToName
receipt_id → receiptId
value_date → valueDate  // In CreateTransactionDto
```

### 2.2 Database Field with Reserved Keyword Handling

**Transaction Model** (`src/modules/transactions/transaction.model.ts`)
- Line 6: `type_: string` (database field with underscore to avoid SQL keyword)
- Line 22: `transaction_type: string` (application view uses different name)
- Line 43: `type_: string` (NewTransaction interface)

**Transaction DTO** (`src/modules/transactions/transaction.dto.ts`)
- Line 6: `type: string` (correctly normalized for API)

**Controller Mapping** (`src/modules/transactions/transaction.controller.ts`)
- Line 19: `transaction.transaction_type` → `dto.type` (inconsistent source field name)
- Line 109: `body.type` → `type_` (request mapping)

**Issue**: Three different names for the same concept:
1. `type_` (raw database field)
2. `transaction_type` (application view)
3. `type` (API DTO)

**Recommendation**: Standardize on `transactionType` in API to avoid ambiguity with other "type" fields.

### 2.3 Ambiguous Field Names

**Budget DTO** - `transactions` field
```typescript
transactions: number;  // Is this a count or an array? Unclear!
```
Should be: `transactionCount: number`

**Report DTO** - Correctly uses `transactions_count`
```typescript
transactions_count: number;  // Clear it's a count (but wrong case)
```
Should be: `transactionCount: number`

### 2.4 Excessive Abbreviations

**Account DTO** - `pri_transactions_src`
- Current: `pri_transactions_src: string`
- Meaning: Primary transactions source
- Issues: Double abbreviation (pri = primary, src = source)
- Should be: `primaryTransactionSource: string`

### 2.5 Type Field Collision

Multiple modules use `type` field with different semantic meanings:

1. **Transaction.type**: Transaction type (DEBIT, CREDIT, TRANSFER)
2. **ReportByCategoryEntry.type**: Category type (EXPENSE, INCOME)
3. **Account.asset_type**: Asset type (CHECKING, SAVINGS)

**Issue**: Field name collision across related DTOs creates confusion.

**Recommendation**: Use specific names:
- `transactionType` for transactions
- `categoryType` for reports
- `assetType` for accounts

### 2.6 Flattened Related Resources

**Receipt DTO** includes flattened transaction and account data:
```typescript
transaction_id: number | null;
transaction_category: string | null;
transaction_amount_cents: number | null;
transaction_currency: string | null;
account_code: string | null;
account_description: string | null;
```

**Issue**: REST best practice is to nest related resources or use separate endpoints.

**Better Structure**:
```typescript
transaction?: {
  id: number;
  category: string;
  amountCents: number;
  currency: string;
} | null;

account?: {
  code: string;
  description: string;
} | null;
```

**Or Use Relationships**:
```typescript
transactionId?: number | null;
// Client fetches: GET /api/transactions/:id if needed
```

---

## 3. Impact Assessment

### Breaking Changes Required

All naming consistency fixes will be **breaking changes** for API consumers:
- Frontend applications
- External integrations
- API documentation
- Test suites

### Risk Levels

**High Risk** (Most Breaking):
1. Renaming all DTO fields from snake_case to camelCase
2. Restructuring Receipt DTO to nest related data
3. Changing report endpoints from fixed paths to query parameters

**Medium Risk**:
1. Renaming ambiguous fields (e.g., `transactions` → `transactionCount`)
2. Implementing missing CRUD operations (additive, but changes API surface)

**Low Risk**:
1. Adding new optional query parameters
2. Documentation improvements

---

## 4. Recommended Remediation Approach

### Option A: Big Bang (All at once)
- Fix all issues in a single v2.0.0 API release
- Pros: Clean break, no technical debt
- Cons: Large migration effort for consumers

### Option B: Versioned API (Gradual)
- Maintain `/api/v1` with current issues
- Create `/api/v2` with all fixes
- Run both versions in parallel
- Sunset v1 after migration period

### Option C: Incremental (Conservative)
- Fix non-breaking issues first (add missing operations)
- Plan breaking changes for major version
- Use deprecation warnings

### Option D: Analysis Only
- Document all issues for reference
- No immediate changes
- Use as technical debt backlog

---

## 5. Implementation Plan (If Fixes Desired)

### Phase 1: Create Transformation Layer
1. Create mapper utilities to convert between DB models and DTOs
2. Implement camelCase transformation functions
3. Keep database models unchanged (snake_case)
4. Transform in controllers

### Phase 2: Update DTOs (Breaking)
1. Update all DTO interfaces to use camelCase
2. Update controllers to use transformation layer
3. Update OpenAPI documentation
4. Version API to /api/v2

### Phase 3: Standardize Endpoints (Breaking)
1. Refactor report endpoints to use query parameters
2. Add missing CRUD operations where needed
3. Restructure nested data (Receipt DTO)

### Phase 4: Testing & Migration
1. Update all test fixtures
2. Create migration guide for API consumers
3. Provide side-by-side comparison examples
4. Parallel run old and new API versions

---

## 6. Files Requiring Changes

### DTO Files (All need camelCase conversion):
- `src/modules/accounts/account.dto.ts`
- `src/modules/alerts/alert.dto.ts`
- `src/modules/budgets/budget.dto.ts`
- `src/modules/receipts/receipt.dto.ts`
- `src/modules/reports/report.dto.ts`
- `src/modules/transactions/transaction.dto.ts`

### Controller Files (Need transformation logic):
- `src/modules/accounts/account.controller.ts`
- `src/modules/alerts/alert.controller.ts`
- `src/modules/budgets/budget.controller.ts`
- `src/modules/receipts/receipt.controller.ts`
- `src/modules/reports/report.controller.ts`
- `src/modules/transactions/transaction.controller.ts`

### Route Files (Need new endpoints):
- `src/modules/accounts/account.routes.ts` - Add POST, PATCH, DELETE
- `src/modules/alerts/alert.routes.ts` - Add GET/:id, POST, PATCH, DELETE
- `src/modules/budgets/budget.routes.ts` - Add GET/:id, POST, PATCH, DELETE
- `src/modules/categories/category.routes.ts` - Add GET/:id, POST, PATCH, DELETE
- `src/modules/receipts/receipt.routes.ts` - Add POST, PATCH, DELETE
- `src/modules/transactions/transaction.routes.ts` - Add DELETE
- `src/modules/reports/report.routes.ts` - Refactor to use query params

### Service Files (Need new operations):
- All `*.service.ts` files will need corresponding CRUD methods

### Documentation:
- `api-docs/openapi.yaml` - Complete rewrite with corrected schemas
- `api-docs/README.md` - Update examples

---

## 7. Verification Plan

### Testing Strategy:
1. **Unit Tests**: Update all controller and service tests
2. **Integration Tests**: Test all endpoints with new naming
3. **Contract Tests**: Verify DTO schemas match OpenAPI spec
4. **Backward Compatibility**: If running v1 + v2, ensure v1 unchanged

### Validation Checklist:
- [ ] All DTO fields use camelCase
- [ ] No snake_case in API responses
- [ ] All resources have consistent CRUD operations
- [ ] Report endpoints use query parameters
- [ ] Receipt DTO properly nests related data
- [ ] Type fields have unambiguous names
- [ ] OpenAPI spec matches implementation
- [ ] All tests pass
- [ ] Bruno collections updated (found in ../../Bruno/PFinance/)
- [ ] Frontend integration tested

---

## 8. Priority Recommendations

### Priority 1 (Critical - Naming Standards):
- Convert all DTO fields to camelCase
- Create transformation layer in controllers
- This affects **every API endpoint**

### Priority 2 (Important - REST Compliance):
- Standardize report endpoints to use query parameters
- Add missing GET/:id endpoints for consistency

### Priority 3 (Nice to Have - Completeness):
- Add missing POST/PATCH/DELETE operations
- Restructure Receipt DTO for proper nesting

### Priority 4 (Future Enhancement):
- Add DELETE endpoint for transactions
- Consider soft delete vs hard delete for audit trail

---

## 9. Estimated Impact

### API Consumers Affected:
- Frontend application (../../Bruno/PFinance/ test collections found)
- Any external integrations using this API
- Documentation and examples

### Code Changes Estimated:
- **42+ fields** need renaming across all DTOs
- **6 controller files** need transformation updates
- **7 route files** need new endpoints or refactoring
- **OpenAPI spec** needs complete schema updates
- **Test files** need fixture updates

### Development Time (Rough Estimate):
- Analysis: Complete ✅
- Implementation: 3-5 days for full remediation
- Testing: 2-3 days
- Documentation: 1 day
- Total: ~1-2 weeks for complete solution

---

## Summary

The Personal Finance API has **systematic consistency issues** stemming from:
1. **Database-first design** leaking snake_case into JSON API
2. **Incomplete REST** implementations across modules
3. **Inconsistent patterns** for similar operations

**Recommendation**: Implement Option B (Versioned API) to provide clean v2 while maintaining v1 backward compatibility during migration period. This allows fixing all issues at once in v2 while giving consumers time to migrate.