import { strict as assert } from 'node:assert'
import { loadBundle } from './index.js'
import Adapter from './adapter.js'

// test('should process bundle', () => {
//     processBundle(testBundle);
// });

// test('should adapt paths', () => {
const testBundle = await loadBundle()
// console.log(JSON.stringify(testBundle, null, 4));

const operations = testBundle.paths['/api/v1/team/{teamId}']
// console.log(JSON.stringify(operations, null, 4));

const operation = operations.get
// console.log(JSON.stringify(operation, null, 4));

const adapter = new Adapter(console.log, 'team', 'get', '/api/v1/team/{teamId}', operation)

const result = adapter.adapt()
console.log(JSON.stringify(result, null, 4))

assert.equal('()', result.requestType)
assert.equal('crate::harbor::entities::Team', result.responseType)
assert.equal('get_team', result.handlerName)

// });
