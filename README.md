# Welcome to jwt-auth

# About

This is a simple HTTP API written in Rust using the actix-web framework.

MongoDB has been used as the database.

# APIs

- /users -> type:get -> create a new user
- /authenticate -> type:post -> authenticate user
- /users/{user_id} -> type:get -> get user infromation
- /users/{user_id} -> type:put -> edit/update information
- /users/{user_id} -> type:delete -> delete user


# Environment Variables

 - MONGO_DB_NAME=myapp
 - MONGO_COLLECTION_NAME=mycollection
 - PUBLIC_KEY_PATH=keys/rsa_public.pem
 - PRIVATE_KEY_PATH=keys/rsa_private.pem
 - RUST_BACKTRACE=1
 - DB_CONN_STRING=mongodb://localhost:27017

 ## Start database locally
 
```sh
mongod --dbpath /path/to/dataset
```
with docker
```sh
docker run --name mongodb  -p 27017:27017 -e ALLOW_EMPTY_PASSWORD=yes -e MONGODB_EXTRA_FLAGS='--wiredTigerCacheSizeGB=2' bitnami/mongodb:latest
```
## Run Web Server

```sh
cargo run
```
