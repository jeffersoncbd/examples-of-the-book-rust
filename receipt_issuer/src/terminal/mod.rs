mod progress_bar;
use std::io::{Stdout, stdout};

use crossterm::{terminal as crossterm_terminal, execute, style::Print, cursor, event::{self, Event, KeyEvent, KeyCode}, Result};

pub struct Terminal {
  pub stdout: Stdout,
  terminal_size: (u16, u16),
  pub end: bool
}
impl Terminal {
  fn new() -> Terminal {
    let stdout = stdout();
    let terminal_size = match crossterm_terminal::size() {
      Ok(size) => size,
      Err(_) => panic!("Erro ao tentar pegar o tamanho do terminal"),
    };
    Terminal { stdout, terminal_size, end: false }
  }
  pub fn print(&mut self, content: &str) {
    execute!(self.stdout, Print(content)).expect("Não foi possível imprimir");
  }
  pub fn clear(&mut self) {
    execute!(
      self.stdout,
      crossterm_terminal::Clear(crossterm_terminal::ClearType::All),
      cursor::MoveTo(0,0)
    ).expect("Não foi possível limpar o terminal");
  }
  pub fn move_to(&mut self, x: u16, y: u16) {
    execute!(self.stdout, cursor::MoveTo(x, y))
      .expect("Não foi possível mover o cursor");
  }
  pub fn fake_loading(&mut self) {
    execute!(
      self.stdout,
      crossterm_terminal::Clear(crossterm_terminal::ClearType::All),
      cursor::MoveTo(0, self.terminal_size.1 / 2),
    ).expect("Não foi possível limpar o terminal e mover o cursos para exibir a ProgressBar");

    progress_bar::run(100, 1);

    self.clear();
  }
  pub fn read_line(&mut self) -> String {
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
}

pub fn new() -> Terminal {
  Terminal::new()
}
