# The Exchange Rate Path Problem

### Dependencies

* [Cargo make](https://sagiegurari.github.io/cargo-make/):

    `cargo install --force cargo-make`
    
* [Clippy](https://github.com/rust-lang/rust-clippy/):

    `rustup component add clippy`

    _Note: `cargo make` will take care to install this when you run task containing it the `check-clippy` task._
* [Rustfmt](https://github.com/rust-lang/rustfmt)

    `rustup component add rustfmt`

    _Note: `cargo make` will take care to install this when you run task containing it the `check-format` or `format` tasks._

### Running the project

`cargo run`

### Running the tests

`cargo make test`

### Code checks

Apart from the tests, there are couple of checks that are running and they should pass in order
for the PRs to be passing the builds:

#### Format ([Rustfmt](https://github.com/rust-lang/rustfmt))
You can check your formatting by this command:

`cargo make check-format`

If you want to fix the formatting automatically, just drop the check:

`cargo make format`

#### [Clippy](https://github.com/rust-lang/rust-clippy/)

You can also run Cargo clippy, for now with one lint is disabled, until I fix it in a appropriate way.
To check the code, please run:

`cargo make check-clippy`

#### Additional make commands:

* Run all the checks + build the project and run the tests:

    * `cargo make check-quick` _Note: Does not clean the previous artifacts (doesn't run `cargo clean`)_
    * `cargo make check-all` _Note: Cleans the previous artifacts (runs `cargo clean`)_
* Clean build (cleans the previous artifacts and runs the tests)

    * `cargo make clean-build` _Note (runs `cargo clean`)_

* Clean the previous artifacts (runs `cargo clean` underneath)

    * `cargo make clean`
    
    
## Docker image

#### Build docker image:

`docker build . -t elpiel/circle-ci-rust`

#### Push docker image:

You can push images to [docker hub](http://hub.docker.com/):

`docker push elpiel/circle-ci-rust:{TAG}`

The Main example being the latest tag:

`docker push elpiel/circle-ci-rust:latest`
