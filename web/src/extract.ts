export async function extractText(file: File): Promise<string> {
  if (isPdf(file)) {
    const { extractPdf } = await import('./pdf')
    return extractPdf(file)
  }
  return file.text()
}

function isPdf(file: File): boolean {
  return file.type === 'application/pdf' || file.name.toLowerCase().endsWith('.pdf')
}
