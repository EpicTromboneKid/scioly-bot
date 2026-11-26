use crate::utils::Error;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Server {
    pub server_id: String,
    pub server_name: String,
    pub server_email: String,
    pub roster_file_id: String,
    pub pc_file_id: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AllData {
    pub servers: Vec<Server>,
}

pub fn get_server_data(file_path: &str) -> Result<AllData, Error> {
    let data = std::fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read server data from {}: {}", file_path, e))?;
    let server: AllData = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse server data JSON: {}", e))?;
    Ok(server)
}

pub fn write_server_data(file_path: &str, to_write: AllData) -> Result<(), Error> {
    std::fs::write(file_path, serde_json::to_string(&to_write)?)?;
    Ok(())
}

pub fn get_server(server_id: &str) -> Result<Server, Error> {
    let servers = get_server_data("serverdata.json")?;
    for server in servers.servers {
        if server.server_id == server_id {
            return Ok(server);
        }
    }
    Err("Server not found".into())
}
