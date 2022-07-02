Start a postgres instance
```
docker run --rm --name pg-instance -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" postgres:alpine
```

Run psql in an other terminal
```
docker exec -it -u postgres pg-instance psql
```

Create a database, connect to it, and run schema.sql
```
create database coredb;
\c coredb;

-- paste schema.sql content 
```

Run project
```
cargo run --release
```

