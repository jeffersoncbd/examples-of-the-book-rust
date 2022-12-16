extern crate pbr;

use rust_decimal::{Decimal, prelude::{FromPrimitive, ToPrimitive}};

use pbr::ProgressBar;
use std::{io::{stdout, Stdout}, thread, time::Duration};
use crossterm::{
  execute, terminal, Result, cursor, style::Print, event::{Event, KeyEvent, self, KeyCode},
};

struct Seller {
  name: String,
  address: String,
  invoicing: f64,
}
impl Seller {
  fn new(name: String, address: String) -> Seller {
    Seller { name, address, invoicing: 0.0 }
  }
  fn new_sale(&mut self, sale: &Sale) {
    self.invoicing += sale.total();
  }
}
struct Client {
  name: String,
  address: String
}
struct Product {
  description: String,
  value: f64
}
struct Sale {
  client: Client,
  product: Product,
  amount: u32
}
impl Sale {
  fn total(&self) -> f64 {
    let decimal_amount = Decimal::from_f64(self.amount as f64).unwrap();
    let decimal_value = Decimal::from_f64(self.product.value).unwrap();
    let total = decimal_amount * decimal_value;
    total.to_f64().unwrap()
  }
}

struct Terminal {
  stdout: Stdout,
  terminal_size: (u16, u16),
  end: bool
}
impl Terminal {
  fn new() -> Terminal {
    let stdout = stdout();
    let terminal_size = match terminal::size() {
      Ok(size) => size,
      Err(_) => panic!("Erro ao tentar pegar o tamanho do terminal"),
    };
    Terminal { stdout, terminal_size, end: false }
  }
  fn print(&mut self, content: &str) {
    execute!(self.stdout, Print(content)).expect("Não foi possível imprimir");
  }
  fn clear(&mut self) {
    execute!(
      self.stdout,
      terminal::Clear(terminal::ClearType::All),
      cursor::MoveTo(0,0)
    ).expect("Não foi possível limpar o terminal");
  }
  fn move_to(&mut self, x: u16, y: u16) {
    execute!(self.stdout, cursor::MoveTo(x, y))
      .expect("Não foi possível mover o cursor");
  }
  fn fake_loading(&mut self) {
    execute!(
      self.stdout,
      terminal::Clear(terminal::ClearType::All),
      cursor::MoveTo(0, self.terminal_size.1 / 2),
    ).expect("Não foi possível limpar o terminal e mover o cursos para exibir a ProgressBar");

    let count = 100;
    let mut pb = ProgressBar::new(count);
    pb.format("[=>-]");
    pb.show_speed = false;
    pb.show_time_left = false;
    for _ in 0..count {
      pb.inc();
      thread::sleep(Duration::from_millis(1));
    }

    self.clear();
  }
}

fn main() {
  let mut terminal = Terminal::new();

  terminal.fake_loading();

  let mut seller = define_seller(&mut terminal);

  terminal.fake_loading();

  loop {
    application(&mut terminal, &mut seller);
    if terminal.end {
      break
    }
  }

  terminal.clear();
}

fn application(terminal: &mut Terminal, seller: &mut Seller) {
  let option = menu(terminal);
  match option {
    1 => new_sale(terminal, seller),
    2 => print_billing(terminal, seller),
    0 => { terminal.end = true; },
    _ => panic!("Unexpected option"),
  }
}

fn read_line() -> String {
  fn read() -> Result<String> {
    let mut line = String::new();
    while let Event::Key(KeyEvent { code, .. }) = event::read()? {
      match code {
        KeyCode::Enter => {
          break;
        }
        KeyCode::Char(c) => {
          line.push(c);
        }
        _ => {}
      }
    }
    Ok(line)
  }
  read().expect("Não foi possível ler a entrada do usuário")
}

fn define_seller(terminal: &mut Terminal) -> Seller {
  terminal.clear();
  execute!(
    terminal.stdout,
    Print("\n\n"),
    Print("  ╭───────── Bem vindo ao emissor de recibos ─────────╮\n"),
    Print("  │                                                   │\n"),
    Print("  │ Antes de começar o vendedor deve ser cadastrado:  │\n"),
    Print("  │                                                   │\n"),
    Print("  │ Nome:                                             │\n"),
    Print("  │ Endereço:                                         │\n"),
    Print("  │                                                   │\n"),
    Print("  ╰───────────────────────────────────────────────────╯"),
    cursor::MoveTo(10, 6),
    cursor::Show,
    cursor::EnableBlinking,
  ).expect("Não foi possível imprimir o formulário de cadastro do vendedor");

  let name = read_line();
  terminal.move_to(14, 7);
  let address = read_line();

  terminal.move_to(4, 10);
  println!("Se estes dados estiverem corretos, digite \"sim\":");
  terminal.move_to(4, 11);
  let confirmation = read_line();
  if confirmation != String::from("sim") {
    return define_seller(terminal);
  }

  Seller::new(name, address)
}

fn menu(terminal: &mut Terminal) -> u32 {
  terminal.clear();
  execute!(
    terminal.stdout,
    Print("\n\n"),
    Print("  ╭─ [MENU] emissor de recibos ───────────────────────╮\n"),
    Print("  │                                                   │\n"),
    Print("  │  Selecione uma das opções abaixo:                 │\n"),
    Print("  │                                                   │\n"),
    Print("  │ [1] Nova venda                                    │\n"),
    Print("  │ [2] Consultar faturamento                         │\n"),
    Print("  │ [0] Encerrar sistema                              │\n"),
    Print("  │                                                   │\n"),
    Print("  ╰───────────────────────────────────────────────────╯"),
    cursor::MoveTo(38, 4),
    cursor::Show,
    cursor::EnableBlinking,
  ).expect("Não foi possível imprimir o formulário do MENU");

  let option = read_line();

  let option: u32 = match option.trim().parse() {
    Ok(num) => num,
    Err(_) => return menu(terminal),
  };

  if option > 2 {
    return menu(terminal)
  }
  option
}

fn new_sale(terminal: &mut Terminal, seller: &mut Seller) {
  terminal.clear();
  execute!(
    terminal.stdout,
    Print("\n\n"),
    Print("  ╭─ [Nova venda] emissor de recibos ─────────────────╮\n"),
    Print("  │                                                   │\n"),
    Print("  │  Vendedor:                                        │\n"),
    Print("  │  Endereço:                                        │\n"),
    Print("  │                                                   │\n"),
    Print("  │  [Dados do cliente]                               │\n"),
    Print("  │  Nome:                                            │\n"),
    Print("  │  Endereço:                                        │\n"),
    Print("  │                                                   │\n"),
    Print("  │  [Itens]                                          │\n"),
    Print("  │  Quantidade:                                      │\n"),
    Print("  │  Descrição:                                       │\n"),
    Print("  │  Valor unitário:                                  │\n"),
    Print("  │                                                   │\n"),
    Print("  │  Valor TOTAL:                                     │\n"),
    Print("  │                                                   │\n"),
    Print("  ╰───────────────────────────────────────────────────╯\n"),
    cursor::EnableBlinking,
  ).expect("Não foi possível imprimir o formulário de nova venda");

  // print seller data
  terminal.move_to(15, 4);
  terminal.print(seller.name.trim());
  terminal.move_to(15, 5);
  terminal.print(seller.address.trim());

  // define client data
  terminal.move_to(11, 8);
  let name = read_line();
  terminal.move_to(15, 9);
  let address = read_line();
  let client = Client { name, address };

  fn get_amount(terminal: &mut Terminal) -> u32 {
    terminal.move_to(17, 12);
    let amount = read_line();

    let amount: u32 = match amount.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        terminal.move_to(17, 13);
        terminal.print("                                     │");
        return get_amount(terminal)
      },
    };
    amount
  }
  // define amount
  let amount = get_amount(terminal);

  fn get_value(terminal: &mut Terminal) -> f64 {
    terminal.move_to(21, 14);
    let value = read_line();

    let value: f64 = match value.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        terminal.move_to(21, 14);
        terminal.print("                                 │");
        return get_value(terminal)
      },
    };
    value
  }
  // define product
  terminal.move_to(16, 13);
  let description = read_line();
  let value = get_value(terminal);
  let product = Product { description, value };

  // define sale
  let sale = Sale { client, product, amount };

  seller.new_sale(&sale);

  terminal.move_to(18, 16);

  terminal.print(&format!(
    "{}",
    sale.total()
  ));
  terminal.move_to(5, 19);

  terminal.print("Venda adicionada! deseja imprimir o recibo?");
  terminal.move_to(5, 20);
  let command = read_line();

  if command == String::from("sim") {
    print_receipt(terminal, &seller, &sale);
  }

}

fn print_billing(terminal: &mut Terminal, seller: &Seller) {
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

  read_line();
}

fn print_receipt(terminal: &mut Terminal, seller: &Seller, sale: &Sale) {
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
  read_line();
}
