const {Fib} = require('../pkg/fib_wasm.js');

const fib = new Fib();

for (let i = 0; i < 7; i++) {
  console.log(fib.next());
}
