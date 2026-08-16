# Dockernes

A small container orchestrator without k8s cluster

![CodeFactor Grade (with branch)](https://img.shields.io/codefactor/grade/github/physx322/dockernes/main)
![GitHub top language](https://img.shields.io/github/languages/top/physx322/dockernes)


> [!CAUTION]
> Do not use dockernes in production, the software is currently in development and is not yet ready

## Roadmap

- [X] Detecting "service-dockrnes.toml" (like docker-compose.yml).
- [X] Supporting environement variables.
- [X] Managing your container.
- [ ] Life cycle.
- [ ] Port forwarging.
- [ ] Proecting from pulling vulnerable image.
- [ ] Multi service deployement.
- [ ] API.

## Usage/Examples
Create a file named `service-dckrnes.toml`

```toml
[service]
name = "PostgresSQL"
image = "postgres:latest"
replica = 1
environement = [
    "POSTGRES_PASSWORD=password",
    "POSTGRES_USER=postgres"
    ]

```
| Field     | Description                                  |
|-----------|----------------------------------------------|
| `name`    | Name of the service/container                |
| `image`   | Docker image to use                          |
| `replica` | Number of container instances to run         |
| `envrionement` | Environement variable for your service  |

### Usage
```bash
dckrnes run
```

> More commands and options will be documented as features are implemented.
 
## Contributing
 
Contributions, issues, and feature requests are welcome. Feel free to check the [issues page](https://github.com/physx322/dockernes/issues) if you want to contribute.
 
## License
 
No license has been specified yet for this project.


## Authors

- [@PhysX](https://www.github.com/physx322)

