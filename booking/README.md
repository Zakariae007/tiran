Start a postgres instance
```
podman run --rm --name pg-instance -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" docker.io/postgres:alpine
```

Run psql in an other terminal
```
podman exec -it -u postgres pg-instance psql
```

Run schema.sql
```
-- paste schema.sql content
```

Run project
```
cargo run --release
```

