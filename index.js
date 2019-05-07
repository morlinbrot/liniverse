const wasm = import('./pkg/liniverse');

const canvas = document.getElementById('canvas');
const start_btn = document.getElementById('start-btn');

wasm.then(wasm => {
  const handler = wasm.main(
    canvas,
    start_btn,
  )
}).catch(console.error);
