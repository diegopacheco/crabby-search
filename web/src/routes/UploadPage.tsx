import { useState } from 'react'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { createDocument } from '../api'
import { extractText } from '../extract'

export function UploadPage() {
  const [title, setTitle] = useState('')
  const [content, setContent] = useState('')
  const [reading, setReading] = useState(false)
  const [fileError, setFileError] = useState('')
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

  async function onFile(event: React.ChangeEvent<HTMLInputElement>) {
    const file = event.target.files?.[0]
    if (!file) {
      return
    }
    if (!title) {
      setTitle(file.name)
    }
    setFileError('')
    setReading(true)
    try {
      setContent(await extractText(file))
    } catch {
      setContent('')
      setFileError(`Could not read ${file.name}`)
    } finally {
      setReading(false)
    }
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

        <label className="label">Load from file (json, xml, txt, md, pdf)</label>
        <input
          className="input"
          type="file"
          accept=".json,.xml,.txt,.md,.pdf,application/json,application/xml,text/xml,text/plain,text/markdown,application/pdf"
          onChange={onFile}
        />
        {reading && <p className="muted">Reading file...</p>}
        {fileError && <p className="error">{fileError}</p>}

        <label className="label">Content</label>
        <textarea
          className="textarea"
          value={content}
          onChange={(event) => setContent(event.target.value)}
          rows={12}
          placeholder="Paste or type document content"
        />

        <button className="button" type="submit" disabled={mutation.isPending || reading || !content.trim()}>
          {mutation.isPending ? 'Indexing...' : 'Index document'}
        </button>

        {mutation.isSuccess && <p className="success">Document indexed.</p>}
        {mutation.isError && <p className="error">{(mutation.error as Error).message}</p>}
      </form>
    </section>
  )
}
