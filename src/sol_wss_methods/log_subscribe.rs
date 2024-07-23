use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::Message::Text;
use tokio_tungstenite::WebSocketStream;

pub async fn send_request(
    stream: &mut WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    let request = r#"
    {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "logsSubscribe",
        "params": [
                        {
                        "mentions": [ "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P" ]
                        },
                        {
                        "commitment": "processed"
                        }
                ]
    }
    "#;

    stream.send(Text(request.into())).await
}
