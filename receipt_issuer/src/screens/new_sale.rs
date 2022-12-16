use crate::{
  terminal::Terminal,
  screens::{seller::Seller, receipt},
  structures::*
};
use crossterm::{execute, style::Print, cursor};


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
    Print("  │  [Dados do cliente]                               │\n"),
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
    cursor::EnableBlinking,
  ).expect("Não foi possível imprimir o formulário de nova venda");

  // print seller data
  terminal.move_to(15, 4);
  terminal.print(seller.name.trim());
  terminal.move_to(15, 5);
  terminal.print(seller.address.trim());

  // define client data
  terminal.move_to(11, 8);
  let name = terminal.read_line();
  terminal.move_to(15, 9);
  let address = terminal.read_line();
  let client = Client { name, address };

  fn get_amount(terminal: &mut Terminal) -> u32 {
    terminal.move_to(17, 12);
    let amount = terminal.read_line();

    let amount: u32 = match amount.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        terminal.move_to(17, 13);
        terminal.print("                                     │");
        return get_amount(terminal)
      },
    };
    amount
  }
  // define amount
  let amount = get_amount(terminal);

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
  let sale = Sale { client, product, amount };

  seller.new_sale(&sale);

  terminal.move_to(18, 16);

  terminal.print(&format!(
    "{}",
    sale.total()
  ));
  terminal.move_to(5, 19);

  terminal.print("Venda adicionada! deseja imprimir o recibo?");
  terminal.move_to(5, 20);
  let command = terminal.read_line();

  if command == String::from("sim") {
    receipt::load(terminal, &seller, &sale);
  }

}
