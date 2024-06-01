# Auto JSDoc

## This project is WIP. DO NOT USE!!

#

This project aims to add [JsDoc](https://jsdoc.app/) blocks to functions and eventually update old blocks.

Input:
```ts
export function test() {

}

function testNoExport(param1: string) {

}
```

#

Output:
```ts
/**
 * test
 */
export function test() {

}

/**
 * testNoExport
 *
 * @param {string} param1
 * @returns {void}
 */
function testNoExport(param1: string) {

}
```
