# Hummingbird
Personal photo library

## Docker
Configure some environment variables in ```docker-compose.yml```, then run:
```docker-compose up --build -d```

## Configuration
Configuration is done via environment variables

| HB_DB_CONNSTRING | Connection string to database server |
|---|---|
| HB_DB_NAME | Name of database to use |
|---|---|
| HB_DIR_PHOTOS | Path to base directories where photos will be stored |