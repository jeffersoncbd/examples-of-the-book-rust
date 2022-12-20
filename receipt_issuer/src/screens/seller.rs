use crate::{terminal::Terminal, structures::{Sale, Client}};
use crossterm::{execute, style::Print, cursor};

pub struct Seller {
  pub name: String,
  pub address: String,
  pub invoicing: f64,
  pub clients: Vec<Client>
}
impl Seller {
  pub fn new(name: String, address: String) -> Seller {
    Seller { name, address, invoicing: 0.0, clients: Vec::new() }
  }
  pub fn new_sale(&mut self, sale: &Sale) {
    self.invoicing += sale.total();
  }
}

pub fn load(terminal: &mut Terminal) -> Seller {
  terminal.clear();
  execute!(
    terminal.stdout,
    Print("\n\n"),
    Print("  ╭───────── Bem vindo ao emissor de recibos ─────────╮\n"),
    Print("  │                                                   │\n"),
    Print("  │ Antes de começar o vendedor deve ser cadastrado:  │\n"),
    Print("  │                                                   │\n"),
    Print("  │ Nome:                                             │\n"),
    Print("  │ Endereço:                                         │\n"),
    Print("  │                                                   │\n"),
    Print("  ╰───────────────────────────────────────────────────╯"),
    cursor::MoveTo(10, 6),
    cursor::Show,
    cursor::EnableBlinking,
  ).expect("Não foi possível imprimir o formulário de cadastro do vendedor");

  let name = terminal.read_line();
  terminal.move_to(14, 7);
  let address = terminal.read_line();

  terminal.move_to(4, 10);
  println!("Se estes dados estiverem corretos, digite \"sim\":");
  terminal.move_to(4, 11);
  let confirmation = terminal.read_line();
  if confirmation != String::from("sim") {
    return load(terminal);
  }

  Seller::new(name, address)
}
