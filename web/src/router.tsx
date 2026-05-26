import { createRootRoute, createRoute, createRouter, Link, Outlet } from '@tanstack/react-router'
import { SearchPage } from './routes/SearchPage'
import { UploadPage } from './routes/UploadPage'
import { IndexPage } from './routes/IndexPage'
import { DocumentPage } from './routes/DocumentPage'

const rootRoute = createRootRoute({
  component: () => (
    <div className="app">
      <header className="header">
        <div className="brand">
          <span className="brand-mark">crabby</span>-search
        </div>
        <nav className="nav">
          <Link to="/" className="nav-link" activeProps={{ className: 'nav-link active' }} activeOptions={{ exact: true }}>
            Search
          </Link>
          <Link to="/upload" className="nav-link" activeProps={{ className: 'nav-link active' }}>
            Upload
          </Link>
          <Link to="/indexes" className="nav-link" activeProps={{ className: 'nav-link active' }}>
            Indexes
          </Link>
        </nav>
      </header>
      <main className="content">
        <Outlet />
      </main>
    </div>
  ),
})

const searchRoute = createRoute({ getParentRoute: () => rootRoute, path: '/', component: SearchPage })
const uploadRoute = createRoute({ getParentRoute: () => rootRoute, path: '/upload', component: UploadPage })
const indexRoute = createRoute({ getParentRoute: () => rootRoute, path: '/indexes', component: IndexPage })
const documentRoute = createRoute({ getParentRoute: () => rootRoute, path: '/documents/$id', component: DocumentPage })

const routeTree = rootRoute.addChildren([searchRoute, uploadRoute, indexRoute, documentRoute])

export const router = createRouter({ routeTree })

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}
