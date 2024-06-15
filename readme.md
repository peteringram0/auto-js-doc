# Auto JSDoc

## This project is WIP. DO NOT USE!!

#

This project aims to add [JsDoc](https://jsdoc.app/) blocks to functions and eventually update old blocks.

Input:
```ts
function myFunction(param1: string, param2?: boolean, param3 = "default value"): Promise<string> {

}
```

Output:
```ts
/**
 * myFunction
 *
 * @param {string} param1 - 
 * @param {boolean} [param2] - 
 * @param {string} [param3="default value"] - 
 * @returns {Promise<string>}
 */
function myFunction(param1: string, param2?: boolean, param3 = "default value"): Promise<string> {

}
```

# TODO
* [X] Parsing - Support classes
* [X] Parsing - Support optional defaults
* [X] Parsing - private functions
* [X] Parsing - Check exported and non exported functions
* [X] Parsing - Static function support
* [X] Parsing - Should ignore constructor with `private a: string` as part of the arguments (need to check this)
* [X] Parsing - Support union type
* [X] Parsing - Add in @returns
* [ ] IO - setup stdin and stdout
* [ ] Site - lock down orgian allowed to call my instance
* [ ] Site - design to look better
* [ ] Hosting - Check over all settings
* [ ] Development - Nothing in place to make it easy to work on (testing out things on bun etc)

# Build

```bash
docker build -t auto-js .
docker run --name auto-js -p 3000:3000 auto-js
```

# Notes
https://github.com/tree-sitter/tree-sitter/discussions/1550
