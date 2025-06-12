use ktorrent_rs::KTorrent;
use ktorrent_rs::torrent::{TorrentStats, UpDownStats};
use std::collections::HashSet;
use std::time::Instant;
use termion::style;

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let kt = KTorrent::new().await;
    assert!(kt.is_ok());

    let kt = kt.unwrap();
    if !kt.is_running().await.unwrap() {
        eprintln!("KTorrent is not running!");
        return;
    }

    let ts = kt.list_torrent_names().await.unwrap();

    let mut active: usize = 0;
    let mut statuses: HashSet<String> = HashSet::new();
    let mut stats = UpDownStats::new();
    for t in &ts {
        let tp = kt.get_torrent_proxy(t.as_str()).await.unwrap();
        let n = tp.name().await.unwrap();
        let s = tp.stats().await.unwrap();
        let st: TorrentStats = serde_bencode::from_bytes(&s).unwrap();
        statuses.insert(st.status.clone());

        if st.session_bytes_downloaded > 0 || st.session_bytes_uploaded > 0 {
            active += 1;
            stats.update(&st);
            let uds = UpDownStats::from_torrent_stats(&st);
            println!("{t} - {}{n}{}, {}", style::Bold, style::Reset, uds);
        }
        if st.started == 0 {
            println!("{t} - {n}, Not started");
        }
        if st.running == 0 {
            println!("{t} - {n}, Not running");
        }
        if st.stopped_by_error != 0 {
            println!("{t} - {n}, Stopped by error");
        }
    }
    println!("torrents: {}, was active: {active}", ts.len());

    println!("{stats}");
    println!("{statuses:?}");
    println!("{:?}", start.elapsed());
}
