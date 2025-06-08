use ktorrent_rs::KTorrent;
use ktorrent_rs::torrent::TorrentStat;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let kt = KTorrent::new().await;
    assert!(kt.is_ok());

    let kt = kt.unwrap();
    let ts = kt.list_torrent_names().await.unwrap();

    println!("len: {}", ts.len());
    for t in ts {
        let tp = kt.get_torrent_proxy(t.as_str()).await.unwrap();
        let n = tp.name().await.unwrap();
        let s = tp.stats().await.unwrap();
        let st: TorrentStat = serde_bencode::from_bytes(&s).unwrap();
        if st.session_bytes_downloaded > 0 || st.session_bytes_uploaded > 0 {
            println!(
                "{t} - {n}, up: {}, dw: {}",
                st.session_bytes_uploaded, st.session_bytes_downloaded
            );
        }
    }

    println!("{:?}", start.elapsed());
}
