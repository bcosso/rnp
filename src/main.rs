use rsocket_rust::{prelude::*, Error};
use std::{
    fs::read,
    io::{stdin, stdout, Write},
};

mod configs;
mod connection_manager;

#[tokio::main]
async fn main() {
    let peers = configs::read_config_file().unwrap();
    let conn = connection_manager::create_instance(peers[0].clone())
        .await
        .unwrap();

    while true {
        let mut command = String::new();
        print!("nimpha> ");
        let _ = stdout().flush();
        stdin().read_line(&mut command);
        print!("{}", command);
        let args = command
            .replace("\n", "")
            .split(" ")
            .map(|s| s.to_string())
            .collect();
        parse(args, command, &conn, peers[0].clone()).await;
    }
}

async fn parse(
    tokens: Vec<String>,
    query: String,
    conn: &connection_manager::ConnManager,
    peer: configs::Peer,
) {
    if tokens[0] == "command" {
        print!("\nparse");
        executeCommand(tokens, peer, conn).await;
    }
    //for token in tokens{
    //    print!("{}\n", token);
    //}
}

fn executeQuery(query: String, peer: configs::Peer, conn: &connection_manager::ConnManager) {}

async fn executeCommand(
    tokens: Vec<String>,
    peer: configs::Peer,
    conn: &connection_manager::ConnManager,
) {
    match tokens[1].as_str() {
        "select" => executeSelect(tokens, peer, conn).await,
        _ => (),
    }
}

async fn executeSelect(
    tokens: Vec<String>,
    peer: configs::Peer,
    conn: &connection_manager::ConnManager,
) {
    let table = tokens[2].clone();
    let fieldkey = tokens[3].clone();
    let fieldvalue = tokens[4].clone();
    let payload = format!(
        r#"{{"table": "{}", "where_field": "{}", "where_content": "{}" }}"#,
        table, fieldkey, fieldvalue
    );

    print!("\n{}\n", payload);

    execute_in_cluster(
        "select_data_where_worker_equals_rsocket",
        payload.as_str(),
        peer,
        conn,
    )
    .await;
}

async fn execute_in_cluster(
    name_method: &str,
    data_json: &str,
    peer: configs::Peer,
    conn: &connection_manager::ConnManager,
) -> Result<String, Error> {
    let name_peer = peer.name;
    let port_peer = peer.port;
    let host_peer = peer.ip;

    // let cli = RSocketFactory::connect()
    //     .transport(TcpClientTransport::from(host_server))
    //     .setup(Payload::from("READY!"))
    //     .mime_type("text/plain", "text/plain")
    //     .on_close(Box::new(|| println!("connection closed")))
    //     .start()
    //     .await?;
    //let conn = connections.lock().unwrap();
    if let Some(cli) = conn.connections.get(&name_peer) {
        let method = "{\"method\":\"execute_something\"}";
        let data = format!("{{\"method\":\"/{name_peer}/{name_method}\",\"payload\":{data_json}}}");
        print!("{}", data);
        let req = Payload::builder()
            .set_data_utf8(&data)
            .set_metadata_utf8(method)
            .build();
        let res = cli.request_response(req).await?;
        println!("got: {:?}", res);
        let result2 = res.unwrap();
        let result1 = result2.data_utf8().unwrap();
        Ok(String::from(result1))
    } else {
        Ok("No connection found".to_string())
    }
}
