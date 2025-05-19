default:
  @just --list


# Run 'cargo run' on the project
drun *ARGS:
  cargo run {{ARGS}}

dbuild:
  cargo build

flip IMAGE:
  magick {{IMAGE}} -flip {{IMAGE}}

# Run 'cargo watch' to run the project (auto-recompiles)
watch *ARGS:
  cargo watch -x "run -- {{ARGS}}"
