import SwaggerParser from '@apidevtools/swagger-parser'
import * as fs from 'fs'
import Adapter from './adapter.js'
import { exec } from 'node:child_process'
import process from 'node:process'
import path from 'node:path'
import { fileURLToPath } from 'url'

const dirname = path.dirname(fileURLToPath(import.meta.url))
const TEMPLATES_PATH = path.resolve(dirname, '../templates')
const SPEC_PATH = path.resolve(dirname, '../../openapi/spec.yaml')

const debug = false
const writeBashScript = true

const supportedGenerators = ['controllers', 'services'] // TODO: Add command support.

const resolveGeneratorType = () => {
  const result = process.argv[2]

  if (!result || supportedGenerators.indexOf(result) === -1) {
    return undefined
  }

  return result
}

const generatorType = resolveGeneratorType()

const log = (msg) => {
  if (!debug) {
    return
  }
  console.log(msg)
}

if (!generatorType) {
  console.log('generator type not valid; exiting')
  process.exit(1)
}

console.log(`processing generators for ${generatorType}`)

const targetDir = path.resolve(dirname, `./generated/${generatorType}`)

log('loading spec.yaml')

const bundle = await loadBundle()
const results = adaptBundle(bundle)

// log(JSON.stringify(results, null, 4));

console.log(`results: ${results.size}`)

const commands = []

// Ensure the output target directory exists.
if (!fs.existsSync(targetDir)) {
  console.log(`creating output directory ${targetDir}`)
  fs.mkdirSync(targetDir, { recursive: true })
}

results.forEach(adapter => {
  const command = `cargo generate --path ${TEMPLATES_PATH} ${generatorType} ${adapter.toOpts(generatorType)}`

  // exit early if command already added. Required for services.
  if (commands.indexOf(command) !== -1) {
    return
  }

  commands.push(command)

  console.log(`command: ${command}`)

  if (writeBashScript) {
    writeBashFile()
  }

  if (debug) {
    return
  }

  exec(command, {
    cwd: targetDir
  }, (error, stdout, stderr) => {
    if (error) {
      console.log(`error: ${error.message}`)
      return
    }
    if (stderr) {
      console.log(`stderr: ${stderr}`)
      return
    }
    console.log(`stdout: ${stdout}`)
  })
})

export async function loadBundle (specPath = SPEC_PATH) {
  return await SwaggerParser.bundle(specPath)
}

export function adaptBundle (bundle) {
  const results = new Map()
  const adapters = []

  for (const [route, operations] of Object.entries(bundle.paths)) {
    console.log(`processing route: ${route}`)
    for (const [method, operation] of Object.entries(operations)) {
      log(`processing method ${method} with tags ${operation.tags} for route ${route} for operation "${operation.summary}"`)
      operation.tags.forEach(tag => {
        const result = results[tag]
        // log(`result for tag ${tag}: ${JSON.stringify(result, null, 4)}`);

        const adapter = new Adapter(log, tag, method, route, operation)
        adapters.push(adapter)

        // log(`processing op: ${JSON.stringify(adapter, null, 4)}`);

        if (result) {
          // TODO: do this some other way
          // eslint-disable-next-line no-prototype-builtins
          if (results[tag].hasOwnProperty(route)) {
            log(`found route ${route} for tag ${tag}`)
            results[tag][route].push(adapter)
          } else {
            log(`route missed ${route} for tag ${tag}`)
            results[tag][route] = [adapter]
          }
        } else {
          log(`did not find tag ${tag} in results`)
          results[tag] = {
            [route]: [adapter]
          }
        }
      })
    };
  }

  if (debug) {
    // Output spec bundle.
    try {
      fs.writeFileSync(`${targetDir}/bundle.json`, JSON.stringify(bundle, null, 4))
      log('bundle file written')
    } catch (err) {
      console.error(err)
    }

    // Output transformation result.
    try {
      fs.writeFileSync(`${targetDir}/output.json`, JSON.stringify(results, null, 4))
      log('output file written')
    } catch (err) {
      console.error(err)
    }
  }

  return adapters
}

function writeBashFile () {
  if (commands.length < 1) {
    return
  }

  try {
    const bashFile = `${targetDir}/harbor-generate.sh`

    fs.writeFileSync(bashFile, '#!/bin/bash\n\n')

    commands.forEach(command => {
      console.info(`appending command: ${command}`)
      fs.appendFileSync(bashFile, `${command}\n`)
    })

    console.info('Script generation complete!')
  } catch (err) {
    console.error(err)
  }
}
