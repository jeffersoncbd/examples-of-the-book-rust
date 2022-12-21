use crossterm::{execute, style::Print, cursor};
use interface_builder::{Application, Page};

use crate::{terminal::Terminal, screens::seller::Seller, structures::Client};

pub fn load(terminal: &mut Terminal, seller: &mut Seller) {
  let mut app = Application::new();
  app.home(Page::new(
    Some("[Novo cliente] emissor de recibos"),
    vec![
      "Nome:",
      "Endereço:",
    ],
    None,
    53, None
  ));
  app.run();

  execute!(
    terminal.stdout,
    cursor::MoveTo(11, 3),
    cursor::Show,
    cursor::EnableBlinking
  ).expect("Não foi possível configurar o cursor4
  4");

  let name = terminal.read_line();
  terminal.move_to(15, 4);
  let address = terminal.read_line();

  execute!(
    terminal.stdout,
    cursor::MoveTo(5, 7),
    Print("Digite \"sim\" se os dados estiverem corretos\n"),
    Print("     Ou \"cancelar\" para voltar"),
    cursor::MoveTo(5, 9)
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
