/**
 * The entry point for the cyclonedx-ui-sbom frontend.
 * @module @cyclonedx/ui/sbom/index
 */
import * as React from 'react'
import * as ReactDOMClient from 'react-dom/client'
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom'
import CssBaseline from '@mui/material/CssBaseline'
import { ThemeProvider } from '@mui/material/styles'
import App from '@/views/App/App'
import Home from '@/views/Home/Home'
import SignIn from '@/views/SignIn/SignIn'
import SignUp from '@/views/SignUp/SignUp'
import reportWebVitals from '@/utils/reportWebVitals'
import theme from '@/theme'

/**
 * IIFE that initializes the root node and renders the application.
 */
;(async function () {
  // create the element in the DOM
  const rootElement = document.createElement('div')
  rootElement.id = 'root'
  document.body.appendChild(rootElement)

  // create the React root node and render the application
  const root = ReactDOMClient.createRoot(rootElement)
  root.render(
    <React.StrictMode>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        <Router>
          <Routes>
            <Route path="/" element={<App />}>
              <Route index element={<Home />} />
              <Route path="join" element={<SignUp />} />
              <Route path="login" element={<SignIn />} />
            </Route>
          </Routes>
        </Router>
      </ThemeProvider>
    </React.StrictMode>
  )
})()

if (process.env.NODE_ENV !== 'production') {
  /** @see https://create-react-app.dev/docs/measuring-performance/ */
  reportWebVitals(console.log)
}
