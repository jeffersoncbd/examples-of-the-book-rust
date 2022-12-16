use crate::{terminal::Terminal, screens::seller::Seller};
use crossterm::{execute, style::Print, cursor};

pub fn load(terminal: &mut Terminal, seller: &Seller) {
  terminal.clear();
  execute!(
    terminal.stdout,
    Print("\n\n"),
    Print("  ╭─ [Faturamento] emissor de recibos ────────────────╮\n"),
    Print("  │                                                   │\n"),
    Print("  │  Vendedor:                                        │\n"),
    Print("  │  Endereço:                                        │\n"),
    Print("  │  Total vendido:                                   │\n"),
    Print("  │                                                   │\n"),
    Print("  ╰───────────────────────────────────────────────────╯\n"),
    Print("     Pressione enter para voltar..."),
    cursor::Hide,
  ).expect("Não foi possível imprimir o formulário de faturamento");

  terminal.move_to(15, 4);
  terminal.print(seller.name.trim());
  terminal.move_to(15, 5);
  terminal.print(seller.address.trim());
  terminal.move_to(20, 6);
  terminal.print(&seller.invoicing.to_string());

  terminal.read_line();
}
