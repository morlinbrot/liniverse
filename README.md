## liniverse
Simulation of a 2-dimensional galaxy of planets and the forces acting on them.

# Usage
`wasm-pack build`

`npm install`

`npm run serve`

# Docs
`cargo doc --no-deps --open`

## Credits
Massive kudos to https://gitlab.com/medusacle/wasm-game-of-life of whom I stole the elegant setup for controlling the animation loop inside the WASM module.

Implementation of the Barnes-Hut algorithm according to http://arborjs.org/docs/barnes-hut and https://www.cs.princeton.edu/courses/archive/fall03/cs126/assignments/nbody.html.