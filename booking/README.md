Start a postgres instance
```
podman run --rm --name pg-instance -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" docker.io/postgres:alpine
```

In a new terminal, run `psql` and copy/paste content of `schema.sql`
```
podman exec -it -u postgres pg-instance psql
```

Run project
```
cargo run --release
```

