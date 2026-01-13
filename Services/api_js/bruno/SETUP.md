# Bruno Collection Setup Guide

## Quick Start

1. **Install Bruno**
   - Download from: https://www.usebruno.com/downloads
   - Or install via package manager:
     ```bash
     # Windows (Chocolatey)
     choco install bruno

     # macOS (Homebrew)
     brew install bruno

     # Linux (Snap)
     snap install bruno
     ```

2. **Open This Collection**
   - Launch Bruno
   - Click **"Open Collection"**
   - Navigate to and select this folder: `Services/api_js/bruno`
   - Bruno will load the `collection.bru` file automatically

3. **Configure Environment**
   - In Bruno, look for the environment dropdown (top right)
   - Select **"Local"** for development
   - Click the environment name to edit it
   - Update the `apiKey` value with your actual API key:
     ```
     vars {
       baseUrl: http://localhost:8001
       apiKey: YOUR-ACTUAL-API-KEY-HERE
     }
     ```

4. **Start Testing**
   - Ensure your API server is running: `npm run dev`
   - In Bruno, open any request from `v1/` or `v2/` folders
   - Click **"Send"** to execute the request
   - View the response in the bottom panel

## Collection Structure

```
bruno/
├── collection.bru              # Collection metadata (required by Bruno)
├── environments/               # Environment configurations
│   ├── Local.bru              # Local development
│   ├── Development.bru        # Dev server
│   ├── Staging.bru            # Staging server
│   └── Production.bru         # Production server
├── v1/                        # V1 API endpoints (snake_case)
│   └── *.bru                  # 14 request files
├── v2/                        # V2 API endpoints (camelCase)
│   └── *.bru                  # 14 request files
├── Health Check.bru           # Health check endpoint
├── README.md                  # Detailed documentation
└── SETUP.md                   # This file
```

## Troubleshooting

### "Collection not found" or "workspace.yml not found"

**Solution:** Make sure you're opening the `bruno` folder itself, not a subfolder.

Correct path: `Services/api_js/bruno/`
Incorrect: `Services/api_js/bruno/v1/`

### "Environment variables not working"

**Solution:**
1. Select an environment from the dropdown
2. Click on the environment name to edit it
3. Save after making changes
4. Requests will automatically use `{{baseUrl}}` and `{{apiKey}}`

### "401 Unauthorized" errors

**Solution:**
1. Check if API key is set in the active environment
2. Verify the API key is valid
3. Check if the server is running with the correct API key in `.env`

### Requests show "ECONNREFUSED"

**Solution:**
1. Start the API server: `cd Services/api_js && npm run dev`
2. Verify the server is running on port 8001
3. Check the `baseUrl` in your environment matches the server URL

## Testing Workflow

### 1. Health Check
```
Request: Health Check.bru
Expected: {"status": "ok"}
```

### 2. Test V1 API (snake_case)
```
Request: v1/Get Transactions.bru
Response fields: account_id, booking_date, amount_cents
```

### 3. Test V2 API (camelCase)
```
Request: v2/Get Transactions.bru
Response fields: accountId, bookingDate, amountCents
```

### 4. Compare Responses
Open both requests side-by-side to see the field naming differences.

## Environment Switching

Switch between environments without changing requests:

1. **Local Development:**
   - Select "Local" environment
   - Points to: `http://localhost:8001`
   - Use local API key

2. **Remote Testing:**
   - Select "Development", "Staging", or "Production"
   - Update the environment file with correct URL and API key
   - All requests automatically use the new environment

## Tips

- **Keyboard Shortcuts:**
  - `Ctrl/Cmd + Enter`: Send request
  - `Ctrl/Cmd + S`: Save request changes
  - `Ctrl/Cmd + K`: Quick search

- **Query Parameters:**
  - Parameters starting with `~` are disabled
  - Remove `~` to enable them
  - Example: `~category: GROCERIES` (disabled) → `category: GROCERIES` (enabled)

- **Variable Usage:**
  - Use `{{baseUrl}}` for the API URL
  - Use `{{apiKey}}` for authentication
  - Bruno replaces these with environment values automatically

## Next Steps

1. ✅ Install Bruno
2. ✅ Open this collection
3. ⚠️ Configure Local environment with your API key
4. ⚠️ Start the API server (`npm run dev`)
5. ⚠️ Test Health Check endpoint
6. ⚠️ Test V1 and V2 endpoints

For more details, see [README.md](./README.md)
