# Hummingbird
Personal photo library

## Docker
In docker compose:

Must modify:
- HB_DB_CONNSTRING

May modify:
- 'outer' ports
- HB_DB_NAME

## Server configuration
Default configuration is inside ```/server/config/default.toml```. Each setting can also be set using environment variables. Environment variables overwrite the settings from the default config file.

| Environment variable 			| Description |
| :---------------------------- | :---------- |
| `HB_SERVER_ADDRESS`           | Address to bind to |
| `HB_DATABASE_CONNECTIONSTRING`| Connection string to database server |
| `HB_DATABASE_NAME`			| Name of database to use |
| `HB_STORAGE_DIRECTORYPHOTOS`	| Path to base directories where photos will be stored |
| `HB_OAUTH_CLIENTID`			| OAuth2 application client ID |
| `HB_OAUTH_CLIENTSECRET`		| OAuth2 application client secret |
| `HB_OAUTH_AUTHURL`			| OAuth2 authorization url |
| `HB_OAUTH_TOKENURL`			| OAuth2 token url |
| `HB_OAUTH_USERINFOURL` 		| Url from which information about the current use can be retreived using an oauth2 access token |

## License
This project is licensed under the MIT license. 