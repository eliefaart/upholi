# Hummingbird
Personal photo library

## Useful commands
```docker-compose up --build -d --force-recreate```

```docker build . --file Dockerfile```

```docker pull docker.pkg.github.com/eliefaart/hummingbird/hummingbird-server:docker```

```docker pull docker.pkg.github.com/eliefaart/hummingbird/hummingbird-app:docker```

## Docker
In docker compose:

Must modify:
- HB_DB_CONNSTRING

May modify:
- 'outer' ports
- HB_DB_NAME

## Server configuration
Configuration is done via environment variables

| HB_DB_CONNSTRING | Connection string to database server |
| HB_DB_NAME | Name of database to use |
| HB_DIR_PHOTOS | Path to base directories where photos will be stored |