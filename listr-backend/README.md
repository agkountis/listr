# Listr backend API - Actix Web
This crate is the backend web API of listr.

It provides the following API:
* Fetch available lists. 
  * [GET] https://localhost:80/api/v1/lists
* Fetch the items of the specified list.
  * [GET] https://localhost:80/api/v1/lists/{id}
* Add a new item to a list.
  * [POST] https://localhost:80/api/v1/lists/{id}/add
* Delete an item from a list.
  * [DELETE] https://localhost:80/api/v1/lists/item/{id}/delete

# Encryption
The backend also serves the front end website and provides SSL encryption (HTTPS) using openssl.

This is done in preparation for using AWS cognito to delegate authentication of users to an external service.

OpenSSL can be pain to develop with on Windows due to the lack of 
the development libraries needed by the openssl Rust crate to compile/link with the native
openssl code.

To allow the server to support HTTPS we need to create a self-signed SSL certificate.
This can be done manually using the openssl CLI or to avoid all that and have a quick and easy cert generation 
you can use ``mkcert``. This tool will create CA (Certificate Authority) and will sign the certificate for the hostnames that you provide.

To install ``mkcert`` follow the instructions provided [here](https://github.com/FiloSottile/mkcert)

To develop locally run the following commands on a terminal:
```shell
mkcert -install
```
This will install a trusted CA to the system.

Change directory to the listr-backend crate and run the following:
```shell
mkcert localhost 127.0.0.1
```
This will generate 2 files:
* localhost+1-key.pem
* localhist+1.pem

These are the key and certificate generated by ``mkcert`` for the hosts ``localhost`` and ``127.0.0.1``.
The server will load these in its SSL configuration which will enable it to serve using HTTPS.

# Database
The database used for listr is PostgreSQL. The schema can be observed by looking at
the schema.rs file of the crate.

The ORM crate used to handle connection & queries is Diesel.