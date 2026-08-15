
# Dockernes

A small container orchestrator without k8s cluster

[![CodeFactor](https://www.codefactor.io/repository/github/physx322/dockernes/badge/main)](https://www.codefactor.io/repository/github/physx322/dockernes/overview/main)

> [!CAUTION]
> Do not use dockernes in production, the software is currently in development and is not yet ready

## Roadmap

- [X] Detecting "service-dockrnes.toml" (like docker-compose.yml)
- [ ] Managing your container
- [ ] Life cycle
- [ ] Port forwarging
- [ ] Proecting from pulling vulnerable image
- [ ] API

## Usage/Examples
Create a file named `service-dckrnes.toml`
```toml
[service]
name = "nginx"
image = "physicsh/ptero-panel-hard:latest"
replica = 1
```
## Authors

- [@PhysX](https://www.github.com/physx322)

