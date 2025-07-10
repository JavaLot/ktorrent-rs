// zbus(signal,

use jiff::Timestamp;
use jiff::tz::TimeZone;
use ktorrent_rs::KTorrent;
use zbus::export::ordered_stream::OrderedStreamExt;

#[tokio::main]
async fn main() {
    let rkt = KTorrent::new().await;
    assert!(rkt.is_ok());

    let bkt = Box::new(rkt.unwrap());
    let kt = Box::leak(bkt);
    let c = kt.get_core_proxy().await.unwrap();

    let h1 = tokio::spawn({
        let c = c.clone();
        async move {
            let mut tas = c.receive_torrent_added().await.unwrap();
            while let Some(m) = tas.next().await {
                if let Ok(a) = m.args() {
                    let dt = Timestamp::now().to_zoned(TimeZone::system()).datetime();
                    println!("{} torrent added: {}", dt.strftime("%F %T"), a.tor);
                }
            }
        }
    });

    let h2 = tokio::spawn({
        let c = c.clone();
        async move {
            let mut trs = c.receive_torrent_removed().await.unwrap();
            while let Some(m) = trs.next().await {
                if let Ok(a) = m.args() {
                    let dt = Timestamp::now().to_zoned(TimeZone::system()).datetime();
                    println!("{} torrent removed: {}", dt.strftime("%F %T"), a.tor);
                }
            }
        }
    });

    let h3 = tokio::spawn({
        let c = c.clone();
        async move {
            let mut fs = c.receive_finished().await.unwrap();
            while let Some(m) = fs.next().await {
                if let Ok(a) = m.args() {
                    let dt = Timestamp::now().to_zoned(TimeZone::system()).datetime();
                    println!("{} finished: {}", dt.strftime("%F %T"), a.tor);
                }
            }
        }
    });

    h1.await.unwrap();
    h2.await.unwrap();
    h3.await.unwrap();
}
