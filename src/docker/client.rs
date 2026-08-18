use bollard::Docker;

pub fn connect() -> Result<Docker, bollard::errors::Error> {
   Docker::connect_with_socket_defaults()
}
