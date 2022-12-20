use crossterm::{execute, style::Print, cursor};

use crate::{terminal::Terminal, screens::seller::Seller, structures::Client};

pub fn load(terminal: &mut Terminal, seller: &mut Seller) {
  terminal.clear();

  execute!(
    terminal.stdout,
    Print("\n\n"),
    Print("  ╭─ [Novo cliente] emissor de recibos ───────────────╮\n"),
    Print("  │                                                   │\n"),
    Print("  │  Nome:                                            │\n"),
    Print("  │  Endereço:                                        │\n"),
    Print("  │                                                   │\n"),
    Print("  ╰───────────────────────────────────────────────────╯\n"),
    cursor::MoveTo(11, 4),
    cursor::Show,
    cursor::EnableBlinking
  ).expect("Não foi possível imprimir o formulário de novo cliente");

  let name = terminal.read_line();
  terminal.move_to(15, 5);
  let address = terminal.read_line();

  execute!(
    terminal.stdout,
    cursor::MoveTo(5, 8),
    Print("Digite \"sim\" se os dados estiverem corretos\n"),
    Print("     Ou \"cancelar\" para voltar"),
    cursor::MoveTo(5, 10)
  ).expect("Não foi possível imprimir o feedback de confirmação");



  let decision = terminal.read_line();
  if decision == String::from("cancelar") {
    return
  }
  if decision != String::from("sim") {
    load(terminal, seller)
  }

  seller.clients.push(Client { name, address })
}
