# CAS Protocol for AI Agent Coordination

This document describes how multiple AI agents can collaboratively edit wiki pages without conflicting writes, using the Compare-and-Swap (CAS) concurrency control built into the wiki-rs REST API.

## Quick Start

Start the server (git backend recommended for version history):

```bash
cargo run -p wiki-server -- --backend git --data-dir ./work/agent-wiki
```

The server listens on port 7402 (git), 7401 (db), or 7400 (file). Override with `--port`.

## Protocol

### Step 1: Read a page and capture its ETag

```
GET /api/pages/FeaturePlan
```

Response:

```
HTTP/1.1 200 OK
ETag: "a1b2c3d4e5f6..."
Content-Type: application/json

{
  "title": "FeaturePlan",
  "content": "# Feature Plan\n\n- Item A\n- Item B",
  "created_at": 1711400000,
  "updated_at": 1711400000
}
```

The `ETag` is a SHA-256 hash of the page content. Save it -- you will need it for the conditional write.

### Step 2: Edit content locally

Make your changes to the content string. Do not write back to the server yet.

### Step 3: Propose your changes with If-Match

```
PUT /api/pages/FeaturePlan
Content-Type: application/json
If-Match: "a1b2c3d4e5f6..."

{
  "title": "FeaturePlan",
  "content": "# Feature Plan\n\n- Item A\n- Item B\n- Item C (added by Agent-1)"
}
```

### Step 4: Handle the response

**Success (200 OK):**

```
HTTP/1.1 200 OK
ETag: "f7e8d9c0b1a2..."
```

Your change was persisted. The new ETag reflects your updated content.

**Conflict (409 Conflict):**

```
HTTP/1.1 409 Conflict
ETag: "b2c3d4e5f6a7..."
Content-Type: application/json

{
  "error": "conflict",
  "message": "Page was modified by another writer",
  "current_page": {
    "title": "FeaturePlan",
    "content": "# Feature Plan\n\n- Item A\n- Item B\n- Item D (added by Agent-2)",
    "created_at": 1711400000,
    "updated_at": 1711400050
  },
  "current_etag": "\"b2c3d4e5f6a7...\""
}
```

Another agent modified the page after you read it. The response body contains the current page content and its ETag, so you can retry without an extra GET round-trip:

1. Re-read the content from `current_page` in the conflict response
2. Re-apply your edits to the new content
3. PUT again with the `current_etag` as the new `If-Match` value
4. Use random backoff between retries (e.g., 100-500ms)

## Round-Robin Coordination Pattern

For a group of agents coordinating on shared pages:

```
for each agent in round-robin order:
    attempts = 0
    loop:
        page = GET /api/pages/{title}  (or use conflict response body)
        etag = response ETag header
        new_content = agent.edit(page.content)
        response = PUT /api/pages/{title} with If-Match: etag
        if response.status == 200:
            break  # success, next agent's turn
        if response.status == 409:
            attempts += 1
            if attempts > MAX_RETRIES:
                break  # yield, try again next round
            sleep(random(100ms, 500ms))
            continue with conflict response body
```

After all agents have had their turn, each agent can GET the final pages to verify consensus.

## Other Useful Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/pages` | List all page titles (JSON array of strings) |
| HEAD | `/api/pages/{title}` | Check existence + get ETag without downloading content |
| DELETE | `/api/pages/{title}` | Delete a page |
| PUT | `/api/pages/{title}` | Unconditional write (omit If-Match header) |

## curl Examples

```bash
SERVER=http://localhost:7402

# List all pages
curl -s $SERVER/api/pages | jq .

# Get a page and capture its ETag
RESPONSE=$(curl -si $SERVER/api/pages/MainPage)
ETAG=$(echo "$RESPONSE" | grep -i '^etag:' | tr -d '\r' | cut -d' ' -f2)
echo "ETag: $ETAG"

# Conditional update (CAS)
curl -s -w "\n%{http_code}" \
  -X PUT \
  -H "Content-Type: application/json" \
  -H "If-Match: $ETAG" \
  -d '{"title":"MainPage","content":"# Updated by agent"}' \
  $SERVER/api/pages/MainPage

# Unconditional update (no CAS, last-writer-wins)
curl -s -X PUT \
  -H "Content-Type: application/json" \
  -d '{"title":"MainPage","content":"# Overwritten"}' \
  $SERVER/api/pages/MainPage
```

## Design Notes

- **ETag computation**: SHA-256 of the page content string, returned as a quoted hex string per HTTP spec.
- **Backward compatible**: PUT without `If-Match` performs an unconditional write (last-writer-wins). Existing clients are unaffected.
- **Per-page locking**: CAS operations acquire a lightweight per-page lock to ensure atomicity of the read-compare-write sequence. Different pages can be written concurrently without contention.
- **Git backend journaling**: The git backend uses a write-ahead journal -- file I/O is immediate, git commits are queued to a background worker. This means CAS responses are fast (sub-millisecond) while full version history is eventually consistent.
- **No authentication**: The server has no auth layer. For multi-agent use on a local machine or trusted network, this is fine. For shared environments, run behind a reverse proxy with auth.
