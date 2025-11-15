const jsImports = {
    logStart: () => console.log('🚀 Начало вычисления'),
    logOperation: (num) => console.log(`📝 Число: ${num}`),
    logResult: (result) => console.log(`✅ Результат: ${result}`)
};

const wasm = await WebAssembly.instantiateStreaming(fetch('sum.wasm'), { env: jsImports });

wasm.instance.exports.sum(2, 5);
