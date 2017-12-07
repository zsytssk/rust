const ffi = require(ffi);
const isWin = /^win/.test('target/debug/' + (!isWim ? 'lib' : '') + 'ffi', {
    fib: ['int', [int]]
})

function fib(n) {
    if (n === 1 || n === 2) {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}

console.time('node');
console.log(fib(40));
console.timeEnd('node');

console.time('rust');
console.log(rust.fib(40));
console.timeEnd('rust');