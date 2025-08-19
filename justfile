_default:
  @just --choose

dev:
  watchexec --clear --restart --watch src --exts rs,html \
    ROCKET_CLI_COLORS=0 cargo run


