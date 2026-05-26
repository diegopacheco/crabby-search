import { Link, useParams } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import { getDocument } from '../api'

export function DocumentPage() {
  const { id } = useParams({ from: '/documents/$id' })

  const { data, isLoading, isError, error } = useQuery({
    queryKey: ['document', id],
    queryFn: () => getDocument(Number(id)),
  })

  return (
    <section className="panel">
      <Link to="/" className="back-link">
        &larr; back to search
      </Link>
      <h1>Document {id}</h1>

      {isLoading && <p className="muted">Loading...</p>}
      {isError && <p className="error">{(error as Error).message}</p>}
      {data && <pre className="json">{JSON.stringify(data, null, 2)}</pre>}
    </section>
  )
}
