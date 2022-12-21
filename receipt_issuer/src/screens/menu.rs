use crate::terminal::Terminal;
use crossterm::{execute, cursor};
use interface_builder::{Application, Page};

pub fn load(terminal: &mut Terminal) -> u32 {
  let mut app = Application::new();
  app.home(Page::new(
    Some("[MENU] emissor de recibos"),
    vec![
      "[1] Nova venda",
      "[2] Consultar faturamento",
      "[3] Clientes cadastrado",
      "[4] Novo cliente",
      "[5] Produtos cadastrados",
      "[0] Encerrar"
    ],
    Some(vec!["Digite o número da opção desejada: "]),
    53, None
  ));
  app.run();

  execute!(
    terminal.stdout,
    cursor::MoveTo(40, 11),
    cursor::Show,
    cursor::EnableBlinking,
  ).expect("Não foi possível configurar o cursor");

  let option = terminal.read_line();

  let option: u32 = match option.trim().parse() {
    Ok(num) => num,
    Err(_) => return load(terminal),
  };

  if option > 5 {
    return load(terminal)
  }
  option
}
