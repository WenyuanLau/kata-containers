// Copyright (c) 2023 Huawei Technologies Co.,Ltd.
//
// SPDX-License-Identifier: Apache-2.0
//

use anyhow::Result;

use crate::stratovirt::StratoVirtInner;
use crate::device::DeviceType;

impl StratoVirtInner {
    pub(crate) async fn add_device(&mut self, device: DeviceType) -> Result<DeviceType> {
        info!(sl!(), "StratoVirtInner: add_device() {}", device);
        
        Ok(device)
    }

    pub(crate) async fn remove_device(&mut self, device: DeviceType) -> Result<()> {
        info!(sl!(), "StratoVirtInner: remove_device() {} ", device);
        todo!()
    }

    pub(crate) async fn update_device(&mut self, device: DeviceType) -> Result<()> {
        info!(sl!(), "StratoVirtInner: update_device() {:?}", &device);
        todo!()
    }
}