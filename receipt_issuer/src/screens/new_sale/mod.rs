use crate::{
  terminal::Terminal,
  screens::{seller::Seller, receipt},
  structures::*
};
use crossterm::{execute, style::Print, cursor};

mod client;
mod amount;

pub fn load(terminal: &mut Terminal, seller: &mut Seller) {
  terminal.clear();
  execute!(
    terminal.stdout,
    Print("\n\n"),
    Print("  ╭─ [Nova venda] emissor de recibos ─────────────────╮\n"),
    Print("  │                                                   │\n"),
    Print("  │  Vendedor:                                        │\n"),
    Print("  │  Endereço:                                        │\n"),
    Print("  │                                                   │\n"),
    Print("  │  [Cliente]                                        │\n"),
    Print("  │  Código:                                          │\n"),
    Print("  │  Nome:                                            │\n"),
    Print("  │  Endereço:                                        │\n"),
    Print("  │                                                   │\n"),
    Print("  │  [Itens]                                          │\n"),
    Print("  │  Quantidade:                                      │\n"),
    Print("  │  Descrição:                                       │\n"),
    Print("  │  Valor unitário:                                  │\n"),
    Print("  │                                                   │\n"),
    Print("  │  Valor TOTAL:                                     │\n"),
    Print("  │                                                   │\n"),
    Print("  ╰───────────────────────────────────────────────────╯\n"),
    Print("     Para novo cliente, deixe o código em branco"),
    cursor::EnableBlinking,
  ).expect("Não foi possível imprimir o formulário de nova venda");

  // print seller data
  terminal.move_to(15, 4);
  terminal.print(seller.name.trim());
  terminal.move_to(15, 5);
  terminal.print(seller.address.trim());

  // define client
  let client_code = client::get(terminal, seller);

  // define amount
  let amount = amount::get(terminal);

  fn get_value(terminal: &mut Terminal) -> f64 {
    terminal.move_to(21, 15);
    let value = terminal.read_line();

    let value: f64 = match value.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        terminal.move_to(21, 15);
        terminal.print("                                 │");
        return get_value(terminal)
      },
    };
    value
  }
  // define product
  terminal.move_to(16, 14);
  let description = terminal.read_line();
  let value = get_value(terminal);
  let product = Product { description, value };

  // define sale
  let sale = Sale { client_code, product, amount };

  seller.new_sale(&sale);

  terminal.move_to(18, 17);

  terminal.print(&format!(
    "{}",
    sale.total()
  ));
  terminal.move_to(5, 20);

  terminal.print("Venda adicionada! deseja imprimir o recibo?");
  terminal.move_to(5, 21);
  let command = terminal.read_line();

  if command == String::from("sim") {
    receipt::load(terminal, &seller, &sale);
  }

}
