extern crate pbr;

use std::{time::Duration, thread};

use pbr::ProgressBar;

pub fn run(total: u64, millis: u64) {
  let mut pb = ProgressBar::new(total);
  pb.format("[=>-]");
  pb.show_speed = false;
  pb.show_time_left = false;
  for _ in 0..total {
    pb.inc();
    thread::sleep(Duration::from_millis(millis));
  }
}
