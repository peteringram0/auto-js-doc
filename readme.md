# Auto JSDoc

## This project is WIP. DO NOT USE!!

#

This project aims to add [JsDoc](https://jsdoc.app/) blocks to functions and eventually update old blocks.

Input:
```ts
function myFunction(param1: string, param2?: boolean, param3 = "default value") {

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
 * @returns {void}
 */
function myFunction(param1: string, param2?: boolean, param3 = "default value") {

}
```

# TODO
* [X] Parsing - Support classes
* [X] Parsing - Support optional defaults
* [X] Parsing - private functions
* [X] Parsing - Check exported and non exported functions
* [X] Parsing - Static function support
* [ ] Parsing - Add in @returns
* [ ] Parsing - Support parmas without typing info
* [ ] Parsing - Support default values without typing information
* [ ] Parsing - Should ignore constructor with `private a: string` as part of the arguments (need to check this)
* [ ] CLI - Run as CLI with flags to target a .ts file (or maybe a group of files?)

# Know issues
* [X] exported function not working with params
