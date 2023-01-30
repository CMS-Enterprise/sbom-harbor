import SwaggerParser from "@apidevtools/swagger-parser";
import * as fs from 'fs';
import Adapter from "./adapter.js";
import { exec } from 'node:child_process';

const debug = true;
const writeBashScript = true;

const log = (msg) => {
    if (!debug) {
        return
    }
    console.log(msg);
}

log("loading spec.yaml")
let bundle = await loadBundle();
let results = adaptBundle(bundle);

// log(JSON.stringify(results, null, 4));

console.log(`results: ${results.size}`);

let commands = [];

results.forEach(adapter => {
    if (adapter.tag !== "member") {
        return
    }

    //let command = `cd ../ && cargo generate --init DerekStrickland/generate-rust-lambda lambda ${adapter.toOpts()}`;
    let command = `cargo generate --path /Users/derek/code/aquia/aquia-rs-generate lambda ${adapter.toOpts()}`;
    commands.push(command);

    console.log(`command: ${command}`);

    if (writeBashScript) {
        writeBashFile();
    }

    if (debug) {
        return;
    }

    exec(command, (error, stdout, stderr) => {
        if (error) {
            console.log(`error: ${error.message}`);
            return;
        }
        if (stderr) {
            console.log(`stderr: ${stderr}`);
            return;
        }
        console.log(`stdout: ${stdout}`);
    });
});

export async function loadBundle(specPath = "/Users/derek/code/aquia/cyclonedx-python/openapi/spec.yaml") {
    return await SwaggerParser.bundle(specPath);
}

export function adaptBundle(bundle) {
    let results = new Map();
    let adapters = [];

    for (const [route, operations] of Object.entries(bundle.paths)) {
        if (route === "/api/v1/{teamId}/{projectId}/{codebaseId}/sbom") {
            log("skipping sbom upload route");
            continue;
        }

        console.log(`processing route: ${route}`);
        for (const [method, operation] of Object.entries(operations)) {
            log(`processing method ${method} with tags ${operation.tags} for route ${route} for operation "${operation.summary}"`);
            operation.tags.map(tag => {
                let result = results[tag];
                // log(`result for tag ${tag}: ${JSON.stringify(result, null, 4)}`);

                let adapter = new Adapter(log, tag, method, route, operation);
                adapters.push(adapter);

                // log(`processing op: ${JSON.stringify(adapter, null, 4)}`);

                if (result) {
                    if (results[tag].hasOwnProperty(route)) {
                        log(`found route ${route} for tag ${tag}`)
                        results[tag][route].push(adapter);
                    } else {
                        log(`route missed ${route} for tag ${tag}`)
                        results[tag][route] = [adapter]
                    }
                } else {
                    log(`did not find tag ${tag} in results`)
                    results[tag] = {
                        [route]: [adapter]
                    };
                }
            });
        };
    }

    if (debug) {
        // Output spec bundle.
        try {
            fs.writeFileSync('bundle.json', JSON.stringify(bundle, null, 4));
            log("bundle file written");
        } catch (err) {
            console.error(err);
        }

        // Output transformation result.
        try {
            fs.writeFileSync('output.json', JSON.stringify(results, null, 4));
            log("output file written");
        } catch (err) {
            console.error(err);
        }
    }

    return adapters;
}

function writeBashFile() {
    if (commands.length < 1) {
        return
    }

    try {
        fs.writeFileSync('aquia-generate.sh', "#!/bin/bash\n\n");

        commands.forEach(command => {
            console.info(`appending command: ${command}`);
            fs.appendFileSync('aquia-generate.sh', `${command}\n`);
        });

        console.info("Script generation complete!");
    } catch (err) {
        console.error(err);
    }
}
