use crossterm::{execute, style::Print, cursor};

use crate::{terminal::Terminal, screens::seller::Seller};

pub fn load(terminal: &mut Terminal, seller: &mut Seller) {
  terminal.clear();

  terminal.print("\n\n  ╭─ [Lista de Clientes] emissor de recibos ──────────╮\n");

  if seller.clients.len() > 0 {
    for (code, client) in seller.clients.iter().enumerate() {
      execute!(
        terminal.stdout,
        Print("  │                                                   │\n"),
        Print("  │  Code:                                            │"),
        cursor::MoveToColumn(11),
        Print(format!("{}", code)),
        cursor::MoveToNextLine(1),
        Print("  │  Nome:                                            │"),
        cursor::MoveToColumn(11),
        Print(&client.name),
        cursor::MoveToNextLine(1),
        Print("  │  Endereço:                                        │"),
        cursor::MoveToColumn(15),
        Print(&client.address),
        cursor::MoveToNextLine(1),
      ).expect("Não foi possível imprimir a lista de clientes");
    }
  } else {
    execute!(
      terminal.stdout,
      Print("  │                                                   │\n"),
      Print("  │  Ainda não há clientes cadastrados                │\n")
    ).expect("Não foi possível imprimir a lista de clientes");
  }

  execute!(
    terminal.stdout,
    Print("  │                                                   │\n"),
    Print("  ╰───────────────────────────────────────────────────╯\n"),
    Print("     Pressione enter para voltar..."),
    cursor::Hide,
  ).expect("Não foi possível imprimir a lista de clientes");

  terminal.read_line();
}
