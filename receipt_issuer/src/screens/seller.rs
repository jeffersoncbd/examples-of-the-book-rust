use crate::{terminal::Terminal, structures::{Sale, Client, Product}};
use crossterm::{execute, cursor};
use interface_builder::{Application, Page};

pub struct Seller {
  pub name: String,
  pub address: String,
  pub invoicing: f64,
  pub clients: Vec<Client>,
  pub products: Vec<Product>
}
impl Seller {
  pub fn new(name: String, address: String) -> Seller {
    Seller {
      name, address,
      invoicing: 0.0,
      clients: Vec::new(),
      products: Vec::new(),
    }
  }
  pub fn new_sale(&mut self, sale: &Sale) {
    self.invoicing += sale.total();
  }
}

pub fn load(terminal: &mut Terminal) -> Seller {
  let mut app = Application::new();
  app.home(Page::new(
    Some("Seja bem vindo vendedor!"),
    vec![
      "Antes de começar, faça o seu cadastro:",
      "",
      "Nome:",
      "Endereço:",
    ],
    None,
    53, None
  ));
  app.run();


  execute!(
    terminal.stdout,
    cursor::MoveTo(11, 5),
    cursor::Show,
    cursor::EnableBlinking,
  ).expect("Não foi possível imprimir o formulário de cadastro do vendedor");

  let name = terminal.read_line();
  terminal.move_to(15, 6);
  let address = terminal.read_line();

  terminal.move_to(5, 9);
  println!("Se estiver certo, digite \"sim\":");
  terminal.move_to(5, 10);
  let confirmation = terminal.read_line();
  if confirmation != String::from("sim") {
    return load(terminal);
  }

  Seller::new(name, address)
}
