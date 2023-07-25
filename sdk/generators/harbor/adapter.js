const defaultImportPath = 'crate::entities'
// Can add more as needed.
const stdTypes = ['()', 'string']

export default class Adapter {
  constructor (log, tag, method, route, operation) {
    this.log = log
    this.tag = tag
    this.method = method
    this.route = route
    this.op = (({ summary, requestBody, responses }) => ({ summary, requestBody, responses }))(operation)
  }

  toOpts (generatorType) {
    switch (generatorType) {
      case 'controllers':
        return `-n ${this.handlerName()} -d tag=${this.tag} -d import_path=${defaultImportPath} -d operation_type=${this.operationType()} -d request_type=${this.toCrateType(this.requestType())} -d response_type=${this.toCrateType(this.responseType())}`
      case 'services':
        return `-n ${this.requestType()}s -d tag=${this.tag} -d import_path=${defaultImportPath} -d service_type=${this.requestType()}`
    }
  }

  operationType () {
    if (this.method !== 'get') {
      return this.method
    }

    if (this.responseType().includes('List')) {
      return 'list'
    }

    return this.method
  }

  requestType () {
    // By convention, delete operations accept only path or querystring paramaters.
    if (this.method === 'delete') {
      return '()'
    }
    // this.log(`this for requestType: ${JSON.stringify(this, null, 4)}`);
    if (this.op.requestBody) {
      return this.refToType(this.op.requestBody)
    }
    // If no requestBody, return the Unit Type.
    // this.log(`requestType Unit handler triggered: ${JSON.stringify(this.op.requestBody, null, 4)}`);
    return '()'
  }

  responseType () {
    // By convention, delete operations return nothing.
    if (this.method === 'delete') {
      return '()'
    }

    const responseType = this.op.responses['200']
    if (responseType) {
      // this.log(`responseType received component: ${JSON.stringify(responseType, null, 4)}`);
      return this.refToType(responseType)
    }
    // If no response type, return the Unit Type.
    // this.log(`responeType Unit handler triggered: ${JSON.stringify(responseType, null, 4)}`);
    return '()'
  }

  handlerName () {
    // Set default that is easy to spot when debugging.
    let typeName = 'unknown'

    switch (this.method) {
      case 'delete':
        // No type information available so we have to rely on route.
        typeName = this.route.substring(0, this.route.lastIndexOf('/')).split('/').pop()
        break
      case 'get':
        typeName = this.responseType()
        break
      case 'post':
      case 'put':
        typeName = this.requestType()
        break
    }

    // this.log(`handlerName computed typeName ${typeName} for method ${this.method}`);

    // Handle list operations
    if (typeName.includes('List')) {
      typeName = typeName.replace('List', 's')
      // this.log(`handlerName computed typeName for List: ${typeName}`);
    }

    return `${this.method.toLowerCase()}_${typeName.toLowerCase()}`
  }

  capitalize (str) {
    return str.charAt(0).toUpperCase() + str.slice(1)
  }

  refToType (component) {
    if (!component || !component.$ref) {
      // this.log(`refToType Unit handler triggered: received undefined component or $ref ${JSON.stringify(component, null, 4)}`);
      return '()'
    }

    // this.log(`refToType received component: ${JSON.stringify(component, null, 4)}`);

    // Strip potential ref paths.
    let typeName = component.$ref.replace('#/components/responses/', '')
    typeName = typeName.replace('#/components/requestBodies/', '')

    // this.log(`refToType sanitized typeName to: ${JSON.stringify(typeName, null, 4)}`);

    // Capitalize first letter.
    typeName = this.capitalize(typeName)
    // this.log(`refToType capitalized type name to: ${JSON.stringify(typeName, null, 4)}`);

    return typeName
  }

  toCrateType (typeName) {
    if (stdTypes.includes(typeName)) {
      if (typeName === '()') {
        // Escape for bash execution.
        console.log('formatting unit type')
        return '\\(\\)'
      }
      return typeName
    }

    // Handle list results
    if (typeName.includes('List')) {
      // Strip "List" from the type name.
      typeName = typeName.replace('List', '')
      // Returning <> is a parsing challenge when execing a
      // node subshell. We'll handle this in the cargo generate
      // until it proves unweildly.
      // typeName = `Vec<${typeName}>`;
    }

    return typeName
  }
}
