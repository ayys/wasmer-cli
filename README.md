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
  - [x] `active`
  - [ ] `add`
  - [x] `config-file`
  - [ ] `invalidate-token`
  - [x] `list`
  - [ ] `list-tokens`
  - [ ] `login`
  - [ ] `logout`
  - [ ] `namespaces`
  - [ ] `new-token`
  - [ ] `packages`
  - [ ] `remove`
  - [ ] `search`
  - [ ] `upgrade`
  - [ ] `use`
  - [ ] `validate`
- [ ] Implement logic functions for package subcommand
  - [ ] `clone`
  - [ ] `fork`
  - [ ] `publish`
  - [ ] `run`
