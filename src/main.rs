use bollard::{
    Docker,
    plugin::ContainerCreateBody,
    query_parameters::{
        CreateContainerOptionsBuilder, CreateImageOptionsBuilder, SearchImagesOptionsBuilder,
    },
};
use futures_util::StreamExt;
use std::{collections::HashMap, env, fs};

mod config;

#[tokio::main]
async fn main() {
    let mut args: pico_args::Arguments = pico_args::Arguments::from_env();

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
        _ => println!("Unknown command"),
    }
}

async fn start_container() -> Result<(), Box<dyn std::error::Error>> {
    println!("starting creating the Container");

    let cwd: std::path::PathBuf = env::current_dir().unwrap();
    let service_file: std::path::PathBuf = cwd.join("./service-dckrnes.toml");
    let docker = Docker::connect_with_local_defaults()?;

    if service_file.exists() {
        println!("Service file found, try parsing...");

        let config_str: String = fs::read_to_string(service_file).expect("Failed to read file");
        let config: config::Dockernes = toml::from_str(&config_str).expect("Failed to parse TOML");

        let ctnr_name: String = config.service.name;
        let image_name: String = config.service.image;
        let _replicas: u32 = config.service.replica;

        if docker.inspect_container(&ctnr_name, None).await.is_ok() {
            println!("Container already exist")
        } else {
            let option = CreateImageOptionsBuilder::default()
                .from_image(&image_name)
                .build();

            let mut filters = HashMap::new();
            filters.insert("until", vec!["10m"]);

            let search_options = SearchImagesOptionsBuilder::default()
                .term(&image_name)
                .filters(&filters)
                .build();

            if docker.search_images(search_options).await.is_ok() {
                println!("Image found !");

                let image_created = docker
                    .create_image(Some(option), None, None)
                    .collect::<Vec<_>>()
                    .await;

                println!("Image pulled successfully")
            } else {
                println!("Image not found, check your service file.")
            }

            let options = CreateContainerOptionsBuilder::default()
                .name(&ctnr_name)
                .build();

            let config = ContainerCreateBody {
                image: Some(()),
                ..Default::default()
            };

            docker.start_container(&ctnr_name, options);
        }
    } else {
        println!("Service file not found at {:?}", service_file)
    }
    Ok(())
}
