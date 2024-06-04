# Auto JSDoc

## This project is WIP. DO NOT USE!!

#

This project aims to add [JsDoc](https://jsdoc.app/) blocks to functions and eventually update old blocks.

Input:
```ts
export function test() {

}

function testNoExport(param1: string, param2?: bool) {

}
```

#

Output:
```ts
/*
 * test
 */
export function test() {

}

/**
 * testNoExport
 *
 * @param {string} param1 - 
 * @param {bool} [param2] - 
 * @returns {void}
 */
function testNoExport(param1: string, param2?: bool) {

}
```

# TODO
* [X] Parsing - Support classes
* [ ] Parsing - Support optional defaults
* [X] Parsing - private functions
* [X] Parsing - Check exported and non exported functions
* [ ] Parsing - Add in @returns
* [ ] Parsing - Static function support
* [ ] CLI - Run as CLI with flags to target a .ts file (or maybe a group of files?)
