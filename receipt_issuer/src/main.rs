extern crate receipt_issuer;

use receipt_issuer::{
  terminal::{self, Terminal},
  screens::{
    self,
    seller::Seller
  }
};

fn main() {
  let mut terminal = terminal::new();

  terminal.fake_loading();

  let mut seller = screens::seller::load(&mut terminal);

  terminal.fake_loading();

  loop {
    application(&mut terminal, &mut seller);
    if terminal.end {
      break
    }
  }

  terminal.clear();
}

fn application(terminal: &mut Terminal, seller: &mut Seller) {
  let option = screens::menu::load(terminal);
  match option {
    1 => screens::new_sale::load(terminal, seller),
    2 => screens::billing::load(terminal, seller),
    3 => screens::clients::load(terminal, seller),
    4 => screens::new_client::load(terminal, seller),
    0 => { terminal.end = true; },
    _ => panic!("Unexpected option"),
  }
}
