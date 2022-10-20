/**
 * The entry point for the cyclonedx-ui-sbom frontend.
 * @module @cyclonedx/ui/sbom
 */
import * as React from 'react'
import * as ReactDOMClient from 'react-dom/client'
import { RouterProvider } from 'react-router-dom'
import { router } from '@/Routes'
import { CONFIG } from '@/utils/constants'
import configureCognito from '@/utils/configureCognito'
import reportWebVitals from '@/utils/reportWebVitals'

// IIFE that initializes the root node and renders the application.
;(async function () {
  configureCognito()

  // create the root element in the DOM
  const rootElement = document.createElement('div')
  rootElement.id = 'root'
  document.body.appendChild(rootElement)

  // create the React root node and render the application
  const root = ReactDOMClient.createRoot(rootElement)

  // render the application
  root.render(
    <React.StrictMode>
      <RouterProvider router={router} />
    </React.StrictMode>
  )

  // if NODE_ENV is production, return early. otherwise, run dev tools.
  if (process.env.NODE_ENV === 'production') return
  // print the global app CONFIG to the console
  console.log('Welcome to the Harbor!', CONFIG)
  // enable React performance measurement tools.
  // see https://create-react-app.dev/docs/measuring-performance/
  reportWebVitals(console.debug)
})()
