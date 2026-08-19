use bollard::{
    Docker, plugin::ContainerCreateBody, query_parameters::{
        CreateContainerOptionsBuilder, CreateImageOptionsBuilder, StopContainerOptionsBuilder,
    },
};
use futures_util::{StreamExt};
use std::{env, fs};

use crate::docker::client;

#[derive(Debug, PartialEq)]
enum ContainerStatus {
    AlreadyRunning,
    Started,
    NotFound,
}

mod config;
mod docker;

#[tokio::main]
async fn main() {
    let mut args: pico_args::Arguments = pico_args::Arguments::from_env();
    let client = client::connect();
    if client.is_ok() {
        println!("Client successfuly connected")
    }

    let command: String = match args.free_from_str() {
        Ok(cmd) => cmd,
        Err(_) => {
            println!("Usage: program <command>");
            return;
        }
    };

    match command.as_str() {
        "run" => {
            if let Err(e) = start_container().await {
                eprintln!("Error running container: {}", e);
            }
        }

        "stop" => {
            if let Err(e) = stop_container().await {
                eprintln!("Error while stoping container: {}", e);
            }
        }
        _ => println!("Unknown command"),
    }
}

async fn start_container() -> Result<(), Box<dyn std::error::Error>> {
    let cwd: std::path::PathBuf = env::current_dir().unwrap();
    let service_file: std::path::PathBuf = cwd.join("./service-dckrnes.toml");
    let docker = Docker::connect_with_socket_defaults()?;

    if service_file.exists() {
        println!("Service file found, try parsing");
        let config_str: String = fs::read_to_string(service_file).expect("Failed to read file");
        let config: config::Dockernes = toml::from_str(&config_str).expect("Failed to parse TOML");
        let ctnr_name: String = config.service.name;
        let image_name: String = config.service.image;
        let env_var: Vec<String> = config.service.environement.unwrap_or_default();
        let volumes: Vec<String> = config.service.volumes.unwrap_or_default();

        if !env_var.is_empty() {
            println!("Environement variable detected")
        }

        if !volumes.is_empty() {
           println!("Volumes detected")
        }

        if docker.inspect_image(&image_name).await.is_err() {
            println!("Image not found locally, Pulling...");
            let options = CreateImageOptionsBuilder::default()
                .from_image(&image_name)
                .build();

            let mut docker_image = docker.create_image(Some(options), None, None);
            while let Some(result) = docker_image.next().await {
                match result {
                    Ok(_) => println!(),
                    Err(e) => return Err(Box::new(e)),
                }
            }
        }
        
        match check_container(&ctnr_name, &docker).await {
         Ok(ContainerStatus::NotFound) => {
            let ctnr = CreateContainerOptionsBuilder::default()
                  .name(&ctnr_name)
                  .build();
      
            let ctnr_config = ContainerCreateBody {
                  image: Some(image_name.clone()),
                  env: Some(env_var.clone()),
                  volumes: Some(volumes.clone()),
                  ..Default::default()
            };
      
            docker.create_container(Some(ctnr), ctnr_config).await?;
            docker.start_container(&ctnr_name, None).await?;
            println!("Container created and started");
         }
         Ok(ContainerStatus::AlreadyRunning) => println!("Container already running"),
         Ok(ContainerStatus::Started) => println!("Container started"),
         Err(e) => return Err(e),
      }
    }
    Ok(())
}

async fn stop_container() -> Result<(), Box<dyn std::error::Error>> {
    println!("Stoping container...");

    let cwd: std::path::PathBuf = env::current_dir().unwrap();
    let service_file: std::path::PathBuf = cwd.join("./service-dckrnes.toml");
    let docker = Docker::connect_with_socket_defaults()?;

    if service_file.exists() {
        println!("Service file found, try parsing");
        let config_str: String = fs::read_to_string(service_file).expect("Failed to read file");
        let config: config::Dockernes = toml::from_str(&config_str).expect("Failed to parse TOML");
        let ctnr_name: String = config.service.name;

        let options = StopContainerOptionsBuilder::default().t(30).build();

        docker.stop_container(&ctnr_name, Some(options)).await?;
        println!("Container stopped")
    } else {
        println!("Service file not found")
    }
    Ok(())
}


async fn check_container(name: &String, docker: &Docker) -> Result<ContainerStatus, Box<dyn std::error::Error>> {
    match docker.inspect_container(name, None).await {
        Ok(info) => {
            let is_running = info.state.as_ref().and_then(|s| s.running).unwrap_or(false);
            if is_running {
                return Ok(ContainerStatus::AlreadyRunning);
            }
            println!("Starting container '{}'", name);
            docker.start_container(name, None).await?;
            println!("Container started");
            Ok(ContainerStatus::Started)
        }
        Err(e) => {
            let msg = e.to_string().to_lowercase();
            if msg.contains("not found") || msg.contains("no such container") {
                println!("Container '{}' does not exist", name);
                Ok(ContainerStatus::NotFound)
            } else {
                Err(Box::new(e))
            }
        }
    }
}
