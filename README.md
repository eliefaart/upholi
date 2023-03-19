# upholi
End-to-end encrypted personal photo library. Upload and view photos, sort them in albums, and share albums via a public password-protected link.

<p align="center">
  <img width="550" height="477" src="https://github.com/eliefaart/upholi/blob/master/.github/preview.png?raw=true">
</p>

## Repository anatomy

| Directory | Description                                                                                                                                                                                                          | Consumes                | Status                                                                                                                                                    |
| :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------- |
| app       | React/TypeScript frontend application.                                                                                                                                                                               | ```wasm```              | [![app](https://github.com/eliefaart/upholi/actions/workflows/app.yml/badge.svg)](https://github.com/eliefaart/upholi/actions/workflows/app.yml)          |
| server    | Rust REST API that uses MongoDB as database.                                                                                                                                                                         | ```lib```               | [![server](https://github.com/eliefaart/upholi/actions/workflows/server.yml/badge.svg)](https://github.com/eliefaart/upholi/actions/workflows/server.yml) |
| lib       | A rust crate that contains some types and functionality that 'server' and 'wasm' share.                                                                                                                              |                         | [![lib](https://github.com/eliefaart/upholi/actions/workflows/lib.yml/badge.svg)](https://github.com/eliefaart/upholi/actions/workflows/lib.yml)          |
| wasm      | Web assembly that exposes a JS client that a frontend can use. The web assembly contains most of the business logic and takes care of all server interaction, encryption/decryption of data, processing photos, etc. | ```server```, ```lib``` | [![wasm](https://github.com/eliefaart/upholi/actions/workflows/wasm.yml/badge.svg)](https://github.com/eliefaart/upholi/actions/workflows/wasm.yml)       |

## Development status
This project is a work in progress. It's basically feature-complete for the initial release and relatively usable, but I may still make breaking changes to the interface and data models until the application is released as v1.
It is primarily a hobby project for personal use, and wouldn't recommend using it in production.

## Encryption
All files and data are end-to-end encrypted, with a few small exceptions. All encryption is done with AES using a 128-bit key.

### What is not encrypted?
The following information is not encrypted, and/or can be determined by someone with full access to the database and storage.
- Dimensions (width and height) and timestamp (uploaded on) of each photo
- File size in bytes of each photo
- Number of photos, albums and shares of a user
- Usernames
- Password hashes
- How often and when a user has logged in

## Docker
A docker image is available.

```docker pull ghcr.io/eliefaart/upholi/upholi:latest```

A docker compose file is included in the repo to run the image, but you will still need to set up a MongoDB database server yourself.

## Server configuration
Default configuration is inside ```/server/config/default.toml```. Each setting can also be set using environment variables. Environment variables overwrite the settings from the default config file.

| Environment variable                     | Description                                                                                 |
| :--------------------------------------- | :------------------------------------------------------------------------------------------ |
| `UPHOLI_SERVER_ADDRESS`                  | Address to bind to.                                                                         |
| `UPHOLI_SERVER_WWWROOT_PATH`             | Path to the app's `wwwroot` directory.                                                      |
| `UPHOLI_DATABASE_CONNECTIONSTRING`       | Connection string to database server.                                                       |
| `UPHOLI_STORAGE_PROVIDER`                | ```Disk``` \| ```Azure```. Storage provider.                                                |
| `UPHOLI_STORAGE_DIRECTORYPHOTOS`         | Only when storage provider is ```Disk```. Path to directory in which photos will be stored. |
| `UPHOLI_STORAGE_AZURESTORAGEACCOUNTNAME` | Only when storage provider is ```Azure```. Azure storage account name.                      |
| `UPHOLI_STORAGE_AZURESTORAGEACCOUNTKEY`  | Only when storage provider is ```Azure```. Azure storage account master key.                |
