use std::{env, fs};
use bollard::{Docker, query_parameters::{CreateImageOptionsBuilder}};
use futures_util::StreamExt;

mod config;
fn main() {
    let mut args: pico_args::Arguments = pico_args::Arguments::from_env();
    let command: String = args.free_from_str().unwrap();

    match command.as_str() {
        "run" => { 
            start_container();
        }
        _ => println!("Uknown command")
    }
}

async fn start_container() -> Result<(), Box<dyn std::error::Error>> {
    println!("starting creating the Container");
    // detect current directory et search for service_dckrnes.toml file
    let cwd: std::path::PathBuf = env::current_dir().unwrap();
    let service_file: std::path::PathBuf = cwd.join("service_dckrnes.toml");
    let docker = Docker::connect_with_local_defaults()?;


    // if service_file exist, try parsing them, and extract value to put on a variables.
    if service_file.exists() {
        println!("Service file found, try parsing");
        let config_str: String = fs::read_to_string(service_file).expect("Failed to read file");
        let config: config::Dockernes = toml::from_str(&config_str).expect("Failed to parse TOML");
        let ctnr_name: String = config.service.name;
        let image_name: String = config.service.image;
        let replicas: u32 = config.service.replica;
        
        if docker.inspect_container(&ctnr_name, None).await.is_ok() {
            println!("Container already exist")
        } else {
            let option = CreateImageOptionsBuilder::default().from_image(&image_name).build();
            docker.create_image(Some(option), None, None)
            .collect::<Vec<_>>()
            .await;
        }
    }
    Ok(())
}

