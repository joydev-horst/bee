// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Autopeering configuration.
//!
//! ## JSON Example
//!
//! ```json
//! "autopeering": {
//!     "enabled": true,
//!     "bindAddress": "0.0.0.0:14626",
//!     "entryNodes": [
//!          "/dns/entry-hornet-0.h.chrysalis-mainnet.iotaledger.net/udp/14626/autopeering/iotaPHdAn7eueBnXtikZMwhfPXaeGJGXDt4RBuLuGgb",
//!          "/dns/entry-hornet-1.h.chrysalis-mainnet.iotaledger.net/udp/14626/autopeering/iotaJJqMd5CQvv1A61coSQCYW9PNT1QKPs7xh2Qg5K2",
//!          "/dns/entry-mainnet.tanglebay.com/udp/14626/autopeering/iot4By1FD4pFLrGJ6AAe7YEeSu9RbW9xnPUmxMdQenC"
//!     ],
//!     "entryNodesPreferIPv6": true,
//! }
//! ```
//!
//! ## TOML Example
//!
//! ```toml
//! [autopeering]
//! enabled = true
//! bind_address = "0.0.0.0:14626"
//! entry_nodes = [
//!     "/dns/entry-hornet-0.h.chrysalis-mainnet.iotaledger.net/udp/14626/autopeering/iotaPHdAn7eueBnXtikZMwhfPXaeGJGXDt4RBuLuGgb",
//!     "/dns/entry-hornet-1.h.chrysalis-mainnet.iotaledger.net/udp/14626/autopeering/iotaJJqMd5CQvv1A61coSQCYW9PNT1QKPs7xh2Qg5K2",
//!     "/dns/entry-mainnet.tanglebay.com/udp/14626/autopeering/iot4By1FD4pFLrGJ6AAe7YEeSu9RbW9xnPUmxMdQenC"
//! ]
//! entry_nodes_prefer_ipv6 = true
//! ```

use crate::multiaddr::AutopeeringMultiaddr;

use serde::Deserialize;

use std::{
    fmt::Debug,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
};

const AUTOPEERING_ENABLED_DEFAULT: bool = false;
// TODO: watch out for possible constification regarding `SocketAddr::new()`.
const AUTOPEERING_BIND_ADDR_DEFAULT: IpAddr = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
const AUTOPEERING_BIND_PORT_DEFAULT: u16 = 14626;
const ENTRYNODES_PREFER_IPV6_DEFAULT: bool = false;
const RUN_AS_ENTRYNODE_DEFAULT: bool = false;
const DROP_NEIGHBORS_ON_SALT_UPDATE_DEFAULT: bool = false;
const PEER_STORAGE_PATH_DEFAULT: &str = "./storage/mainnet/peers";

/// The autopeering config.
#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Clone, Debug)]
pub struct AutopeeringConfig {
    enabled: bool,
    bind_addr: SocketAddr,
    entry_nodes: Vec<AutopeeringMultiaddr>,
    entry_nodes_prefer_ipv6: bool,
    run_as_entry_node: bool,
    drop_neighbors_on_salt_update: bool,
    peer_storage_path: PathBuf,
}

impl AutopeeringConfig {
    /// Whether autopeering should be enabled.
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// The bind address for the server.
    pub fn bind_addr(&self) -> SocketAddr {
        self.bind_addr
    }

    /// The entry nodes for bootstrapping.
    pub fn entry_nodes(&self) -> &[AutopeeringMultiaddr] {
        &self.entry_nodes
    }

    /// Whether `Ipv4` or `Ipv6` should be preferred in case a hostname supports both.
    pub fn entry_nodes_prefer_ipv6(&self) -> bool {
        self.entry_nodes_prefer_ipv6
    }

    /// Whether the node should run as an entry node.
    pub fn run_as_entry_node(&self) -> bool {
        self.run_as_entry_node
    }

    /// Whether all neighbors should be disconnected from when the salts are updated.
    pub fn drop_neighbors_on_salt_update(&self) -> bool {
        self.drop_neighbors_on_salt_update
    }

    /// Reduces this config to its list of entry node addresses.
    pub fn into_entry_nodes(self) -> Vec<AutopeeringMultiaddr> {
        self.entry_nodes
    }

    /// The peer storage path.
    pub fn peer_storage_path(&self) -> &Path {
        &self.peer_storage_path
    }
}

// Note: In case someone wonders why we use `Option<bool>`: Although serde actually provides a way to allow for the
// default of a boolean parameter to be `true` - so that missing config parameters could be created on the fly - it felt
// too awkward and also a bit too cumbersome to me: serde(default = "default_providing_function_name").

/// The autopeering config builder.
#[derive(Clone, Debug, Deserialize)]
#[must_use]
pub struct AutopeeringConfigBuilder {
    /// Whether autopeering should be enabled.
    pub enabled: bool,
    /// The bind address for the server.
    #[serde(alias = "bindAddress", alias = "bind_address")]
    pub bind_addr: SocketAddr,
    /// The entry nodes for bootstrapping.
    #[serde(alias = "entryNodes")]
    pub entry_nodes: Vec<AutopeeringMultiaddr>,
    /// Whether `Ipv4` or `Ipv6` should be preferred in case a hostname supports both.
    #[serde(alias = "entryNodesPreferIPv6")]
    pub entry_nodes_prefer_ipv6: Option<bool>,
    /// Whether the node should run as an entry node.
    #[serde(alias = "runAsEntryNode")]
    pub run_as_entry_node: Option<bool>,
    /// Whether all neighbors should be disconnected from when the salts are updated.
    #[serde(alias = "dropNeighborsOnSaltUpdate")]
    pub drop_neighbors_on_salt_update: Option<bool>,
    /// The peer storage path.
    #[serde(alias = "peerStoragePath")]
    pub peer_storage_path: Option<PathBuf>,
}

impl AutopeeringConfigBuilder {
    /// Builds the actual `AutopeeringConfig`.
    pub fn finish(self) -> AutopeeringConfig {
        AutopeeringConfig {
            enabled: self.enabled,
            bind_addr: self.bind_addr,
            entry_nodes: self.entry_nodes,
            entry_nodes_prefer_ipv6: self.entry_nodes_prefer_ipv6.unwrap_or(ENTRYNODES_PREFER_IPV6_DEFAULT),
            run_as_entry_node: self.run_as_entry_node.unwrap_or(RUN_AS_ENTRYNODE_DEFAULT),
            drop_neighbors_on_salt_update: self
                .drop_neighbors_on_salt_update
                .unwrap_or(DROP_NEIGHBORS_ON_SALT_UPDATE_DEFAULT),
            peer_storage_path: self
                .peer_storage_path
                .unwrap_or_else(|| PEER_STORAGE_PATH_DEFAULT.into()),
        }
    }
}

impl Default for AutopeeringConfigBuilder {
    fn default() -> Self {
        Self {
            enabled: AUTOPEERING_ENABLED_DEFAULT,
            bind_addr: SocketAddr::new(AUTOPEERING_BIND_ADDR_DEFAULT, AUTOPEERING_BIND_PORT_DEFAULT),
            entry_nodes: Vec::default(),
            entry_nodes_prefer_ipv6: Some(ENTRYNODES_PREFER_IPV6_DEFAULT),
            run_as_entry_node: Some(RUN_AS_ENTRYNODE_DEFAULT),
            drop_neighbors_on_salt_update: Some(DROP_NEIGHBORS_ON_SALT_UPDATE_DEFAULT),
            peer_storage_path: Some(PEER_STORAGE_PATH_DEFAULT.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_config_from_json_str() -> AutopeeringConfig {
        let config_json_str = r#"
        {
            "enabled": true,
            "bindAddress": "0.0.0.0:14626",
            "entryNodes": [
                "/dns/lucamoser.ch/udp/14826/autopeering/4H6WV54tB29u8xCcEaMGQMn37LFvM1ynNpp27TTXaqNM",
                "/dns/entry-hornet-0.h.chrysalis-mainnet.iotaledger.net/udp/14626/autopeering/iotaPHdAn7eueBnXtikZMwhfPXaeGJGXDt4RBuLuGgb",
                "/dns/entry-hornet-1.h.chrysalis-mainnet.iotaledger.net/udp/14626/autopeering/iotaJJqMd5CQvv1A61coSQCYW9PNT1QKPs7xh2Qg5K2",
                "/dns/entry-mainnet.tanglebay.com/udp/14626/autopeering/iot4By1FD4pFLrGJ6AAe7YEeSu9RbW9xnPUmxMdQenC"
            ],
            "entryNodesPreferIPv6": true,
            "runAsEntryNode": false,
            "dropNeighborsOnSaltUpdate": false,
            "peerStoragePath": "./storage/mainnet/peers"
        }"#;

        serde_json::from_str::<AutopeeringConfigBuilder>(config_json_str)
            .expect("error deserializing json config")
            .finish()
    }

    fn create_config_from_toml_str() -> AutopeeringConfig {
        let toml_config_str = r#"
            enabled = true
            bind_address = "0.0.0.0:14626"
            entry_nodes = [
                "/dns/lucamoser.ch/udp/14826/autopeering/4H6WV54tB29u8xCcEaMGQMn37LFvM1ynNpp27TTXaqNM",
                "/dns/entry-hornet-0.h.chrysalis-mainnet.iotaledger.net/udp/14626/autopeering/iotaPHdAn7eueBnXtikZMwhfPXaeGJGXDt4RBuLuGgb",
                "/dns/entry-hornet-1.h.chrysalis-mainnet.iotaledger.net/udp/14626/autopeering/iotaJJqMd5CQvv1A61coSQCYW9PNT1QKPs7xh2Qg5K2",
                "/dns/entry-mainnet.tanglebay.com/udp/14626/autopeering/iot4By1FD4pFLrGJ6AAe7YEeSu9RbW9xnPUmxMdQenC"
            ]
            entry_nodes_prefer_ipv6 = true
            run_as_entry_node = false
            drop_neighbors_on_salt_update = false
            peer_storage_path = "./storage/mainnet/peers"
        "#;

        toml::from_str::<AutopeeringConfigBuilder>(toml_config_str)
            .unwrap()
            .finish()
    }

    fn create_config() -> AutopeeringConfig {
        AutopeeringConfig {
            enabled: true,
            bind_addr: "0.0.0.0:14626".parse().unwrap(),
            entry_nodes: vec![
                "/dns/lucamoser.ch/udp/14826/autopeering/4H6WV54tB29u8xCcEaMGQMn37LFvM1ynNpp27TTXaqNM".parse().unwrap(),
                "/dns/entry-hornet-0.h.chrysalis-mainnet.iotaledger.net/udp/14626/autopeering/iotaPHdAn7eueBnXtikZMwhfPXaeGJGXDt4RBuLuGgb".parse().unwrap(),
                "/dns/entry-hornet-1.h.chrysalis-mainnet.iotaledger.net/udp/14626/autopeering/iotaJJqMd5CQvv1A61coSQCYW9PNT1QKPs7xh2Qg5K2".parse().unwrap(),
                "/dns/entry-mainnet.tanglebay.com/udp/14626/autopeering/iot4By1FD4pFLrGJ6AAe7YEeSu9RbW9xnPUmxMdQenC".parse().unwrap(),
            ],
            entry_nodes_prefer_ipv6: true,
            run_as_entry_node: false,
            drop_neighbors_on_salt_update: false,
            peer_storage_path: "./storage/mainnet/peers".into()
        }
    }

    /// Tests config serialization and deserialization.
    #[test]
    fn config_serde() {
        // Create format dependent configs from their respective string representation.
        let json_config = create_config_from_json_str();
        let toml_config = create_config_from_toml_str();

        // Manually create an instance of a config.
        let config = create_config();

        // Compare whether the deserialized JSON str equals the JSON-serialized config instance.
        assert_eq!(json_config, config, "json config de/serialization failed");

        // Compare whether the deserialized TOML str equals the TOML-serialized config instance.
        assert_eq!(toml_config, config, "toml config de/serialization failed");
    }
}
