use bollard::{
    Docker,
    plugin::ContainerCreateBody,
    query_parameters::{
        CreateContainerOptionsBuilder, CreateImageOptionsBuilder, SearchImagesOptionsBuilder,
        StopContainerOptionsBuilder,
    },
};
use futures_util::StreamExt;
use std::{collections::HashMap, env, fs, vec};

use crate::docker::client;

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
    println!("starting creating the Container");

    let cwd: std::path::PathBuf = env::current_dir().unwrap();
    let service_file: std::path::PathBuf = cwd.join("./service-dckrnes.toml");
    let docker = Docker::connect_with_local_defaults()?;

    if service_file.exists() {
        println!("Service file found, try parsing");
        let config_str: String = fs::read_to_string(service_file).expect("Failed to read file");
        let config: config::Dockernes = toml::from_str(&config_str).expect("Failed to parse TOML");
        let ctnr_name: String = config.service.name;
        let image_name: String = config.service.image;
        let env_var: Vec<String> = config.service.environement;
        let _replicas: u32 = config.service.replica;

        if !env_var.is_empty() {
            println!("Environement variable detected")
        }

        match docker.inspect_container(&ctnr_name, None).await {
            Ok(info) => {
                let is_running = info.state.as_ref().and_then(|s| s.running).unwrap_or(false);

                if is_running {
                    println!("Container '{}' already exist and running", ctnr_name);
                    return Ok(());
                }

                println!("Starting container '{}'", ctnr_name);
                docker.start_container(&ctnr_name, None).await?;
                println!("Container started");
                return Ok(());
            }
            Err(e) => {
                let msg = e.to_string().to_lowercase();

                if msg.contains("not found") || msg.contains("no such container") {
                    println!("Container '{}' does not exist", ctnr_name);
                } else {
                    return Err(Box::new(e));
                }
            }
        }

        let mut filters = HashMap::new();
        filters.insert("until", vec!["10m"]);

        let search_options = SearchImagesOptionsBuilder::default()
            .term(&image_name)
            .filters(&filters)
            .build();

        if docker.search_images(search_options).await.is_ok() {
            println!("Image found !")
        } else {
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

            let ctnr = CreateContainerOptionsBuilder::default()
                .name(&ctnr_name)
                .build();

            let ctnr_config = ContainerCreateBody {
                image: Some(image_name),
                env: Some(env_var),
                ..Default::default()
            };

            docker.create_container(Some(ctnr), ctnr_config).await?;
            docker.start_container(&ctnr_name, None).await?;
            print!("Container created and started")
        }
    }
    Ok(())
}

async fn stop_container() -> Result<(), Box<dyn std::error::Error>> {
    println!("Stoping container...");

    let cwd: std::path::PathBuf = env::current_dir().unwrap();
    let service_file: std::path::PathBuf = cwd.join("./service-dckrnes.toml");
    let docker = Docker::connect_with_local_defaults()?;

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
