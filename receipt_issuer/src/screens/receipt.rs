use crate::{
  terminal::Terminal,
  screens::seller::Seller,
  structures::Sale
};
use crossterm::{execute, style::Print, cursor};

pub fn load(terminal: &mut Terminal, seller: &Seller, sale: &Sale) {
  terminal.clear();
  execute!(
    terminal.stdout,
    Print("\n\n"),
    Print("  ╭─ [Recibo de venda] ───────────────────────────────╮\n"),
    Print("  │                                          ╭──────╮ │\n"),
    Print("  │  Vendedor:                               │bonita│ │\n"),
    cursor::MoveTo(15,4),
    Print(format!("{}", seller.name)),
    cursor::MoveTo(0,5),
    Print("  │  Endereço:                               │ essa │ │\n"),
    cursor::MoveTo(15,5),
    Print(format!("{}", seller.address)),
    cursor::MoveTo(0,6),
    Print("  │                                          │ logo │ │\n"),
    Print("  │  Cliente:                                ╰──────╯ │\n"),
    cursor::MoveTo(14,7),
    Print(format!("{}", sale.client.name)),
    cursor::MoveTo(0,8),
    Print("  │  Endereço:                                        │\n"),
    cursor::MoveTo(15,8),
    Print(format!("{}", sale.client.address)),
    cursor::MoveTo(0,9),
    Print("  │                                                   │\n"),
    cursor::MoveTo(
      39 - (sale.total().to_string().len() as u16),
      9
    ),
    Print(format!("VALOR TOTAL: {}\n", sale.total())),
    Print("  │                                                   │\n"),
    Print("  │  [Itens]                                          │\n"),
    Print("  │                                                   │\n"),
    Print("  │  ...............................................  │\n"),
    cursor::MoveTo(5,13),
    Print(format!(
      "[{}] {} - {} ",
      sale.amount,
      sale.product.description,
      sale.product.value
    )),
    cursor::MoveTo(
      51 - (sale.total().to_string().len() as u16),
      13
    ),
    Print(format!(" {}", sale.total())),
    cursor::MoveTo(0,14),
    Print("  │                                                   │\n"),
    Print("  ╰───────────────────────────────────────────────────╯\n"),
    Print("     Pressione enter para voltar..."),
    cursor::Hide,
  ).expect("Não foi possível imprimir o Recibo de venda");
  terminal.read_line();
}
