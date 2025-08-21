_default:
  @just --choose

dev:
  concurrently -n css,srv -c cyan,green "just watch-css" "just serve"

serve:
  watchexec --clear --restart --watch src --exts rs,html,css,xml \
    --ignore src/assets/output.css ROCKET_CLI_COLORS=0 cargo run

build-css:
  @{{tailwind}} -i src/assets/styles.css -o src/assets/output.css

watch-css:
  @{{tailwind}} -i src/assets/styles.css -o src/assets/output.css --watch


# https://github.com/tailwindlabs/tailwindcss/releases/download/v4.1.12
tailwind := "tailwindcss-cli-v4-1-2"
