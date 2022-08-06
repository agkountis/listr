# Listr
[![CI](https://github.com/agkountis/listr/actions/workflows/ci.yml/badge.svg)](https://github.com/agkountis/listr/actions/workflows/ci.yml)

A list management full stack web app.

This pet project was spawned from experimentation with enterprise tech stacks.

The experiment that spawned this project can be found [here](https://github.com/agkountis/enterprise-stack-experiments/tree/main/rust/experiment-6-list-app).

Tech stack:
* Rust
* Yew/WASM
* AWS Cognito/JWT
* ActixWeb
* Diesel
* PostgreSQL

# Project Structure
The project consists of 3 crates:
* [listr-backend](/listr-backend)
* [listr-common](/listr-common)
* [listr-frontend](/listr-frontend)

## Backend
The backend crate is the app's web server. 

It is responsible for serving the frontend as a
single page application as well as provide the necessary APIs for list management as well as
retrieving & verifying oauth2 authentication tokens.

## Frontend
The front end crate is a web app creating with the Yew framework. It is a single page application 
that provides basic functionality to manage lists.

It has a minimalistic interface at the moment with no styling. There is no good CSS framework integration with Yew at this point in time.
However, experiments has been done to use [TailwindCSS](https://tailwindcss.com/) as well as [Bootstrap5](https://getbootstrap.com/) with Yew.

The experiments can be found here:
* [TailwindCSS + Yew](https://github.com/agkountis/enterprise-stack-experiments/tree/main/rust/experiment-5-yew-tailwindcss)
* [Bootstrap5 + Yew](https://github.com/agkountis/enterprise-stack-experiments/tree/main/rust/experiment-3-yew-bootstrap5-css)

## Common
The common crate simply contains common code that is used by both the frontend and the backend. Primarily request/response data structures.

# Development
To develop on listr you need to do some setup on you local machine for the frontend the backend.

## Install Rust
If you haven't already have Rust installed on your development machine please [do so](https://www.rust-lang.org/tools/install)!

## Frontend
With Rust installed on your development machine do the following:

### Add the WASM Compilation Target
Run the following command on the terminal:
```
rustup target add wasm32-unknown-unknown
```

### Install Trunk
To build and serve the Yew application we need to first install **Trunk**.
Run the following command on the terminal:
```
cargo install trunk
```

### Build and Run the App using Trunk
Run the following command on the terminal:
```
trunk serve --open
```
This will instruct trunk to build the web app and launch it on your default browser.

Trunk serves the app at localhost:8080.

> **NOTE:** As will be explained in the Backend section, we use ``trunk serve`` just for its watching capabilities (rebuilding the app on file change). 
> The backend is the one serving the front end App using HTTPS at **localhost:80**

To simply build the app and not serve it, use the following command:
```shell
trunk build
```

Both ``trunk serve`` and ``trunk build`` will compile the app to wasm and bundle it in a ``dist`` directory created in 
the crate's directory.

## Backend

// TODO

### Serving the Frontend
The backend also serves the front end app. To do that it needs access to the build output of trunk from the frontend crate.
To achieve this we need to create a symbolic link to the dist build directory of the front end crate.

For Linux:
```shell
ln -s listr-frontend/dist listr-backend/dist
```

For Windows open an **Administrator** command prompt and run the following:
```shell
mklink /J listr-backend\dist listr-frontend\dist
```

Now a symlink should be created and the backend can "see" the build output of the frontend and serve it at https://localhost:80.
