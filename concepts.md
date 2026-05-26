# Concepts

This document explains the ideas behind crabby-search: what full text search
is, how an inverted index works, and how the BM25 ranking function decides which
documents are most relevant. It is meant to be read alongside the
[design document](design-doc.md), which describes how these ideas map to the
code.

## Full text search

Full text search answers a question of the form "which documents contain these
words, and which are the most relevant?" over a collection of text. Unlike an
exact lookup by key, it scans the words inside documents, ranks every candidate
by how well it matches the query, and returns the best matches first.

Two problems have to be solved:

1. **Matching**: find every document that contains the query terms.
2. **Ranking**: order those documents so the most relevant appear first.

crabby-search solves matching with an inverted index and ranking with BM25.

## Documents

A document is a unit of text with an id, a title, and a body of content. When a
document is indexed it is broken into terms and recorded in the index. The
original text is kept so it can be shown back and so snippets can be built.

## Tokenization

Tokenization turns raw text into a list of terms. crabby-search uses a simple,
predictable rule:

- Split the text on any character that is not a letter or a digit.
- Lowercase every remaining run of characters.

So `"Rust, the Language!"` becomes `["rust", "the", "language"]`. The same rule
is applied to documents when indexing and to queries when searching, which
guarantees a query term can match an indexed term.

There is no stopword removal. Very common words such as "the" are kept because
the ranking function already gives them almost no weight, as explained below.

## The inverted index

A naive search would scan every document for every query. That is too slow. An
inverted index flips the relationship around: instead of mapping a document to
its words, it maps each word to the documents that contain it.

```
term      ->  { document id -> term frequency }

"rust"    ->  { 2: 3, 6: 1 }
"index"   ->  { 4: 2, 5: 1 }
"search"  ->  { 2: 1, 3: 1, 4: 1, 5: 1, 6: 1 }
```

Each entry is a posting list. Reading "rust" tells us immediately that documents
2 and 6 contain it, that document 2 uses it three times, and that document 6
uses it once. The index also stores the length of every document and the running
total of all lengths, so the average document length is always available.

From this structure three quantities are cheap to read:

- **Term frequency** (`tf`): how often a term appears in one document.
- **Document frequency** (`df`): how many documents contain a term, which is the
  size of the term's posting list.
- **Document length** (`dl`) and the average length (`avgdl`) across the index.

## Why frequency alone is not enough

A first idea for ranking is to count how many times the query terms appear in
each document. This has two flaws:

- **Common words dominate.** A word that appears in almost every document tells
  you almost nothing about relevance, yet raw counts reward it.
- **Long documents win unfairly.** A long document repeats every word more often
  simply because it has more words, not because it is more relevant.

Good ranking has to discount common words and account for document length.

## Inverse document frequency

Inverse document frequency (`idf`) measures how rare and therefore how
informative a term is. A term in few documents is informative; a term in nearly
all documents is not. crabby-search uses:

```
idf(t) = ln( (N - df(t) + 0.5) / (df(t) + 0.5) + 1 )
```

where `N` is the number of documents and `df(t)` is the document frequency of
the term. As `df` approaches `N`, `idf` approaches zero, so a word that appears
everywhere contributes almost nothing to the score. This is why stopword removal
is unnecessary.

## BM25

BM25 is the ranking function used by crabby-search. For one query term in one
document it produces a score, and the document's total score is the sum over all
query terms:

```
score(d, q) = sum over terms t in q of

    idf(t) * ( tf(t, d) * (k1 + 1) )
            / ( tf(t, d) + k1 * (1 - b + b * dl(d) / avgdl) )
```

The constants are `k1 = 1.2` and `b = 0.75`, the common defaults.

Reading the parts:

- **`idf(t)`** weights the term by how rare it is across the collection.
- **`tf(t, d)`** rewards documents that use the term more, but with diminishing
  returns: `k1` caps how much extra weight repeated occurrences add, so the
  tenth occurrence matters far less than the second.
- **`b * dl(d) / avgdl`** normalizes by document length. A document longer than
  average is penalized and a shorter one is rewarded, with `b` controlling how
  strong that correction is. At `b = 0` length is ignored; at `b = 1` it is
  fully applied.

Compared to plain term-frequency times inverse-document-frequency, BM25 adds the
saturation from `k1` and the length normalization from `b`, which is why it is
the standard choice for text ranking.

## A search, step by step

Suppose the index holds the six documents shown in the web admin and the query
is `rust ownership`.

1. The query is tokenized into `["rust", "ownership"]`.
2. For each term, the posting list is read. Terms that are not in any document
   are skipped.
3. For every document in those posting lists, the BM25 contribution of each
   matching term is computed and summed into that document's score.
4. Documents are sorted by score descending, ties broken by id, and the top
   results are returned.

The document titled "Rust Ownership" scores highest because it contains both
terms, uses them more than once, and is close to the average length.

## Snippets and highlighting

For each returned document a snippet is built: the engine finds the first term
that matches the query and returns a window of about thirty words around it, with
ellipses when the window is not at the start or end of the document. The web
admin highlights the query terms inside the snippet so the match is visible at a
glance.

## In-memory design

crabby-search keeps the entire index in memory. This makes indexing and search
fast and the code small, at the cost of losing the index when the process stops.
The store and index are held behind a read-write lock so many searches can run
at once while indexing and deletion take exclusive access.

## Architecture in one paragraph

The search core, which is everything described above, uses only the Rust
standard library and has no knowledge of the network. A thin REST layer built
with actix-web on the Tokio runtime exposes the core over HTTP and converts
between JSON and the core's types. The web admin extracts text from uploaded
files in the browser and only ever sends plain text to the engine, so the engine
stays a pure text search engine. See the [design document](design-doc.md) for
the module layout.

## Glossary

- **Term**: a single token after tokenization.
- **Posting list**: the set of documents, with frequencies, for one term.
- **Term frequency (tf)**: occurrences of a term in one document.
- **Document frequency (df)**: number of documents containing a term.
- **Inverse document frequency (idf)**: a weight that grows as a term gets rarer.
- **Document length (dl)**: number of tokens in a document.
- **BM25**: the ranking function that combines tf, idf, and length normalization.
