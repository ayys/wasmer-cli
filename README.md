# WASMIE

# NOTE: Most commands are not implemented yet. This is a very bare-bones cli. Please do not use it right now.

- [Introduction](#introduction)
- [Features](#features)
- [Progress](#progress)
- [Progress so far](#progress-so-far)
- [CLI design](#cli-design)
    + [registry](#registry)
      - [wasmie registry add](#wasmie-registry-add)
      - [wasmie registry remove](#wasmie-registry-remove)
      - [wasmie registry search](#wasmie-registry-search)
      - [wasmie registry validate](#wasmie-registry-validate)
      - [wasmie registry login](#wasmie-registry-login)
      - [wasmie registry logout](#wasmie-registry-logout)
      - [wasmie registry packages](#wasmie-registry-packages)
      - [wasmie registry new-token](#wasmie-registry-new-token)
      - [wasmie registry invalidate-token](#wasmie-registry-invalidate-token)
      - [wasmie registry list-tokens](#wasmie-registry-list-tokens)
      - [wasmie registry active](#wasmie-registry-active)
      - [wasmie registry use](#wasmie-registry-use)
      - [wasmie registry namespaces](#wasmie-registry-namespaces)
      - [wasmie registry upgrade](#wasmie-registry-upgrade)
      - [wasmie registry list](#wasmie-registry-list)
    + [package](#package)
- [Wasmie config files](#wasmie-config-files)
    + [registries.toml - list of registries](#registriestoml---list-of-registries)
    + [packages.toml - list of installed packages](#packagestoml---list-of-installed-packages)
    + [tokens.toml - list of tokens for a user](#tokenstoml---list-of-tokens-for-a-user)
    + [wasmie.toml](#wasmietoml)
- [Contributing](#contributing)
- [License](#license)

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

| Command           | Description                                                                            |
|-------------------|----------------------------------------------------------------------------------------|
| add               | Add a new registry to to the local config file                                         |
| remove            | Remove a registry from the local config file                                           |
| search            | search for packages in the current registry                                            |
| validate          | Verify that all the registries in the config file are valid wasmer registries          |
| login             | Login to the current registry                                                          |
| logout            | Logout of the current registry                                                         |
| packages          | List all installed packages from the active registry                                   |
| new-token         | Get a new token for the active registry                                                |
| inavalidate-token | Invalidate a token from the active registry                                            |
| list-tokens       | List all the tokens for the active registry                                            |
| active            | Get the current registry                                                               |
| use               | Set a registry to be the active registry                                               |
| namespaces        | List all namespaces on the current registry (this includes both namespaces + username) |
| upgrade           | Update one or all packages from the active registry                                    |
| list              | List all known registries                                                              |
| config-file       | Print the registry config file to stdout                                               |

#### wasmie registry add

This command adds a new registry to Wasmie CLI. The user provides the name, endpoint, and description, and the command validates the endpoint. If successful, it adds the new registry to the list and returns a success message. The --non-interactive flag reminds the user to switch to interactive mode or provide an endpoint URL.

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

#### wasmie registry remove

This command removes a registry from Wasmie CLI. The user is prompted to choose which registry to remove, and the command warns them that packages from that registry cannot be updated anymore. The user is then asked to confirm before the registry is removed.

Alternatively, the user can specify the registry name and use the --non-interactive flag to bypass the confirmation prompt. If the user does not provide a name, the command reminds them to do so and suggests using wasmie registry list to view all existing registries.

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

#### wasmie registry search
This command searches for packages in the Wasmie CLI registry that match a partial package name provided by the user. The command returns a list of packages that match the search query, along with a brief description of each package, including the package name and version number.
```bash
$ wasmie registry search <partial_package_name>
Found 10 packages matching your search on registry `wapm-dev`

wasmer/ls@1.2.0
  Port of the ls command to wasm

unicode-prettytable@0.3.1
  Table formatting library using Unicode Box-drawing characters
```

#### wasmie registry validate
The wasmie registry validate command validates the registries configured in the local environment. It verifies whether the registry is a valid wasmer registry or not. If all registries are valid, it returns a success message. If any registry is invalid, it returns a list of invalid registries.
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

#### wasmie registry login
These commands are used to login to a wasmer registry using an access token. The first command prompts the user to enter their token, while the second command allows the token to be provided as an argument. The third command informs the user that the provided token is not valid. The fourth command sets the token as an environment variable and the login command uses it for authentication. The fifth command specifies a new token as an argument, but the command still uses the token from the environment variable.

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

#### wasmie registry logout
This command logs out the user from the active registry.
```bash
$ wasmie registry logout

You've logged out of your account (ayys) on registry `wapm-dev`
```

#### wasmie registry packages
The command wasmie registry packages lists the downloaded packages from the active registry, including package name, version, and description.
```bash
$ wasmie registry packages
List of downloaded packages from registry `wapm-dev`

wasmer/ls@1.2.3
	Run the ls command on the server

aysjha/test1@1.2034.3
	Test program written by aysjha
```

#### wasmie registry new-token

```bash
$ wasmie registry new-token
2983ue9823h29rnf9h92r92hr92hr2

$ wasmie registry new-token
Please login to the registry to generate a new token.
```

#### wasmie registry invalidate-token

```bash
$ wasmie registry invalidate-token <token>

Are you sure you want to invalidate the token? [Y/n]

The token has been invalidated.

$ wasmie registry invalidate-token <token> -y

The token has been invalidated.
```

#### wasmie registry list-tokens

```bash
$ wasmie registry list-tokens

List of active tokens on registry `wapm-dev`
meow-1
	a8suyd98asy9dashd9adh9ahd

meow-2
	laksjdhoashd98asdas98dha9
```

#### wasmie registry active

```bash
$ wasmie registry active
wapm-dev
```

#### wasmie registry use

```bash
$ wasmie registry use <registry>
Active registry set to `<registry>`. You're automatically logged in as aysjha
```

#### wasmie registry namespaces

```bash
$ wasmie registry namespaces

namespace-example
	description
	5 packages, 2 apps

namespace-two
	description
	5 packages, 1 app
```

#### wasmie registry upgrade

```bash
$ wasmie registry upgrade

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

#### wasmie registry list

```bash
$ wasmie registry list

wapm-dev
	endpoint:     https://registry.wapm.dev/graphql
	frontend-url: https://wapm.dev
	description:  The wasmer dev registry
```

### package
| Command | Description                                         |
|---------|-----------------------------------------------------|
| clone   | Clone a package into the current directory          |
| fork    | Fork a package on the registry                      |
| publish | Publish the package at path to the current registry |
| run     | Run a package from the registry                     |
| install | Globally install a package from the registry        |


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


## Contributing

Contributions to Wasmie are welcome and appreciated! Please see the contribution guidelines for more information on how to get started.
Wasmie is a third party CLI for wasmer.

## License

Wasmie is licensed under the [GPL 3.0 License](https://github.com/ayys/wasmie/blob/master/LICENSE).
