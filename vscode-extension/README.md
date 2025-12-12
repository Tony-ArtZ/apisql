# ApiSQL for VS Code

**ApiSQL** is a query language that brings SQL's expressive power to REST APIs. Write familiar `SELECT`, `WHERE`, `ORDER BY`, and `JOIN`-like queries against any JSON API—no backend modifications required.

Unlike gRPC or GraphQL which demand server-side integration, ApiSQL is **purely frontend-facing**. Point it at any existing REST endpoint, and you get a structured, type-safe querying experience instantly.

![ApiSQL Suggestions](https://github.com/Tony-ArtZ/apisql/raw/main/public/suggestions.gif)

## What Makes ApiSQL Unique

### Live Schema Inference

The Language Server **actually executes** your API requests in the background and analyzes the returned JSON to build an on-the-fly schema. This means:

- **No manual type definitions**: No `.d.ts`, Protobuf, or GraphQL schemas to maintain.
- **Real-time accuracy**: If your API returns `{"users": [{"id": 1, "name": "Alice"}]}`, the LSP knows it and suggests `id` and `name` automatically.
- **Instant feedback**: Change the API endpoint? The autocomplete updates immediately.

### Full SQL-Like Query Power

ApiSQL supports the querying capabilities you know and love:

- **Filtering**: `WHERE age > 21 AND status = "active"`
- **Pattern Matching**: `WHERE email =~ ".*@example.com"`
- **Sorting**: `ORDER BY created_at DESC`
- **Limiting**: `LIMIT 10 OFFSET 20`
- **Projections**: `SELECT { id, name, isAdmin: role = "admin" }`
- **Computed Fields**: Create new fields on the fly with expressions
- **Nested Access**: `FROM body.data.users` to navigate deep JSON structures

All of this runs against live API data, transforming REST responses into queryable datasets.

## Features

### Intelligent Auto-Completion

Type `SELECT ` and the extension suggests the actual fields from your API's response. Type `user.` and it shows you `name`, `email`, `age`—whatever the API returned.

### Real-Time Diagnostics

- **Field Validation**: Warns if you reference fields that don't exist in the response.
- **Syntax Checking**: Highlights ApiSQL syntax errors instantly.
- **HTTP Warnings**: Shows alerts if the API request fails or times out.

### Syntax Highlighting

Full syntax support for ApiSQL keywords (`REQUEST`, `RESPONSE`, `FROM`, `SELECT`, `WHERE`), operators, strings, and comments.

### Works With Any API

Just point it at a URL. Works with:

- Public APIs (GitHub, PokéAPI, OpenWeather, etc.)
- Your own backend services
- Third-party services (Stripe, Twilio, etc.)

No special backend support needed—if it returns JSON over HTTP, it works.

### Cross-Platform Support

Pre-built Language Server binaries included for:

- **Windows** (x64)
- **Linux** (x64)
- **macOS** (Intel & Apple Silicon)

## How It Works

1. **Define a Request**: Write a `REQUEST` block pointing to any REST endpoint.
2. **Live Fetch**: The LSP executes the HTTP call and caches the response.
3. **Schema Inference**: The JSON structure is analyzed to extract field names and types.
4. **Write Queries**: Use SQL-like syntax to filter, sort, and transform the data.
5. **Get Tooling**: Autocomplete, validation, and diagnostics are powered by the real API response.

## Example: Querying the PokéAPI

```sql
-- Define the API endpoint
REQUEST GetPokemon
  GET https://pokeapi.co/api/v2/pokemon?limit=50

-- Query it like a database
RESPONSE
  FROM body.results
  WHERE name =~ "^char"           -- Filter by regex
  ORDER BY name ASC               -- Sort alphabetically
  SELECT {
    name,
    url,
    generation: 1                 -- Add computed field
  }
  LIMIT 5
```

**What happens:**

1. The LSP fetches the JSON from PokéAPI.
2. It sees `results` is an array with objects containing `name` and `url`.
3. When you type `WHERE`, it autocompletes `name` and `url` for you.
4. The query filters Pokémon whose names start with "char", sorts them, and limits to 5 results.

## Example: Working with Headers and Variables

```sql
-- Define reusable variables
USING
  token: "Bearer sk_live_..."
  apiBase: "https://api.stripe.com/v1"

REQUEST ListCustomers
  GET {apiBase}/customers
  HEADERS
    Authorization: {token}
  CACHE 60                        -- Cache for 60 seconds

RESPONSE
  FROM body.data
  WHERE created > 1640000000      -- Filter by timestamp
  SELECT {
    id,
    email,
    name,
    isPremium: metadata.tier = "premium"
  }
```

## Requirements

Everything is bundled—no external dependencies. Just install the extension and start writing ApiSQL.

## Feedback & Contributing

Found a bug or want to request a feature? Open an issue on our [GitHub Repository](https://github.com/Tony-ArtZ/apisql).

---

_ApiSQL: Treat every API like a database. Built with ❤️_
