use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Dockernes {
    pub service: Service,
}

#[derive(Debug, Deserialize)]
pub struct Service {
    pub name: String,
    pub image: String,
    pub replica: u32,
    pub environement: Vec<String>,
}