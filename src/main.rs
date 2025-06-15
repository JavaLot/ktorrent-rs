use jiff::Timestamp;
use jiff::tz::TimeZone;
use ktorrent_rs::KTorrent;
use ktorrent_rs::torrent::{TorrentStats, UpDownStats};
use std::collections::HashMap;
use std::time::Instant;
use termion::{color, style};

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
    let mut statuses: HashMap<String, usize> = HashMap::new();
    let mut stats = UpDownStats::new();
    for t in &ts {
        let tp = kt.get_torrent_proxy(t.as_str()).await.unwrap();
        let n = tp.name().await.unwrap();
        let s = tp.stats().await.unwrap();
        let st: TorrentStats = serde_bencode::from_bytes(&s).unwrap();
        if let Some(c) = statuses.get_mut(&st.status) {
            *c += 1;
        } else {
            statuses.insert(st.status.clone(), 1);
        }

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
    let dt = Timestamp::now().to_zoned(TimeZone::system()).datetime();
    println!(
        "{}{}{}{} - torrents: {}, was active: {}{active}{}",
        style::Bold,
        color::Fg(color::Yellow),
        dt.strftime("%F %T"),
        style::Reset,
        ts.len(),
        style::Bold,
        style::Reset,
    );

    println!("{stats}");
    println!("{statuses:?}");
    println!("{:?}", start.elapsed());
}
