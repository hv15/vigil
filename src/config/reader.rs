// Vigil
//
// Microservices Status Page
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use envsubst::substitute;
use std::{
    collections::{hash_set::HashSet, HashMap},
    env, fs,
};

use toml;

use super::config::*;
use crate::APP_ARGS;

pub struct ConfigReader;

impl ConfigReader {
    pub fn make() -> Config {
        debug!("reading config file: {}", &APP_ARGS.config);

        // Read configuration
        let mut conf = fs::read_to_string(&APP_ARGS.config).expect("cannot find config file");

        debug!("read config file: {}", &APP_ARGS.config);

        // Replace environment variables
        let environment = env::vars().collect::<HashMap<String, String>>();

        conf = substitute(&conf, &environment).expect("cannot substitute environment variables");

        // Parse configuration
        let config = toml::from_str(&conf).expect("syntax error in config file");

        // Validate configuration
        Self::validate(&config);

        config
    }

    fn validate(config: &Config) {
        // Validate all identifiers
        Self::validate_identifiers(config)
    }

    fn validate_identifiers(config: &Config) {
        // Scan for identifier duplicates
        let mut identifiers = HashSet::new();

        for service in config.probe.service.as_deref().unwrap_or_default().iter() {
            // Service identifier was already previously inserted? (caught a duplicate)
            if identifiers.insert(&service.id) == false {
                panic!(
                    "configuration has duplicate service identifier: {}",
                    service.id
                )
            }

            // Scan for node identifier duplicates
            let mut node_identifiers = HashSet::new();

            for node in service.node.iter() {
                // Node identifier was already previously inserted? (caught a duplicate)
                if node_identifiers.insert(&node.id) == false {
                    panic!(
                        "configuration has duplicate node identifier: {} in service: {}",
                        node.id, service.id
                    )
                }
            }
        }

        // clear to check new identifiers
        identifiers.clear();

        for cluster in config.probe.cluster.as_deref().unwrap_or_default().iter() {
            // Cluster identifier was already previously inserted? (caught a duplicate)
            if identifiers.insert(&cluster.id) == false {
                panic!(
                    "configuration has duplicate cluster identifier: {}",
                    cluster.id
                )
            }

            // Scan for group and node identifier duplicates
            let mut group_identifiers = HashSet::new();
            let mut node_identifiers = HashSet::new();

            for group in cluster.group.iter() {
                // Node identifier was already previously inserted? (caught a duplicate)
                if group_identifiers.insert(&group.id) == false {
                    panic!(
                        "configuration has duplicate group identifier: {} in cluster: {}",
                        group.id, cluster.id
                    )
                }
                for node in group.node.iter() {
                    // Node identifier was already previously inserted? (caught a duplicate)
                    if node_identifiers.insert(&node.id) == false {
                        panic!(
                            "configuration has duplicate node identifier: {} in group: {} from cluster: {}",
                            node.id, group.id, cluster.id
                        )
                    }
                }
            }
        }
    }
}
