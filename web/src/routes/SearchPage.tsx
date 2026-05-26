import { useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { search } from '../api'

export function SearchPage() {
  const [input, setInput] = useState('')
  const [query, setQuery] = useState('')

  const { data, isFetching } = useQuery({
    queryKey: ['search', query],
    queryFn: () => search(query),
    enabled: query.length > 0,
  })

  function submit(event: React.FormEvent) {
    event.preventDefault()
    setQuery(input.trim())
  }

  return (
    <section className="panel">
      <h1>Full text search</h1>
      <form className="search-bar" onSubmit={submit}>
        <input
          className="input"
          value={input}
          onChange={(event) => setInput(event.target.value)}
          placeholder="Search indexed documents"
        />
        <button className="button" type="submit">
          Search
        </button>
      </form>

      {isFetching && <p className="muted">Searching...</p>}

      {data && !isFetching && (
        <div>
          <p className="muted">
            {data.count} result{data.count === 1 ? '' : 's'} for "{data.query}"
          </p>
          <ul className="results">
            {data.results.map((hit) => (
              <li key={hit.id} className="result">
                <div className="result-head">
                  <span className="result-title">{hit.title}</span>
                  <span className="badge">score {hit.score.toFixed(3)}</span>
                </div>
                <p className="snippet" dangerouslySetInnerHTML={{ __html: highlight(hit.snippet, data.query) }} />
              </li>
            ))}
          </ul>
        </div>
      )}
    </section>
  )
}

function highlight(text: string, query: string): string {
  const escaped = escapeHtml(text)
  const terms = query.split(/\s+/).filter(Boolean).map(escapeRegExp)
  if (terms.length === 0) {
    return escaped
  }
  const pattern = new RegExp(`(${terms.join('|')})`, 'gi')
  return escaped.replace(pattern, '<mark>$1</mark>')
}

function escapeHtml(text: string): string {
  return text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')
}

function escapeRegExp(text: string): string {
  return text.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
}
