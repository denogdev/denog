// Copyright 2023 Jo Bates. All rights reserved. MIT license.

use std::{
  collections::HashMap,
  fmt::{self, Debug, Formatter},
};
use winit::{event_loop::EventLoopWindowTarget, window::Window};

pub type ExecuteRequestFn =
  dyn FnOnce(&EventLoopWindowTarget<()>, &mut HashMap<u64, Window>) + Send;

pub enum Request {
  NextEvent,
  Execute(Box<ExecuteRequestFn>),
}

impl Debug for Request {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Request::NextEvent => f.write_str("Request::NextEvent"),
      Request::Execute(_) => f.write_str("Request::Execute"),
    }
  }
}
