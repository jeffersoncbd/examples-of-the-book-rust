use crate::{terminal::Terminal, screens::seller::Seller};
use crossterm::{execute, cursor};
use interface_builder::{Page, Application};

pub fn load(terminal: &mut Terminal, seller: &Seller) {
  let mut app = Application::new();
  app.home(Page::new(
    Some("[Faturamento] emissor de recibos"),
    vec![
      &format!("Vendedor: {}", seller.name),
      &format!("Endereço: {}", seller.address),
      &format!("Total vendido: {}", seller.invoicing),
    ],
    Some(vec!["Pressione enter para voltar..."]),
    53, None
  ));
  app.run();

  execute!(
    terminal.stdout,
    cursor::Hide,
  ).expect("Não foi possível configurar o cursor");

  terminal.read_line();
}
