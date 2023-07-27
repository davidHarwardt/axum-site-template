
watch:
    watchexec -c -r -e rs,css,js,ts,html -- just run

run:
    just build-css
    cargo run

build:
    just build-css && cargo build

build-css:
    npx tailwindcss -i ./src/style.css -o ./dist/style.css

build-release:
    just build-css-release && cargo build --release

build-css-release:
    npx tailwindcss -i ./src/style.css -o ./dist/style.css --minify

