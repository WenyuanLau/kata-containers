// Copyright (c) 2023 Huawei Technologies Co.,Ltd.
//
// SPDX-License-Identifier: Apache-2.0
//

use anyhow::Result;

use crate::{HypervisorConfig, VcpuThreadIds};
use kata_types::capabilities::{Capabilities, CapabilityBits};

#[derive(Debug)]
pub struct StratoVirtInner {
    config: HypervisorConfig,
}

impl StratoVirtInner {
    pub fn new() -> Self {
        Self {
            config: Default::default(),
        }
    }

    pub(crate) async fn prepare_vm(&mut self, _id: &str, _netns: Option<String>) -> Result<()> {
        info!(sl!(), "StratoVirtInner: prepare_vm()");
        Ok(())
    }

    pub(crate) async fn start_vm(&mut self, _timeout: i32) -> Result<()> {
        info!(sl!(), "StratoVirtInner: start_vm()");

        Ok(())
    }

    pub(crate) fn stop_vm(&mut self) -> Result<()> {
        info!(sl!(), "StratoVirtInner: stop_vm()");

        todo!()
    }

    pub(crate) fn pause_vm(&self) -> Result<()> {
        todo!()
    }

    pub(crate) fn resume_vm(&self) -> Result<()> {
        todo!()
    }

    pub(crate) async fn save_vm(&self) -> Result<()> {
        todo!()
    }

    pub(crate) async fn get_agent_socket(&self) -> Result<String> {
        info!(sl!(), "StratoVirtInner: get_agent_socket()");
        todo!()
    }

    pub(crate) async fn disconnect(&mut self) {
        info!(sl!(), "StratoVirtInner: disconnect()");
        todo!()
    }

    pub(crate) async fn get_thread_ids(&self) -> Result<VcpuThreadIds> {
        info!(sl!(), "StratoVirtInner: get_thread_ids()");
        todo!()
    }

    pub(crate) async fn get_vmm_master_tid(&self) -> Result<u32> {
        info!(sl!(), "StratoVirtInner: get_vmm_master_tid()");
        todo!()
    }

    pub(crate) async fn get_ns_path(&self) -> Result<String> {
        info!(sl!(), "StratoVirtInner: get_ns_path()");
        todo!()
    }

    pub(crate) async fn cleanup(&self) -> Result<()> {
        info!(sl!(), "StratoVirtInner: cleanup()");
        todo!()
    }

    pub(crate) async fn resize_vcpu(&self, _old_vcpus: u32, _new_vcpus: u32) -> Result<(u32, u32)> {
        info!(sl!(), "StratoVirtInner: resize_vcpu()");
        todo!()
    }

    pub(crate) async fn get_pids(&self) -> Result<Vec<u32>> {
        info!(sl!(), "StratoVirtInner: get_pids()");
        todo!()
    }

    pub(crate) async fn check(&self) -> Result<()> {
        todo!()
    }

    pub(crate) async fn get_jailer_root(&self) -> Result<String> {
        todo!()
    }

    pub(crate) async fn capabilities(&self) -> Result<Capabilities> {
        let mut caps = Capabilities::default();
        caps.set(CapabilityBits::FsSharingSupport);

        Ok(caps)
    }

    pub(crate) async fn get_hypervisor_metrics(&self) -> Result<String> {
        todo!()
    }

    pub fn set_hypervisor_config(&mut self, config: HypervisorConfig) {
        info!(sl!(), "StratoVirtInner: set_hypervisor_config()");

        self.config = config;
    }

    pub fn get_hypervisor_config(&self) -> HypervisorConfig {
        info!(sl!(), "StratoVirtInner: get_hypervisor_config()");
        self.config.clone()
    }
}