# Kassensystem

## Database connection
The application relies on a postgres Database instance
Following enironment variables need to be defined to allow a database connection.

Note: ssl-connection is enabled by default

| Enviroment variable | Example |
|---------------------|---------|
|APP_DATABASE__USERNAME| APP_DATABASE__USERNAME=postgres|
|APP_DATABASE__PASSWORD| APP_DATABASE__PASSWORD=password|
|APP_DATABASE__HOST    | APP_DATABASE__HOST=localhost|
|APP_DATABASE__PORT    | APP_DATABASE__PORT=5432|
|APP_DATABASE__DATABASE_NAME| APP_DATABASE__DATABASE_NAME=mydatabase|
