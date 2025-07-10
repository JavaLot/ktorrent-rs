use ktorrent_rs::KTorrent;

#[tokio::main]
async fn main() {
    let rkt = KTorrent::new().await;
    assert!(rkt.is_ok());

    let kt = rkt.unwrap();
    let c = kt.get_core_proxy().await.unwrap();

    let s = c.suspended().await.unwrap();
    println!("suspended: {:?}", s);

    let gs = c.groups().await.unwrap();
    println!("groups: {:?}", gs);

    // let ts = c.torrents().await.unwrap();
    // println!("torrents: {:?}", ts);

    c.log("Example log message from Rust throw zbus")
        .await
        .unwrap();

    let nt = c.num_torrents_running().await.unwrap();
    println!("Torrents running: {:?}", nt);
}
