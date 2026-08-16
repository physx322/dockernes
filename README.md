
# Dockernes

A small container orchestrator without k8s cluster

![CodeFactor Grade (with branch)](https://img.shields.io/codefactor/grade/github/physx322/dockernes/main)
![GitHub top language](https://img.shields.io/github/languages/top/physx322/dockernes)


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

