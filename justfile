set dotenv-load

create day:
    cargo generate --path ./template --name {{day}}

test day:
    cargo test --package {{day}}

run day:
    cargo test --package {{day}}
    cargo run --package {{day}}
