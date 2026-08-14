use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Authoritative configuration for a virtual machine / runtime instance.
/// This type replaces the stringly-typed spec_json and breaks the dependency
/// cycle between plaza-workspace and plaza-runtime.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MachineConfig {
    pub image_ref: String,
    pub memory_mb: u64,
    pub cpu_cores: u32,
    pub environment: HashMap<String, String>,
    pub network: NetworkConfig,
    pub storage: Vec<StorageMount>,
}

impl Default for MachineConfig {
    fn default() -> Self {
        Self {
            image_ref: String::new(),
            memory_mb: 512,
            cpu_cores: 1,
            environment: HashMap::new(),
            network: NetworkConfig::default(),
            storage: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct NetworkConfig {
    pub disable_networking: bool,
    pub forwarded_ports: Vec<PortForward>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortForward {
    pub host_port: u16,
    pub guest_port: u16,
    pub protocol: Protocol,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum Protocol {
    #[default]
    Tcp,
    Udp,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StorageMount {
    pub source: String,
    pub destination: String,
    pub read_only: bool,
}
