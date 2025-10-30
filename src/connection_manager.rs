use crate::configs;
use crate::configs::Peer;
use rsocket_rust::prelude::*;
use rsocket_rust::Client;
use rsocket_rust::Result;
use rsocket_rust_transport_tcp::TcpClientTransport;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

pub struct ConnManager {
    pub connections: HashMap<String, Client>,
}

impl ConnManager {
    pub fn new() -> Self {
        ConnManager {
            connections: HashMap::new(),
        }
    }

    pub async fn init_connections(&mut self, peers: Peer) -> Result<String> {
        self.create_client(peers).await?;

        Ok(String::from("Ok"))
    }

    pub async fn create_client(&mut self, peer: configs::Peer) -> Result<String> {
        let name_peer = peer.name;
        let port_peer = peer.port;
        let host_peer = peer.ip;
        let host_server = format!("{host_peer}:{port_peer}");

        let cli = RSocketFactory::connect()
            .transport(TcpClientTransport::from(host_server))
            .setup(Payload::from("READY!"))
            .mime_type("text/plain", "text/plain")
            .on_close(Box::new(|| println!("connection closed")))
            .start()
            .await?;
        self.connections.insert(name_peer.to_string(), cli);
        Ok(String::from("Ok"))

        //add cli to the list
    }
}

pub async fn create_instance(peer: Peer) -> Result<ConnManager> {
    let mut conn = ConnManager::new();

    conn.init_connections(peer).await;

    Ok(conn)
}
