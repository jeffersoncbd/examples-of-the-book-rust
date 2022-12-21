use crate::{
  terminal::Terminal,
  screens::seller::Seller,
  structures::Sale
};
use crossterm::{execute, cursor};
use interface_builder::{Application, Page};

pub fn load(terminal: &mut Terminal, seller: &Seller, sale: &Sale) {
  let client = match seller.clients.get(sale.client_code) {
    Some(client) => client,
    None => panic!("O cliente cadastrado na venda não existe")
  };

  let mut app = Application::new();
  app.home(Page::new(
    Some("[Recibo de venda]"),
    vec![
      &format!("Vendedor: {}", seller.name),
      &format!("Endereço: {}", seller.address),
      "",
      &format!("Cliente: {}", client.name),
      &format!("Endereço: {}", client.address),
      "",
      &format!("VALOR TOTAL: {}", sale.total()),
      "",
      "[Itens]",
      &format!(
        "[{}] {} - {} ... {}",
        sale.amount,
        sale.product.description,
        sale.product.value,
        sale.total()
      )
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
