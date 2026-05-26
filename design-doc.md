# crabby-search — Design Document

## 1. Summary

crabby-search is an in-memory full text search engine written in Rust, paired
with a web admin built on Vite, Bun, React and TanStack. The search core
(tokenizer, inverted index, BM25 ranking, document store) uses only the Rust
standard library. The REST layer is served by actix-web on the Tokio runtime.
The web admin lets a user upload documents, inspect the index, and run searches.

The engine targets Rust edition 2024 and toolchain 1.94 or newer.

## 2. Goals

- Index documents in memory and search them by relevance.
- Keep the search core free of third-party crates: standard library only.
- Serve a REST API with actix-web running on Tokio.
- High modularity: small, single-purpose modules across many files.
- Web admin to upload documents, view index statistics, and search.
- One-command run and stop through shell scripts.

## 2.1 Dependency boundary

Third-party crates are confined to the REST boundary. The search core has no
crate dependencies. The HTTP boundary uses:

- `actix-web` and `actix-cors` for the REST API and CORS.
- `tokio` as the async runtime driving the server.
- `serde` and `serde_json` for request and response bodies.

## 3. Non-Goals

- Persistence to disk (the index lives only while the process runs).
- Authentication, multi-tenancy, or access control.
- Distributed indexing or sharding.
- Stemming, lemmatization, or language-specific analysis beyond tokenization.

## 4. Architecture

```
+-------------------+        HTTP / JSON        +--------------------------+
|   Web admin       | <-----------------------> |   Rust engine            |
|   Vite + Bun      |   /api/documents          |   std-only HTTP server   |
|   React + TanStack|   /api/search             |   inverted index + BM25  |
+-------------------+   /api/index              +--------------------------+
```

Two independent processes:

1. The **engine** binds `127.0.0.1:7700` and serves a JSON API.
2. The **web admin** runs on Vite (`127.0.0.1:5173`) and proxies `/api` to the
   engine, so the browser talks to a single origin.

## 5. Engine modules

```
engine/src/
  main.rs              process entry, builds Tokio runtime, starts actix-web
  engine/
    mod.rs             re-exports the engine surface
    document.rs        Document record
    tokenizer.rs       text to lowercase alphanumeric terms
    store.rs           document storage and id assignment
    index.rs           inverted index, postings, document lengths
    search.rs          BM25 scoring and snippet extraction
    engine.rs          SearchEngine facade over store and index
  api/
    mod.rs             route configuration
    state.rs           AppState holding the engine behind a RwLock
    dto.rs             serde request and response types
    handlers.rs        actix handlers mapping engine data to JSON
```

The engine lives inside `AppState` as `RwLock<SearchEngine>`, shared across
workers through actix `web::Data`. Searches and listings take a read lock;
indexing and deletion take a write lock. Critical sections are short, so the
synchronous lock does not block the async workers meaningfully.

## 6. Data model

A document is:

```
Document {
  id: u64,
  title: String,
  content: String,
}
```

The store assigns sequential ids starting at 1 and keeps documents in a
`HashMap<u64, Document>`.

The inverted index keeps:

- `postings: HashMap<term, HashMap<doc_id, term_frequency>>`
- `doc_lengths: HashMap<doc_id, token_count>`
- `total_length: u64` for computing the average document length

## 7. Tokenization

Text is split on any non-alphanumeric character. Each run of alphanumeric
characters becomes one lowercase token. There is no stopword removal; BM25
inverse document frequency naturally down-weights common terms.

## 8. Ranking — BM25

For a query, the engine scores every document that contains at least one query
term and sums the per-term contributions:

```
idf(t)   = ln( (N - df(t) + 0.5) / (df(t) + 0.5) + 1 )
score    = sum over query terms of
           idf(t) * ( tf(t,d) * (k1 + 1) )
                   / ( tf(t,d) + k1 * (1 - b + b * dl(d) / avgdl) )
```

with `k1 = 1.2` and `b = 0.75`, where `N` is the document count, `df` the
document frequency of a term, `tf` the term frequency in the document, `dl` the
document length and `avgdl` the average document length. Results are sorted by
score descending, ties broken by id, then truncated to the requested limit.

## 9. Snippets

For each result the engine locates the first token that contains a query term
and returns a window of about thirty surrounding words, with leading and
trailing ellipses when the window is not at a document boundary. The web admin
highlights matching terms inside the snippet.

## 10. HTTP API

| Method | Path                   | Body / Query              | Response |
|--------|------------------------|---------------------------|----------|
| GET    | `/api/health`          | none                      | `{ "status": "ok" }` |
| POST   | `/api/documents`       | `{ "title", "content" }`  | created document summary |
| GET    | `/api/documents`       | none                      | array of document summaries |
| DELETE | `/api/documents/{id}`  | none                      | `204 No Content` |
| GET    | `/api/search`          | `q`, optional `limit`     | query, count, ranked results |
| GET    | `/api/index`           | none                      | document count, term count, average length, top terms |

A document summary is `{ id, title, length, preview }`. A search result is
`{ id, title, score, snippet }`. Responses are JSON and carry permissive CORS
headers via `actix-cors`.

## 11. HTTP server

actix-web runs on the Tokio runtime, started with `#[tokio::main]`. Routes are
registered through `App::configure`. Request bodies arrive as `web::Json` and
deserialize into serde request types; query strings arrive as `web::Query`; path
parameters such as a document id arrive as `web::Path`. Handlers acquire the
engine lock, perform the operation, and return serde response types serialized
to JSON. A permissive CORS layer is wrapped around the app.

## 12. Web admin

Built with Vite and run by Bun. React renders three TanStack Router routes
inside a shared layout:

- **Search** (`/`): a query box; results show title, score, and a highlighted snippet.
- **Upload** (`/upload`): title and content fields, plus a file picker that reads a text file into the content area; submitting indexes the document.
- **Indexes** (`/index`): document count, unique term count, average length, top terms by document frequency, and a document table with delete actions.

TanStack Query owns all server state. Mutations invalidate the `documents` and
`stats` queries so views refresh after indexing or deletion. Vite proxies `/api`
to the engine to keep the browser on one origin.

## 13. Scripts

- `release.sh`: builds the engine in release mode and installs web dependencies.
- `run-web.sh`: builds the engine if needed, starts it in the background, waits for `/api/health`, then starts the Vite dev server; records both process ids.
- `stop-web.sh`: terminates the recorded process trees for the web server and the engine.

## 14. Build and run

```
./release.sh
./run-web.sh
```

Then open `http://localhost:5173`. Stop everything with `./stop-web.sh`.

## 15. Concurrency and limits

- The index is protected by a single `RwLock`; reads run concurrently, writes are exclusive.
- The index is in memory only and is lost when the engine stops.
- There is no request size cap; documents are expected to be ordinary text.

## 16. Testing strategy

Unit tests cover tokenization, inverted index counts, and BM25 ordering. A
search over a known set of documents asserts that the document most relevant to
a query ranks first, encoding the intent that ranking reflects term frequency
and rarity rather than insertion order.

## 17. Future work

- Optional disk persistence and index reload on start.
- Phrase and boolean query operators.
- Token stemming and configurable stopwords.
- Pagination on search and document listing.
