const wasm = import('./pkg/liniverse');

const canvas = document.getElementById('canvas');
const restart_btn = document.getElementById('restart-btn');
const play_pause_btn = document.getElementById('play-pause-btn');

canvas.width = window.innerWidth - 100;
canvas.height = window.innerHeight - 100;

wasm.then(wasm => {
    const handler = wasm.main(
        canvas,
        restart_btn,
        play_pause_btn,
    )
}).catch(console.error);
