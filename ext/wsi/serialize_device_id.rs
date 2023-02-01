// Copyright 2023 Jo Bates. All rights reserved. MIT license.

use std::{collections::HashMap, sync::Mutex};
use winit::event::DeviceId;

pub fn serialize_device_id(device_id: DeviceId) -> u32 {
  static STATE: Mutex<Option<State>> = Mutex::new(None);
  struct State {
    map: HashMap<DeviceId, u32>,
    next: u32,
  }

  let mut state = STATE.lock().unwrap();
  let state = match &mut *state {
    None => {
      *state = Some(State {
        map: HashMap::new(),
        next: 1,
      });
      state.as_mut().unwrap()
    }
    Some(state) => state,
  };

  match state.map.get(&device_id) {
    Some(&u) => u,
    None => {
      let u = state.next;
      state.next += 1;
      state.map.insert(device_id, u);
      u
    }
  }
}
