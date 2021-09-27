# upholi
Personal photo library. Upload and view photos, and sort them in albums.

## Repository anatomy
### app
React/TypeScript frontend application. Consumes 'wasm'.

### lib
A rust crate that contains some types and functionality that 'server' and 'wasm' share.

### server
Rust REST API that uses MongoDB as database. Consumes 'lib'.

### wasm
Web assembly that exposes a JS client that a frontend can use. The web assembly contains most of the business logic and takes care of all server interaction, encryption/decryption of data, processing photos, etc. Consumes 'lib' and 'server'.

## Docker
Docker images for the server and frontend app are available. A docker compose file is provided to run both images, but you will still need to set up a MongoDB database server yourself.

## Server configuration
Default configuration is inside ```/server/config/default.toml```. Each setting can also be set using environment variables. Environment variables overwrite the settings from the default config file.

| Environment variable 					| Description |
| :------------------------------------ | :---------- |
| `HB_SERVER_ADDRESS`					| Address to bind to. |
| `HB_DATABASE_CONNECTIONSTRING`		| Connection string to database server. |
| `HB_DATABASE_NAME`					| Name of database to use. |
| `HB_STORAGE_PROVIDER`					| ```Disk``` \| ```Azure```. Storage provider. |
| `HB_STORAGE_DIRECTORYPHOTOS`			| Only when storage provider is ```Disk```. Path to directory in which photos will be stored. |
| `HB_STORAGE_AZURESTORAGEACCOUNTNAME`	| Only when storage provider is ```Azure```. Azure storage account name. |
| `HB_STORAGE_AZURESTORAGEACCOUNTKEY`	| Only when storage provider is ```Azure```. Azure storage account master key. |
