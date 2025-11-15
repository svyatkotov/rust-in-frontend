const wasm = await WebAssembly.instantiateStreaming(fetch('../factorial.wasm'))

console.log(wasm.instance.exports.factorial(0));
console.log(wasm.instance.exports.factorial(1));
console.log(wasm.instance.exports.factorial(5));
