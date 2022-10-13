/**
 * The entry point for the cyclonedx-ui-sbom frontend.
 * @module @cyclonedx/ui/sbom
 */
import * as React from 'react'
import * as ReactDOMClient from 'react-dom/client'
import { HashRouter as Router } from 'react-router-dom'

// ** AWS Imports
import { Auth } from '@aws-amplify/auth'

// ** App Imports
import Main from '@/Main'
import { CONFIG } from '@/utils/constants'
import reportWebVitals from '@/utils/reportWebVitals'

/**
 * IIFE that initializes the root node and renders the application.
 */
;(async function () {
  // Initializes and configures Amazon Cognito authentication.
  Auth.configure({
    region: CONFIG.AWS_REGION,
    userPoolId: CONFIG.USER_POOL_ID || new Error('USER_POOL_ID is not defined'),
    userPoolWebClientId:
      CONFIG.USER_POOL_CLIENT_ID ||
      new Error('USER_POOL_CLIENT_ID is not defined'),
  })

  // create the element in the DOM
  const rootElement = document.createElement('div')
  rootElement.id = 'root'
  document.body.appendChild(rootElement)

  // create the React root node and render the application
  const root = ReactDOMClient.createRoot(rootElement)
  root.render(
    <React.StrictMode>
      <Router>
        <Main />
      </Router>
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
