# Telerust

Run db locally

```bash
$ docker run --rm --name postgres-rust -p 5432:5432 -e POSTGRES_PASSWORD=password -e POSTGRES_USER=user -e POSTGRES_DB=teleport postgres:alpine
```

Run migrations

```bash
$ sqlx database reset
```
