const BASE = '/api'

export interface DocumentSummary {
  id: number
  title: string
  length: number
  preview: string
}

export interface DocumentDetail {
  id: number
  title: string
  content: string
  length: number
}

export interface SearchHit {
  id: number
  title: string
  score: number
  snippet: string
}

export interface SearchResponse {
  query: string
  count: number
  results: SearchHit[]
}

export interface TermStat {
  term: string
  documentFrequency: number
}

export interface IndexStats {
  documentCount: number
  termCount: number
  averageLength: number
  topTerms: TermStat[]
}

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const response = await fetch(`${BASE}${path}`, init)
  if (!response.ok) {
    const message = await response.text()
    throw new Error(message || response.statusText)
  }
  if (response.status === 204) {
    return undefined as T
  }
  return (await response.json()) as T
}

export function listDocuments() {
  return request<DocumentSummary[]>('/documents')
}

export function getDocument(id: number) {
  return request<DocumentDetail>(`/documents/${id}`)
}

export function createDocument(title: string, content: string) {
  return request<DocumentSummary>('/documents', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ title, content }),
  })
}

export function deleteDocument(id: number) {
  return request<void>(`/documents/${id}`, { method: 'DELETE' })
}

export function search(query: string) {
  return request<SearchResponse>(`/search?q=${encodeURIComponent(query)}`)
}

export function getStats() {
  return request<IndexStats>('/index')
}
