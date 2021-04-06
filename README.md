# Website using Yew (RUST)

## Learning how to use Yew with Rust

This is a test on how to code in Rust with components that acts like React but is faster.

This is building to WebAssembly.

#### Commands
- install cargo and rust
- cargo new --lib name-of-app && cd name-of-app

- wasm-pack build --target web --out-name wasm --out-dir ./static (to build into the static folder)
Using python to create server
- python -m http.server 8000 --directory static (in the folder)

### Components/Building

- Created static folder (where the index.html and css with go and wasm files will go into).
    - Added bootstrap 5
    - Added script to inititalise wasm.js
        - < script type="module">
                import init from wasm.js
                init()
            </ script>

- src folder with lib.rs. (This renders the App, like App.js in React)
    - Added the boiler plate code from Yew

- Created 2 folders in src (components and pages)
    - Created home page and mod.rs to export the pages
    - Added counter in components and mod.rs to export that as one file

- Created app_router.rs for routing

