use ktorrent_rs::KTorrent;

#[tokio::main]
async fn main() {
    let rkt = KTorrent::new().await;
    assert!(rkt.is_ok());

    let kt = rkt.unwrap();
    let s = kt.get_settings_proxy().await.unwrap();

    let cpu = s.cpu_usage().await.unwrap();
    println!("cpu: {:?}", cpu);

    let d = s.last_save_dir().await.unwrap();
    println!("last_save_dir: {:?}", d);
}
