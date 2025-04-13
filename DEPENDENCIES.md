## Project Structure

### Crate Dependencies

```mermaid
graph TD
    domain[domain]
    command[command]
    query[query]
    infrastructure[infrastructure]
    api[api]
    main[main]

    domain --> domain
    command --> domain
    query --> domain
    infrastructure --> domain
    api --> domain
    api --> command
    api --> query
    main --> domain
    main --> command
    main --> query
    main --> infrastructure
    main --> api
```

The diagram shows the dependency relationships between the crates:
- **domain**: Core business logic and interfaces
- **command**: Command handling functionality
- **query**: Query handling functionality
- **infrastructure**: Concrete implementations of domain interfaces
- **api**: HTTP endpoints exposing the application
- **main**: Entry point that wires everything together

