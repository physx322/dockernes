use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Dockernes {
    pub service: Service,
}

#[derive(Debug, Deserialize)]
pub struct Service {
    pub name: String,
    pub image: String,
    pub environement: Vec<String>,
}
