const wasm = import('./pkg/liniverse');

const canvas = document.getElementById('canvas');
const start_btn = document.getElementById('start-btn');
const stop_btn = document.getElementById('stop-btn');

wasm.then(wasm => {
  const handler = wasm.main(
    canvas,
    start_btn,
    stop_btn,
  )
}).catch(console.error);
