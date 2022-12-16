use crate::terminal::Terminal;
use crossterm::{execute, style::Print, cursor};

pub fn load(terminal: &mut Terminal) -> u32 {
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

  let option = terminal.read_line();

  let option: u32 = match option.trim().parse() {
    Ok(num) => num,
    Err(_) => return load(terminal),
  };

  if option > 2 {
    return load(terminal)
  }
  option
}
