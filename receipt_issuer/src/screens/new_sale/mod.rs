use crate::{
  terminal::Terminal,
  screens::{seller::Seller, receipt},
  structures::*
};
use crossterm::{execute, cursor};
use interface_builder::{Application, Page};

mod client;
mod amount;

pub fn load(terminal: &mut Terminal, seller: &mut Seller) {
  let mut app = Application::new();
  app.home(Page::new(
    Some("[Nova venda] emissor de recibos"),
    vec![
      &format!("Vendedor: {}", seller.name),
      &format!("Endereço: {}", seller.address),
      "",
      "[Cliente]",
      "Código:",
      "Nome:",
      "Endereço:",
      " ",
      "[Itens]",
      "Quantidade:",
      "Descrição:",
      "Valor unitário:",
      "",
      "Valor TOTAL:",
    ],
    Some(vec!["Para novo cliente, deixe o código em branco."]),
    53, None
  ));
  app.run();

  execute!(
    terminal.stdout,
    cursor::EnableBlinking,
  ).expect("Não foi possível configurar o cursor");

  // define client
  let client_code = client::get(terminal, seller);

  // define amount
  let amount = amount::get(terminal);

  fn get_value(terminal: &mut Terminal) -> f64 {
    terminal.move_to(21, 14);
    let value = terminal.read_line();

    let value: f64 = match value.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        terminal.move_to(21, 14);
        terminal.print("                                 │");
        return get_value(terminal)
      },
    };
    value
  }
  // define product
  terminal.move_to(16, 13);
  let description = terminal.read_line();
  let value = get_value(terminal);
  let product = Product { description, value };

  // define sale
  let sale = Sale { client_code, product, amount };

  seller.new_sale(&sale);

  terminal.move_to(18, 16);

  terminal.print(&format!(
    "{}",
    sale.total()
  ));
  terminal.move_to(5, 19);

  terminal.print("Venda adicionada! deseja imprimir o recibo? ");
  terminal.move_to(5, 20);
  let command = terminal.read_line();

  if command == String::from("sim") {
    receipt::load(terminal, &seller, &sale);
  }

}
