# Dockernes

A small container orchestrator like docker-compose

![CodeFactor Grade (with branch)](https://img.shields.io/codefactor/grade/github/physx322/dockernes/main)
![GitHub top language](https://img.shields.io/github/languages/top/physx322/dockernes)


> [!CAUTION]
> Do not use dockernes in production, the software is currently in development and is not yet ready

## Roadmap

- [X] Detecting "service-dockrnes.toml" (like docker-compose.yml).
- [X] Supporting environement variables.
- [X] Managing your container.
- [X] Mount volume.
- [ ] Port forwarging.
- [ ] Protecting from pulling vulnerable image.
- [ ] Multi service deployement.
- [ ] API.

## Usage/Examples
Create a file named `service-dckrnes.toml`

```toml
[service]
name = "PostgresSQL"
image = "postgres:latest"
environement = [
    "POSTGRES_PASSWORD=password",
    "POSTGRES_USER=postgres"
    ]
volumes = [
   "/home/physx/posgres:/app/data"
]

```
| Field     | Description                                  |
|-----------|----------------------------------------------|
| `name`    | Name of the service/container                |
| `image`   | Docker image to use                          |
| `envrionement` | Environement variable for your service  |
| `volumes` | Mount volume docker in your computer         |

### Usage
For starting container
```bash
dckrnes run
```
For stopping the container
```bash
dckrnes stop
```

> More commands and options will be documented as features are implemented.
 
## Contributing
 
Contributions, issues, and feature requests are welcome. Feel free to check the [issues page](https://github.com/physx322/dockernes/issues) if you want to contribute.
 
## License
 
No license has been specified yet for this project.


## Authors

- [@PhysX](https://www.github.com/physx322)
