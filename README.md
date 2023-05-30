## liniverse
2d-simulation of a universe of planets and the [Newtonian forces](https://en.wikipedia.org/wiki/Newton%27s_laws_of_motion) acting on them using a [quadtree](https://en.wikipedia.org/wiki/Quadtree).

# Usage
`wasm-pack build`

`npm install`

`npm run serve`

# Docs
`cargo doc --no-deps --open`

## Credits
Kudos to https://gitlab.com/medusacle/wasm-game-of-life of whom I stole the elegant setup for controlling the animation loop inside the WASM module.

Implementation of the Barnes-Hut algorithm according to http://arborjs.org/docs/barnes-hut and https://www.cs.princeton.edu/courses/archive/fall03/cs126/assignments/nbody.html.