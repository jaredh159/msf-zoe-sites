_default:
  @just --choose

dev:
  concurrently -n css,srv -c cyan,green "just watch-css" "just serve"

serve:
  watchexec --clear --restart --watch src --exts rs,html,css,xml,webp,png,svg \
    --ignore {{cssdir}}/output.css ROCKET_CLI_COLORS=0 cargo run

build-css:
  {{tailwind}} -i {{cssdir}}/styles.css -o {{cssdir}}/output.css

watch-css:
  {{tailwind}} -i {{cssdir}}/styles.css -o {{cssdir}}/output.css --watch

format:
  prettier --write src/assets/html/*.html


# https://github.com/tailwindlabs/tailwindcss/releases/download/v4.1.12
tailwind := "tailwindcss-cli-v4-1-2"
cssdir := "src/assets/css"
