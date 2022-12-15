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
  invoicing: f64
}
impl Seller {
  fn add_sale(&mut self, invoice: &Invoice) {
    self.invoicing += invoice.total()
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
struct Invoice {
  seller: Seller,
  client: Client,
  product: Product,
  amount_of_product: u32,
}
impl Invoice {
  fn total(&self) -> f64 {
    // 5 * 5.24
    let decimal_amount = Decimal::from_f64(self.amount_of_product as f64).unwrap();
    let value_amount = Decimal::from_f64(self.product.value).unwrap();
    let total = decimal_amount * value_amount;
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
      Err(_) => panic!(""),
    };
    Terminal { stdout, terminal_size, end: false }
  }
  fn print(&mut self, content: &str) -> Result<()> {
    execute!(
      self.stdout,
      Print(content),
    )?;
    Ok(())
  }
  fn clear(&mut self) -> Result<()> {
    execute!(
      self.stdout,
      terminal::Clear(terminal::ClearType::All),
      cursor::MoveTo(0,0)
    )?;
    Ok(())
  }
  fn move_to(&mut self, x: u16, y: u16) -> Result<()> {
    execute!(
      self.stdout,
      cursor::MoveTo(x, y)
    )?;
    Ok(())
  }
  fn fake_loading(&mut self) -> Result<()> {
    execute!(
      self.stdout,
      terminal::Clear(terminal::ClearType::All),
      cursor::MoveTo(0, self.terminal_size.1 / 2),
    )?;

    let count = 100;
    let mut pb = ProgressBar::new(count);
    pb.format("[=>-]");
    pb.show_speed = false;
    pb.show_time_left = false;
    for _ in 0..count {
      pb.inc();
      thread::sleep(Duration::from_millis(1));
    }

    self.clear()?;

    Ok(())
  }
}

fn main() -> Result<()> {
  let mut terminal = Terminal::new();

  terminal.fake_loading()?;

  let mut seller = define_seller(&mut terminal)?;

  terminal.fake_loading()?;

  loop {
    application(&mut terminal, &mut seller)?;
    if terminal.end {
      break
    }
  }

  terminal.clear()?;
  Ok(())
}

fn application(terminal: &mut Terminal, seller: &mut Seller) -> Result<()> {
  let option = menu(terminal)?;
  match option {
    1 => new_sale(terminal, seller)?,
    2 => print_billing(terminal, seller)?,
    0 => { terminal.end = true; },
    _ => panic!("Unexpected option"),
  }
  Ok(())
}

fn read_line() -> Result<String> {
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

fn define_seller(terminal: &mut Terminal) -> Result<Seller> {
  terminal.clear()?;
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
  )?;

  let name = read_line()?;
  terminal.move_to(14, 7)?;
  let address = read_line()?;

  terminal.move_to(4, 10)?;
  println!("Se estes dados estiverem corretos, digite \"sim\":");
  terminal.move_to(4, 11)?;
  let confirmation = read_line()?;
  if confirmation != String::from("sim") {
    return define_seller(terminal);
  }

  Ok(Seller { name, address, invoicing: 0.0 })
}

fn menu(terminal: &mut Terminal) -> Result<u32> {
  terminal.clear()?;
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
  )?;

  let option = read_line()?;

  let option: u32 = match option.trim().parse() {
    Ok(num) => num,
    Err(_) => return menu(terminal),
  };

  if option > 2 {
    return menu(terminal)
  }
  Ok(option)
}

fn new_sale(terminal: &mut Terminal, seller: &mut Seller) -> Result<()> {
  terminal.clear()?;
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
    Print("  │  Descrição:                                       │\n"),
    Print("  │  Quantidade:                                      │\n"),
    Print("  │  Valor unitário:                                  │\n"),
    Print("  │                                                   │\n"),
    Print("  │  Valor TOTAL:                                     │\n"),
    Print("  │                                                   │\n"),
    Print("  ╰───────────────────────────────────────────────────╯\n"),
    cursor::EnableBlinking,
  )?;

  terminal.move_to(15, 4)?;
  terminal.print(seller.name.trim())?;
  terminal.move_to(15, 5)?;
  terminal.print(seller.address.trim())?;

  terminal.move_to(11, 8)?;
  let name = read_line()?;
  terminal.move_to(15, 9)?;
  let address = read_line()?;
  let client = Client { name, address };

  terminal.move_to(16, 12)?;
  let description = read_line()?;

  fn get_amount(terminal: &mut Terminal) -> Result<u32> {
    terminal.move_to(17, 13)?;
    let amount = read_line()?;

    let amount: u32 = match amount.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        terminal.move_to(17, 13)?;
        terminal.print("                                     │")?;
        return Ok(get_amount(terminal)?)
      },
    };
    Ok(amount)
  }
  let amount = get_amount(terminal)?;

  fn get_value(terminal: &mut Terminal) -> Result<f64> {
    terminal.move_to(21, 14)?;
    let value = read_line()?;

    let value: f64 = match value.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        terminal.move_to(21, 14)?;
        terminal.print("                                 │")?;
        return Ok(get_value(terminal)?)
      },
    };
    Ok(value)
  }
  let value = get_value(terminal)?;

  let product = Product { description, value };

  let invoice = Invoice {
    seller: Seller {
      name: seller.name.clone(),
      address: seller.address.clone(),
      invoicing: seller.invoicing.clone()
    },
    client,
    product,
    amount_of_product: amount
  };

  seller.add_sale(&invoice);

  terminal.move_to(18, 16)?;

  terminal.print(&format!(
    "{}",
    (Decimal::from_f64(amount as f64).unwrap() * Decimal::from_f64(value).unwrap()).to_f64().unwrap()
  ))?;
  terminal.move_to(5, 19)?;

  terminal.print("Venda adicionada! deseja imprimir o recibo?")?;
  terminal.move_to(5, 20)?;
  let command = read_line()?;

  if command == String::from("sim") {
    print_receipt(terminal, &invoice)?;
  }

  Ok(())
}

fn print_billing(terminal: &mut Terminal, seller: &Seller) -> Result<()> {
  terminal.clear()?;
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
  )?;

  terminal.move_to(15, 4)?;
  terminal.print(seller.name.trim())?;
  terminal.move_to(15, 5)?;
  terminal.print(seller.address.trim())?;
  terminal.move_to(20, 6)?;
  terminal.print(&seller.invoicing.to_string())?;

  read_line()?;
  Ok(())
}

fn print_receipt(terminal: &mut Terminal, invoice: &Invoice) -> Result<()> {
  terminal.clear()?;
  execute!(
    terminal.stdout,
    Print("\n\n"),
    Print("  ╭─ [Recibo de venda] ───────────────────────────────╮\n"),
    Print("  │                                          ╭──────╮ │\n"),
    Print("  │  Vendedor:                               │bonita│ │\n"),
    cursor::MoveTo(15,4),
    Print(format!("{}", invoice.seller.name)),
    cursor::MoveTo(0,5),
    Print("  │  Endereço:                               │ essa │ │\n"),
    cursor::MoveTo(15,5),
    Print(format!("{}", invoice.seller.address)),
    cursor::MoveTo(0,6),
    Print("  │                                          │ logo │ │\n"),
    Print("  │  Cliente:                                ╰──────╯ │\n"),
    cursor::MoveTo(14,7),
    Print(format!("{}", invoice.client.name)),
    cursor::MoveTo(0,8),
    Print("  │  Endereço:                                        │\n"),
    cursor::MoveTo(15,8),
    Print(format!("{}", invoice.client.address)),
    cursor::MoveTo(0,9),
    Print("  │                                                   │\n"),
    cursor::MoveTo(
      39 - (invoice.total().to_string().len() as u16),
      9
    ),
    Print(format!("VALOR TOTAL: {}\n", invoice.total())),
    Print("  │                                                   │\n"),
    Print("  │  [Itens]                                          │\n"),
    Print("  │                                                   │\n"),
    Print("  │  ...............................................  │\n"),
    cursor::MoveTo(5,13),
    Print(format!(
      "[{}] {} - {} ",
      invoice.amount_of_product,
      invoice.product.description,
      invoice.product.value
    )),
    cursor::MoveTo(
      51 - (invoice.total().to_string().len() as u16),
      13
    ),
    Print(format!(" {}", invoice.total())),
    cursor::MoveTo(0,14),
    Print("  │                                                   │\n"),
    Print("  ╰───────────────────────────────────────────────────╯\n"),
    Print("     Pressione enter para voltar..."),
    cursor::Hide,
  )?;
  read_line()?;
  Ok(())
}
