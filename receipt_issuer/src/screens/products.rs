use crossterm::{execute, cursor};
use interface_builder::{Page, Application};

use crate::{terminal::Terminal, screens::seller::Seller, structures::Product};

fn parse_products(products: &Vec<Product>) -> Vec<String> {
  let mut parsed_products = Vec::new();

  for (i, product) in products.iter().enumerate() {
    if i != 0 {
      parsed_products.push(String::from(""))
    }
    parsed_products.push(format!("Código: {}", i));
    parsed_products.push(format!("Descrição: {}", product.description));
    parsed_products.push(format!("Valor: {}", product.value));
  }

  parsed_products
}

pub fn load(terminal: &mut Terminal, seller: &mut Seller) {
  let mut app = Application::new();

  let parsed = parse_products(&seller.products);
  let parsed_products = if seller.products.len() > 0 {
    parsed.iter().map(|s| s as &str).collect()
  } else {
    vec!["Ainda não há produtos cadastrados"]
  };

  app.home(Page::new(
    Some("[Produtos cadastrados] emissor de recibos"),
    parsed_products,
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
