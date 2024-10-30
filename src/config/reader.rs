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
        let mut sub_identifiers = HashSet::new();

        for service in config.probe.service.iter() {
            // Service identifier was already previously inserted? (caught a duplicate)
            if identifiers.insert(&service.id) == false {
                panic!(
                    "configuration has duplicate service identifier: {}",
                    service.id
                )
            }

            for node in service.node.as_deref().unwrap_or_default().iter() {
                // Node identifier was already previously inserted? (caught a duplicate)
                if sub_identifiers.insert(&node.id) == false {
                    panic!(
                        "configuration has duplicate node identifier: {} in service: {}",
                        node.id, service.id
                    )
                }
            }

            sub_identifiers.clear();

            for group in service.group.as_deref().unwrap_or_default().iter() {
                // Group identifier was already previously inserted? (caught a duplicate)
                if sub_identifiers.insert(&group.id) == false {
                    panic!(
                        "configuration has duplicate group identifier: {} in service: {}",
                        group.id, service.id
                    )
                }
            }

            sub_identifiers.clear();
            for group in service.group.as_deref().unwrap_or_default().iter() {
                for node in group.node.iter() {
                    // Node identifier was already previously inserted? (caught a duplicate)
                    if sub_identifiers.insert(&node.id) == false {
                        panic!(
                            "configuration has duplicate node identifier: {} in group: {} in service: {}",
                            node.id, group.id, service.id
                        )
                    }
                }
            }
        }
    }
}
