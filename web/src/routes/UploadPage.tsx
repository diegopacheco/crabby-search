import { useState } from 'react'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { createDocument } from '../api'

export function UploadPage() {
  const [title, setTitle] = useState('')
  const [content, setContent] = useState('')
  const queryClient = useQueryClient()

  const mutation = useMutation({
    mutationFn: () => createDocument(title.trim() || 'untitled', content),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['documents'] })
      queryClient.invalidateQueries({ queryKey: ['stats'] })
      setTitle('')
      setContent('')
    },
  })

  function onFile(event: React.ChangeEvent<HTMLInputElement>) {
    const file = event.target.files?.[0]
    if (!file) {
      return
    }
    if (!title) {
      setTitle(file.name)
    }
    const reader = new FileReader()
    reader.onload = () => setContent(String(reader.result ?? ''))
    reader.readAsText(file)
  }

  function submit(event: React.FormEvent) {
    event.preventDefault()
    if (content.trim()) {
      mutation.mutate()
    }
  }

  return (
    <section className="panel">
      <h1>Upload a document</h1>
      <form className="form" onSubmit={submit}>
        <label className="label">Title</label>
        <input
          className="input"
          value={title}
          onChange={(event) => setTitle(event.target.value)}
          placeholder="Document title"
        />

        <label className="label">Load from file</label>
        <input className="input" type="file" accept=".txt,.md,.json,.csv,.log" onChange={onFile} />

        <label className="label">Content</label>
        <textarea
          className="textarea"
          value={content}
          onChange={(event) => setContent(event.target.value)}
          rows={12}
          placeholder="Paste or type document content"
        />

        <button className="button" type="submit" disabled={mutation.isPending || !content.trim()}>
          {mutation.isPending ? 'Indexing...' : 'Index document'}
        </button>

        {mutation.isSuccess && <p className="success">Document indexed.</p>}
        {mutation.isError && <p className="error">{(mutation.error as Error).message}</p>}
      </form>
    </section>
  )
}
