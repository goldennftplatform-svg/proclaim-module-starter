<!-- SPDX-License-Identifier: MIT AND Apache-2.0 -->

# JS client example

`search.mjs` is a zero-dependency ESM example using the exact API shape of the
published `clients/js` package (`@lp0023/registry-client`) in
<https://github.com/goldennftplatform-svg/proclaim>.

## Get the client

From a checkout of the upstream repository:

```bash
npm install ./clients/js
```

Or, in your own project's `package.json`:

```json
{ "dependencies": { "@lp0023/registry-client": "file:../registry/clients/js" } }
```

There are no runtime dependencies. `fetch` must be global (Node.js 18+).

## Run

Start the read-only API (it never submits transactions) and then:

```bash
export REGISTRY_API_URL='http://127.0.0.1:3030'   # optional; this is the default
node search.mjs
```

## API surface used

| Call | Purpose |
| --- | --- |
| `new RegistryClient(baseUrl)` | Construct with an optional base URL (default `http://127.0.0.1:3030`) |
| `health()` | Liveness probe |
| `index()` | Array of base58 program IDs |
| `entries()` | All decoded entries |
| `search({ name, tags, verified, limit })` | Filtered entries (`name` substring, tags OR, verified flag, cap) |
| `entry(programId)` | One entry by base58/hex/byte-array ID |
| `idlAvailability(entry)` | `'available'` or `'unavailable'` — render metadata either way |
| `RegistryApiError` | `status` (0 = fetch failed) and parsed `body` |

Keys (`program_id`, `author`, `verifier`) are normalized to base58 by the
client. The client builds query strings with `URLSearchParams` and path
segments with `encodeURIComponent`, so CIDs containing `*` are safe.

Running this example locally is **not** third-party adoption; see the root
`README.md` and `docs/INTEGRATION-CHECKLIST.md` for the evidence requirements.
