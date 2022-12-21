use crate::{terminal::Terminal, structures::Client, screens::seller::Seller};

fn get_client_code(terminal: &mut Terminal, seller: &Seller) -> Option<usize> {
  terminal.move_to(13, 7);
  terminal.print("                                         │");
  terminal.move_to(13, 7);
  let code = terminal.read_line();

  if code == String::from("") {
    return None
  }

  let code: usize = match code.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      return get_client_code(terminal, seller)
    },
  };

  Some(code)
}

pub fn get(terminal: &mut Terminal, seller: &mut Seller) -> usize {
  let code = get_client_code(terminal, seller);

  match code {
    Some(code) => {
      let client_with_code = seller.clients.get(code);
      match client_with_code {
        Some(client) => {
          terminal.move_to(13, 7);
          terminal.print(&format!("{}", code));
          terminal.move_to(11, 8);
          terminal.print(&client.name);
          terminal.move_to(15, 9);
          terminal.print(&client.address);
          return code
        },
        None => return get(terminal, seller)
      }
    },
    None => {
      let code = seller.clients.len() as usize;
      terminal.move_to(13, 7);
      terminal.print(&format!("{}", code));
      terminal.move_to(11, 8);
      let name = terminal.read_line();
      terminal.move_to(15, 9);
      let address = terminal.read_line();
      seller.clients.push(Client { name, address });
      return code
    }
  }
}
