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


# CLI design

### registry

| add | Add a new registry to to the local config file |
| --- | --- |
| remove | Remove a registry from the local config file |
| search | search for packages in the current registry |
| validate | Verify that all the registries in the config file are valid wasmer registries |
| login | Login to the current registry |
| logout | Logout of the current registry |
| packages | List all installed packages from the active registry |
| new-token | Get a new token for the active registry |
| inavalidate-token | Invalidate a token from the active registry |
| list-tokens | List all the tokens for the active registry |
| active | Get the current registry |
| use | Set a registry to be the active registry |
| namespaces | List all namespaces on the current registry (this includes both namespaces + username) |
| upgrade | Update one or all packages from the active registry |
| list | List all known registries |
| config-file | Print the registry config file to stdout |

```bash
$ wasmie registry add
Name: registry-dev
Endpoint: https://registry.wapm.dev/graphql
  [x] validating endpoint...
  [x] endpoint is a valid wasmer registry.
Description [""]: wasmer dev registry

name: registry-dev
endpoint: https://registry.wapm.dev/graphql
description: wasmer dev registry

Is this correct? [Y/n] n

Name [registry-dev]: \n
Endpoint [https://registry.wapm.dev/graphql]: \n
Description ["wasmer dev registry"]: Wasmer dev registry

name: registry-dev
endpoint: https://registry.wapm.dev/graphql
description: wasmer dev registry

is this correct? [Y/n] Y

[x] The new registry has been successfully saved

$ wasmie registry add --name "registry-dev" \
  --endpoint "https://registry.wapm.dev/graphql" \
  --description "Wasmer dev registry"

  [x] validating endpoint...
  [x] endpoint is a valid wasmer registry.

[x] The new registry has been successfully saved

$ wasmie registry add --name "registry-dev" \
  --endpoint "https://registry.wapm.dev/graphql"
## if no description is provided, use default empty string

  [x] validating endpoint...
  [x] endpoint is a valid wasmer registry.

[x] The new registry has been successfully saved

$ wasmie registry add --name "registry-dev" \
  --description "Wasmer dev registry"
## Ask for missing fields if they are not specified.
Endpoint:

  [x] validating endpoint...
  [x] endpoint is a valid wasmer registry.

[x] The new registry has been successfully saved

$ wasmie registry add --name "registry-dev" \
  --description "Wasmer dev registry"
  --non-interactive

Ops! You forgot to specify the endpoint. Since you are in non-interactive mode,
either switch to interactive mode by removing the `--non-interactive` flag, or
pass the `--endpoint` flag with argument URL to a valid wasmer registry.
```

```bash
$ wasmie registry remove
[1] wapm-dev
[2] wapm-prod
Which registry do you want to remove? [1]
: 2

Warning: you cannot update packages from that registry anymore.
Are you sure you want to remove the `wapm-prod` registry? [y/N]

Registry `wapm-prod` has been removed from list of registries.

$ wasmie registry remove wapm-prod
Warning: you cannot update packages from that registry anymore.
Are you sure you want to remove the `wapm-prod` registry? [y/N]

Registry `wapm-prod` has been removed from list of registries.

$ wasmie registry remove wapm-prod --non-interactive

Warning: you cannot update packages from that registry anymore.
Registry `wapm-prod` has been removed from list of registries.

$ wasmie registry remove --non-interactive

Ops! You need to specify name of the registry to remove. Options are:
wapm-dev, wapm-prod.
Use the `wasmie registry list` to get more info about all the existing registies.
```

```bash
$ wasmie registry search <partial_package_name>
Found 10 packages matching your search on registry `wapm-dev`

wasmer/ls@1.2.0
  Port of the ls command to wasm

unicode-prettytable@0.3.1
  Table formatting library using Unicode Box-drawing characters
```

```bash
$ wasmie registry validate
Validating registry `wapm-dev`...
Registry `wapm-dev` is a valid registry ✅

Validating registry `wapm-prod`...
Registry `wapm-dev` is a valid registry ✅

Success! All the registries in your local config are valid.

$ wasmie registry validate
Validating registry `wapm-dev`...
Registry `wapm-dev` is a not a valid registry ❌

Validating registry `wapm-prod`...
Registry `wapm-dev` is a valid registry ✅

The following registries are not valid:
	- `wapm-dev`
```

```bash
$ wasmie registry login
Youre trying to login to the `wapm-dev` registry.
Enter your token: aosidh98asd9ahda9dh9ausdh9ahdad

Youve successfully logged in as aysjha

$ wasmie registry login <token>
Youve successfully logged in as aysjha

$ wasmie registry login <token>
The token you provided was not valid. Get a valid token and try again.

$ export WASMIE_LOGIN_TOKEN=<token>
$ wasie registry login
Youve successfully logged in as aysjha

$ export WASMIE_LOGIN_TOKEN=<token2>
$ wasmie registry login <token1> # uses token 1

You've successfully logged in as aysjha.
```

```bash
$ wasmie registry logout

You've logged out of your account (ayys) on registry `wapm-dev`
```

```bash
$ wasmie registry packages
List of downloaded packages from registry `wapm-dev`

wasmer/ls@1.2.3
	Run the ls command on the server

aysjha/test1@1.2034.3
	Test program written by aysjha
```

```bash
$ wasmie registry new-token
2983ue9823h29rnf9h92r92hr92hr2

$ wasmie registry new-token
Please login to the registry to generate a new token.
```

```bash
$ wasmie registry invalidate-token <token>

Are you sure you want to invalidate the token? [Y/n]

The token has been invalidated.

$ wasmie registry invalidate-token <token> -y

The token has been invalidated.
```

```bash
$ wasmie registry list-tokens

List of active tokens on registry `wapm-dev`
meow-1
	a8suyd98asy9dashd9adh9ahd

meow-2
	laksjdhoashd98asdas98dha9
```

```bash
$ wasmie registry get-active
wapm-dev
```

```bash
$ wasmie registry set-active <registry>
Active registry set to `<registry>`. You're automatically logged in as aysjha
```

```bash
$ wasmie registry namespaces

namespace-example
	description
	5 packages, 2 apps

namespace-two
	description
	5 packages, 1 app
```

```bash
$ wasmie registry update

The following packages need to be updated:
hello/ays
	description
1.2.3 -> 1.2.5

hello/meow
	descriptiokn
0.3.4 -> 45.6.7

Do you want to update them? [Y/n]

Updating packages [................]

Youve successfully updated all the packages in the registry.

$ wasmie registry update hello/meow

A new version of hello/meow is available.
Do you want to update to the latest version? [Y/n]

Updating packages [................]

Youve successfully updated hello/meow to version 0.1.3 in the registry.
```

```bash
$ wasmie registry list

wapm-dev
	endpoint:     https://registry.wapm.dev/graphql
	frontend-url: https://wapm.dev
	description:  The wasmer dev registry
```

### package

| clone | Clone a package into the current directory |
| --- | --- |
| fork | Fork a package on the registry |
| publish | Publish the package at path to the current registry |
| run | Run a package from the registry |
| install | Globally install a package from the registry |

## Wasmie config files

The wasmie config files will contain global configurations, that means the following:

### registries.toml - list of registries

```toml
[[registries]]
name = "wapm-dev"
description = "The wasmer dev registry. Not for production use."
endpoint = "https://registry.wapm.dev/graphql"

[[registries]]
name = "wapm-prod"
description = "The wasmer prod registry."
endpoint = "https://registry.wapm.io/graphql"
```

### packages.toml - list of installed packages

```toml
[[packages]]
name = "example/hello"
version = "1.2.3"
registry = "wapm-dev"
command = "hello"
installed-at = "817263871623871". # unix timestamp

[[packages]]
name = "wasmer/ls"
version = "0.1.1"
registry = "wapm-dev"
command = "ls"
installed-at = "817263871623871". # unix timestamp

```

### tokens.toml - list of tokens for a user

```toml
[[tokens]]
name = "sample-token"
user = "testuser"
registry = "wapm-prod"
token = "aoshud9asdh9ashdahas9hasdha9sd"

[[tokens]]
name = "sample-token2"
user = "testuser"
registry = "wapm-dev"
token = "iuashda9hs9adhas9udhaisdahud"

[[tokens]]
name = "sample-token"
user = "testuser"
registry = "wapm-dev"
token = "kasihdiuanyd98aydh9uahd"

```

wasmie will also have a local config file, called `wasmie.toml` which will store local dependencies for a project.

### wasmie.toml

```toml
[package]
name = "package-name"
version = "package-version"

[[registries]]
name = "wapm-dev"
description = "The wasmer dev registry. Not for production use."
endpoint = "https://registry.wapm.dev/graphql"

[dependencies]
helloworld = {version = "0.1.0", registry = "wapm-dev", name = "example/helloworld"}
```

```bash
$ wasmie registry list

wapm-dev: https://registry.wapm.dev
	This is the description

```
