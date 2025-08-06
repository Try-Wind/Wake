use std::os::unix::net::UnixStream;
use std::path::Path;

use crate::fc::history::{CommandEntry, CommandHistory, HistoryStats};
use crate::fc::protocol::{WakeProtocol, WakeRequest, WakeResponse, ResponseData};

/// Client for querying the command history via Unix socket
pub struct WakeSessionClient {
    socket_path: String,
}

impl WakeSessionClient {
    pub fn new(session_id: &str) -> Self {
        let socket_path = format!("/tmp/wake_history_{}", session_id);
        Self { socket_path }
    }

    pub fn get_last_commands(&self, n: usize) -> Result<CommandHistory, Box<dyn std::error::Error>> {
        let mut stream = UnixStream::connect(&self.socket_path)
            .map_err(|_| "Could not connect to WAKE history session (is server running?)")?;
        
        let request = WakeRequest::GetLastCmd { n };
        WakeProtocol::write_request(&mut stream, &request)?;
        
        let response = WakeProtocol::read_response(&mut stream)?;
        
        match response {
            WakeResponse::Ok { data: ResponseData::Commands(entries) } => Ok(entries.into()),
            WakeResponse::Ok { .. } => Err("Unexpected response type".into()),
            WakeResponse::Error { message } => Err(message.into()),
        }
    }

    pub fn get_all_commands(&self) -> Result<CommandHistory, Box<dyn std::error::Error>> {
        let mut stream = UnixStream::connect(&self.socket_path)
            .map_err(|_| "Could not connect to WAKE history session (is server running?)")?;
        
        let request = WakeRequest::GetAllCmd;
        WakeProtocol::write_request(&mut stream, &request)?;
        
        let response = WakeProtocol::read_response(&mut stream)?;
        
        match response {
            WakeResponse::Ok { data: ResponseData::Commands(entries) } => Ok(entries.into()),
            WakeResponse::Ok { .. } => Err("Unexpected response type".into()),
            WakeResponse::Error { message } => Err(message.into()),
        }
    }

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stream = UnixStream::connect(&self.socket_path)
            .map_err(|_| "Could not connect to WAKE history session (is server running?)")?;
        
        let request = WakeRequest::Clear;
        WakeProtocol::write_request(&mut stream, &request)?;
        
        let response = WakeProtocol::read_response(&mut stream)?;
        
        match response {
            WakeResponse::Ok { .. } => Ok(()),
            WakeResponse::Error { message } => Err(message.into()),
        }
    }

    pub fn get_status(&self) -> Result<HistoryStats, Box<dyn std::error::Error>> {
        let mut stream = UnixStream::connect(&self.socket_path)
            .map_err(|_| "Could not connect to WAKE history session (is server running?)")?;
        
        let request = WakeRequest::Status;
        WakeProtocol::write_request(&mut stream, &request)?;
        
        let response = WakeProtocol::read_response(&mut stream)?;
        
        match response {
            WakeResponse::Ok { data: ResponseData::Stats(stats) } => Ok(stats),
            WakeResponse::Ok { .. } => Err("Unexpected response type".into()),
            WakeResponse::Error { message } => Err(message.into()),
        }
    }

    pub fn pre_command(&self, cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut stream = UnixStream::connect(&self.socket_path)
            .map_err(|_| "Could not connect to WAKE history session (is server running?)")?;
        
        let request = WakeRequest::PreCmd { cmd: cmd.to_string() };
        WakeProtocol::write_request(&mut stream, &request)?;
        
        let response = WakeProtocol::read_response(&mut stream)?;
        
        match response {
            WakeResponse::Ok { .. } => Ok(()),
            WakeResponse::Error { message } => Err(message.into()),
        }
    }

    pub fn post_command(&self, exit_code: i32,  cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut stream = UnixStream::connect(&self.socket_path)
            .map_err(|_| "Could not connect to WAKE history session (is server running?)")?;
        
        let request = WakeRequest::PostCmd { 
            cmd: cmd.to_string(), 
            exit_code
        };
        WakeProtocol::write_request(&mut stream, &request)?;
        
        let response = WakeProtocol::read_response(&mut stream)?;
        
        match response {
            WakeResponse::Ok { .. } => Ok(()),
            WakeResponse::Error { message } => Err(message.into()),
        }
    }

    pub fn session_exists(&self) -> bool {
        Path::new(&self.socket_path).exists()
    }
}

