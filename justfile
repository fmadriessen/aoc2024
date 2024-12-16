set dotenv-load

create day:
    cargo generate --path ./template --name {{day}}

test day part:
    cargo nextest run --package {{day}} {{part}}

run day:
    cargo run --package {{day}}
