// Copyright (c) 2023 Huawei Technologies Co.,Ltd.
//
// SPDX-License-Identifier: Apache-2.0
//

use std::io::Result;
use std::sync::Arc;

use super::register_hypervisor_plugin;
use crate::config::default::{MAX_STRATOVIRT_VCPUS, MIN_STRATOVIRT_MEMORY_SIZE_MB};
use crate::config::{ConfigPlugin, TomlConfig};

/// Hypervisor name for StratoVirt, used to index `TomlConfig::hypervisor`.
pub const HYPERVISOR_NAME_STRATOVIRT: &str = "stratovirt";

/// Configuration information for StratoVirt.
#[derive(Default, Debug)]
pub struct StratoVirtConfig {}

impl StratoVirtConfig {
    /// Create a new instance of `StratoVirtConfig`.
    pub fn new() -> Self {
        StratoVirtConfig {}
    }

    /// Register the stratovirt plugin.
    pub fn register(self) {
        let plugin = Arc::new(self);
        register_hypervisor_plugin(HYPERVISOR_NAME_STRATOVIRT, plugin);
    }
}

impl ConfigPlugin for StratoVirtConfig {
    fn name(&self) -> &str {
        HYPERVISOR_NAME_STRATOVIRT
    }

    fn get_min_memory(&self) -> u32 {
        MIN_STRATOVIRT_MEMORY_SIZE_MB
    }

    fn get_max_cpus(&self) -> u32 {
        MAX_STRATOVIRT_VCPUS
    }

    /// Adjust the configuration information after loading from configuration file.
    fn adjust_config(&self, _conf: &mut TomlConfig) -> Result<()> {
        Ok(())
    }

    /// Validate the configuration information.
    fn validate(&self, _conf: &TomlConfig) -> Result<()> {
        Ok(())
    }
}
