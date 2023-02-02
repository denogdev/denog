// Copyright 2023 Jo Bates. All rights reserved. MIT license.

use std::collections::HashMap;
use winit::event::DeviceId;

pub struct DeviceIds {
  map: HashMap<DeviceId, u32>,
  next: u32,
}

impl DeviceIds {
  pub fn new() -> Self {
    Self {
      map: HashMap::new(),
      next: 1,
    }
  }

  pub fn get(&mut self, device_id: DeviceId) -> u32 {
    match self.map.get(&device_id) {
      Some(&u) => u,
      None => {
        let u = self.next;
        self.next += 1;
        self.map.insert(device_id, u);
        u
      }
    }
  }
}
