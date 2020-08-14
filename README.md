# Hummingbird
Personal photo library. Upload and view photos, and sort them in albums.

## Docker
Docker images for the server and frontend app are available. A docker compose file is provided to run both images, but you will still need to set up a MongoDB database server yourself.

## Server configuration
Default configuration is inside ```/server/config/default.toml```. Each setting can also be set using environment variables. Environment variables overwrite the settings from the default config file.

| Environment variable 			| Description |
| :---------------------------- | :---------- |
| `HB_SERVER_ADDRESS`           | Address to bind to |
| `HB_DATABASE_CONNECTIONSTRING`| Connection string to database server |
| `HB_DATABASE_NAME`			| Name of database to use |
| `HB_STORAGE_DIRECTORYPHOTOS`	| Path to directory in which photos will be stored |

### OAuth2 settings
Settings of OAuth2 providers can also be overwritten using environment variables, but because there can be multiple OAuth2 providers the keys are not fixed. The table below shows the available settings per OAuth2 provider.

| Environment variable 			| Description |
| :---------------------------- | :---------- |
| `HB_OAUTH_XXX_CLIENTID`   	| OAuth2 application client ID |
| `HB_OAUTH_XXX_CLIENTSECRET`   | OAuth2 application client secret |
| `HB_OAUTH_XXX_AUTHURL`    	| OAuth2 authorization url |
| `HB_OAUTH_XXX_TOKENURL`   	| OAuth2 token url |
| `HB_OAUTH_XXX_USERINFOURL`    | Url from which information about the current user can be retrieved using an oauth2 access token |

 The ```XXX``` within each environment variable must be replaced with the ID of the OAuth2 provider, this is not case sensitive. So for OAuth2 provider with ID 'github' (the only one supported right now), you could for example set environment variable ```HB_OAUTH_GITHUB_CLIENTSECRET```.

## License
This project is licensed under the MIT license. 