# upholi 🦜
[![build](https://github.com/eliefaart/upholi/actions/workflows/publish.yml/badge.svg)](https://github.com/eliefaart/upholi/actions/workflows/app.yml)
[![build](https://github.com/eliefaart/upholi/actions/workflows/validate.yml/badge.svg)](https://github.com/eliefaart/upholi/actions/workflows/validate.yml)

End-to-end encrypted personal photo library. Upload and view photos, sort them in albums, and share albums via a public password-protected link.

<p align="center">
  <img width="525" height="376" src="https://github.com/eliefaart/upholi/blob/main/.github/preview.png?raw=true">
</p>

## Repository anatomy

| Directory | Description                                                                            |
| :-------- | :------------------------------------------------------------------------------------- |
| app       | Yew.rs frontend application.                                                           |
| server    | Rust REST API that uses MongoDB as database.                                           |
| lib       | A rust crate that contains some types and functionality that 'app' and 'server' share. |

## Development status
This project is primarily a hobby project for personal use, and not recommended to use in production. It is also a work in progress and I may still make breaking changes to the interface and data models until the application is released as v1

## Encryption
All files and data are end-to-end encrypted, with a few small exceptions. All encryption is done with AES using a 256-bit key.

### What is not encrypted?
The following information is not encrypted, and/or can be determined by someone with full access to the database and storage:
- File size in bytes of each photo
- Usernames and password hashes
- When and how often a user has logged in

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
