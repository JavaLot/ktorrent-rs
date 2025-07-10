pub mod core;
pub mod group;
pub mod settings;
pub mod torrent;

use crate::core::coreProxy;
use crate::group::GroupProxy;
use crate::settings::settingsProxy;
use crate::torrent::TorrentProxy;
use quick_xml::de::Deserializer;
use std::io::{BufReader, Read};
use zbus::Connection;
use zbus::export::serde::Deserialize;

pub const KTORRENT_NAME: &str = "org.kde.ktorrent";

#[derive(Clone)]
pub struct KTorrent {
    con: Connection,
}

#[derive(Debug)]
pub enum KTorrentError {
    ZBusError(zbus::Error),
    ZBusFdoError(zbus::fdo::Error),
    ZBusXmlError(zbus_xml::Error),
}

impl From<zbus::Error> for KTorrentError {
    fn from(value: zbus::Error) -> Self {
        KTorrentError::ZBusError(value)
    }
}

impl From<zbus::fdo::Error> for KTorrentError {
    fn from(value: zbus::fdo::Error) -> Self {
        KTorrentError::ZBusFdoError(value)
    }
}

impl From<zbus_xml::Error> for KTorrentError {
    fn from(value: zbus_xml::Error) -> Self {
        KTorrentError::ZBusXmlError(value)
    }
}

impl KTorrent {
    pub async fn new() -> Result<Self, KTorrentError> {
        let c = Connection::session().await;

        match c {
            Ok(con) => Ok(Self { con }),
            Err(err) => Err(KTorrentError::ZBusError(err)),
        }
    }

    pub async fn is_running(&self) -> Result<bool, KTorrentError> {
        let dbus = zbus::fdo::DBusProxy::new(&self.con).await?;
        let ns = dbus.list_names().await?;
        for n in ns {
            if n == KTORRENT_NAME {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub async fn list_group_names(&self) -> Result<Vec<String>, KTorrentError> {
        let group = zbus::fdo::IntrospectableProxy::builder(&self.con)
            .destination(KTORRENT_NAME)?
            .path("/group")?
            .build()
            .await?;
        let group_xml = group.introspect().await?;
        let group_node = zbus_xml::Node::from_trusted_reader(group_xml.as_bytes())?;
        Ok(group_node
            .nodes()
            .iter()
            .filter_map(|n| n.name())
            .map(|s| s.into())
            .collect())
    }

    pub async fn list_torrent_names(&self) -> Result<Vec<String>, KTorrentError> {
        let torrent = zbus::fdo::IntrospectableProxy::builder(&self.con)
            .destination(KTORRENT_NAME)?
            .path("/torrent")?
            .build()
            .await?;
        let torrent_xml = torrent.introspect().await?;
        let torrent_node = zbus_xml::Node::from_trusted_reader(torrent_xml.as_bytes())?;
        Ok(torrent_node
            .nodes()
            .iter()
            .filter_map(|n| n.name())
            .map(|s| s.into())
            .collect())
    }

    pub async fn get_group_proxy(&self, name: &str) -> Result<GroupProxy, KTorrentError> {
        let path = format!("/group/{name}");
        Ok(GroupProxy::builder(&self.con)
            .destination(KTORRENT_NAME)?
            .path(path)?
            .build()
            .await?)
    }

    pub async fn get_torrent_proxy(&self, name: &str) -> Result<TorrentProxy, KTorrentError> {
        let path = format!("/torrent/{name}");
        Ok(TorrentProxy::builder(&self.con)
            .destination(KTORRENT_NAME)?
            .path(path)?
            .build()
            .await?)
    }

    pub async fn get_core_proxy(&self) -> Result<coreProxy, KTorrentError> {
        let path = "/core".to_string();
        Ok(coreProxy::builder(&self.con)
            .destination(KTORRENT_NAME)?
            .path(path)?
            .build()
            .await?)
    }

    pub async fn get_settings_proxy(&self) -> Result<settingsProxy, KTorrentError> {
        let path = "/settings".to_string();
        Ok(settingsProxy::builder(&self.con)
            .destination(KTORRENT_NAME)?
            .path(path)?
            .build()
            .await?)
    }
}

pub trait TrustedReader<'a> {
    fn from_trusted_reader<R: Read>(reader: R) -> zbus_xml::Result<zbus_xml::Node<'a>>;
}

impl<'a> TrustedReader<'a> for zbus_xml::Node<'a> {
    fn from_trusted_reader<R: Read>(reader: R) -> zbus_xml::Result<zbus_xml::Node<'a>> {
        let mut deserializer = Deserializer::from_reader(BufReader::new(reader));
        Ok(zbus_xml::Node::deserialize(&mut deserializer)?)
    }
}
