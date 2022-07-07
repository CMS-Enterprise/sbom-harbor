/**
 * The entry point for the cyclonedx-ui-sbom frontend.
 * @module @cyclonedx/ui/sbom/index
 */
import * as React from 'react'
import * as ReactDOMClient from 'react-dom/client'
import { BrowserRouter as Router } from 'react-router-dom'
import { Auth } from '@aws-amplify/auth'
import { ConfigProvider } from '@/providers/ConfigContext'
import { AppConfig } from '@/utils/types'
import reportWebVitals from '@/utils/reportWebVitals'
import RootLayout from '@/layouts/RootLayout'

// get the app config from the environment variable defined by the webpack
// (craco) config. See {ui/sbom/src/utils/prebuild.js} for the implementation.
const ENV_CONFIG = JSON.parse(JSON.stringify(process.env.CONFIG))

// initialize the global app config object
export const CONFIG = {
  ...ENV_CONFIG,
  TEAMS_API_URL: `${ENV_CONFIG.API_URL}/team`,
} as AppConfig

/**
 * Initializes and configures Amazon Cognito authentication.
 */
function initCognitoAuth() {
  Auth.configure({
    region: CONFIG.AWS_REGION,
    userPoolId: CONFIG.USER_POOL_ID || new Error('USER_POOL_ID is not defined'),
    userPoolWebClientId:
      CONFIG.USER_POOL_CLIENT_ID ||
      new Error('USER_POOL_CLIENT_ID is not defined'),
  })
}

/**
 * Helper function that runs development-only code.
 *  - prints the config object to the console.
 *  - enables react performance measurements.
 */
function runDevTools() {
  // only run the dev tools if NODE_ENV is not production
  if (process.env.NODE_ENV === 'production') return
  // print the global app CONFIG to the console
  console.log('Welcome to the Harbor!', CONFIG)
  // enable React performance measurement tools.
  // see https://create-react-app.dev/docs/measuring-performance/
  reportWebVitals(console.debug)
}

/**
 * IIFE that initializes the root node and renders the application.
 */
;(async function () {
  initCognitoAuth()

  // create the element in the DOM
  const rootElement = document.createElement('div')
  rootElement.id = 'root'
  document.body.appendChild(rootElement)

  // create the React root node and render the application
  const root = ReactDOMClient.createRoot(rootElement)

  // render the application
  root.render(
    <React.StrictMode>
      <ConfigProvider initialState={CONFIG}>
        <Router>
          <RootLayout />
        </Router>
      </ConfigProvider>
    </React.StrictMode>
  )

  runDevTools()
})()
