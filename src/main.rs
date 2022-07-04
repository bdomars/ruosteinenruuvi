use tokio::{sync::broadcast};

#[tokio::main]
async fn main() -> ruuvi::Result<()>{
    let (tx, mut rx) = broadcast::channel(16);

    tokio::spawn(ruuvi::scan_btle(tx));

    loop {
        let rm = rx.recv().await;
        println!("Got a ruuvimessage:");
        println!("{:#?}", rm);
    }
}
