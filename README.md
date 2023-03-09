# WASMIE

# NOTE: Most commands are not implemented yet. This is a very bare-bones cli. Please do not use it right now.

## Introduction
Wasmie is a third-party command-line interface (CLI) tool for Wasmer, a WebAssembly runtime that enables developers to run WebAssembly code outside of the web browser. The purpose of Wasmie is to provide a more user-friendly interface to interact with Wasmer and to streamline common tasks.

## Features

Wasmie provides several subcommands for interacting with Wasmer, including:

- `registry`: Commands for managing packages in the Wasmer registry, such as listing available packages, adding new packages, and upgrading existing packages.
- `package`: Commands for working with local WebAssembly packages, such as publishing packages, running them locally, and cloning/forking existing packages.

## Progress

The following tasks are currently being worked on for Wasmie:

- **Registry subcommand**: Implementing the logic functions for the remaining commands in the registry subcommand.
- **Package subcommand**: Implementing the logic functions for the commands in the package subcommand.
- **CI** integration: Implementing continuous integration (CI) to publish the package for all platforms.
- **Testing**: Writing tests for each command and logic functions to ensure the stability and reliability of the CLI.


## Contributing

Contributions to Wasmie are welcome and appreciated! Please see the contribution guidelines for more information on how to get started.
Wasmie is a third party CLI for wasmer.

## License

Wasmie is licensed under the [GPL 3.0 License](https://github.com/ayys/wasmie/blob/master/LICENSE).


## Progress so far

- [ ] Add CI to publish package for all platforms
- [ ] Write tests for each command
- [ ] Write tests for logic functions
- [ ] Implement logic functions for registry subcommand
  - [x] `active` Get the current registry
  - [ ] `use` Set a registry to be the active registry
  - [ ] `add` Add a new registry to to the local config file
  - [ ] `remove` Remove a registry from the local config file
  - [ ] `search` search for packages in the active registry
  - [ ] `validate` Verify that all the registries in the config file are valid wasmer registries
  - [ ] `login` Login to the active registry
  - [ ] `logout` Logout of the active registry
  - [ ] `packages` List all installed packages from the active registry
  - [ ] `new-token` Get a new token for the active registry
  - [ ] `list-tokens` List all the tokens for the active registry
  - [ ] `invalidate-token` Invalidate a token from the active registry
  - [x] `config-file` Print the registry config file to stdout
  - [x] `list` List all known registries
  - [ ] `namespaces` List all namespaces on the current registry (this includes both namespaces + username)
  - [ ] `upgrade` Update one or all packages from the active registry
- [ ] Implement logic functions for package subcommand
  - [ ] `clone` Clone a package into the current directory
  - [ ] `fork` Fork a package on the registry
  - [ ] `publish` Publish the package at path to the current registry
  - [ ] `run` Run a package from the registry
  - [ ] `install` Globally install a package from the registry
