# wasmer-cli

A demo CLI for wasmer


## wasmie
- [ ] List existing registries
  Read the config file and spitting out all the registries in there
- [ ] Add a new registry
  verify that it's a valid wapm registry. Then add a new entry to the config file
- [ ] Remove a registry
  Remove the lines for that registry from the config file
- [ ] Login to a registry
  Call the graphql API to login, get the token and store it in the config file.
- [ ] Logout of a registry
  Remove the token from the config file
- [ ] Search for a package
  Search for a package in the current registry via the graphql API.
