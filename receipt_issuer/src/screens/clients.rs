use crossterm::{execute, cursor};
use interface_builder::{Page, Application};

use crate::{terminal::Terminal, screens::seller::Seller, structures::Client};

fn parse_clients(clients: &Vec<Client>) -> Vec<String> {
  let mut parsed_clients = Vec::new();

  for (i, client) in clients.iter().enumerate() {
    if i != 0 {
      parsed_clients.push(String::from(""))
    }
    parsed_clients.push(format!("Código: {}", i));
    parsed_clients.push(format!("Nome: {}", client.name));
    parsed_clients.push(format!("Endereço: {}", client.address));
  }

  parsed_clients
}

pub fn load(terminal: &mut Terminal, seller: &mut Seller) {
  let mut app = Application::new();

  let parsed = parse_clients(&seller.clients);
  let parsed_clients = if seller.clients.len() > 0 {
    parsed.iter().map(|s| s as &str).collect()
  } else {
    vec!["Ainda não há clientes cadastrados"]
  };

  app.home(Page::new(
    Some("[Clientes cadastrados] emissor de recibos"),
    parsed_clients,
    Some(vec!["Pressione enter para voltar..."]),
    53, None
  ));
  app.run();

  execute!(
    terminal.stdout,
    cursor::Hide,
  ).expect("Não foi possível imprimir a lista de clientes");

  terminal.read_line();
}
