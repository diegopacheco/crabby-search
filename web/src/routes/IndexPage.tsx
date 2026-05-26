import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { deleteDocument, getStats, listDocuments } from '../api'

export function IndexPage() {
  const queryClient = useQueryClient()
  const stats = useQuery({ queryKey: ['stats'], queryFn: getStats })
  const documents = useQuery({ queryKey: ['documents'], queryFn: listDocuments })

  const remove = useMutation({
    mutationFn: (id: number) => deleteDocument(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['documents'] })
      queryClient.invalidateQueries({ queryKey: ['stats'] })
    },
  })

  return (
    <section className="panel">
      <h1>Index overview</h1>

      <div className="cards">
        <div className="card">
          <span className="card-value">{stats.data?.documentCount ?? 0}</span>
          <span className="card-label">Documents</span>
        </div>
        <div className="card">
          <span className="card-value">{stats.data?.termCount ?? 0}</span>
          <span className="card-label">Unique terms</span>
        </div>
        <div className="card">
          <span className="card-value">{stats.data ? stats.data.averageLength.toFixed(1) : '0'}</span>
          <span className="card-label">Avg length</span>
        </div>
      </div>

      <h2>Top terms</h2>
      <div className="terms">
        {stats.data?.topTerms.map((term) => (
          <span key={term.term} className="term-chip">
            {term.term}
            <span className="term-count">{term.documentFrequency}</span>
          </span>
        ))}
        {stats.data && stats.data.topTerms.length === 0 && <p className="muted">No terms indexed yet.</p>}
      </div>

      <h2>Documents</h2>
      <table className="table">
        <thead>
          <tr>
            <th>ID</th>
            <th>Title</th>
            <th>Length</th>
            <th>Preview</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          {documents.data?.map((document) => (
            <tr key={document.id}>
              <td>{document.id}</td>
              <td>{document.title}</td>
              <td>{document.length}</td>
              <td className="preview">{document.preview}</td>
              <td>
                <button className="link-button" onClick={() => remove.mutate(document.id)}>
                  delete
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      {documents.data && documents.data.length === 0 && <p className="muted">No documents indexed yet.</p>}
    </section>
  )
}
