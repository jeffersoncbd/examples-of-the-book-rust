use crate::terminal::Terminal;

pub fn get(terminal: &mut Terminal) -> u32 {
  terminal.move_to(17, 12);
  let amount = terminal.read_line();

  let amount: u32 = match amount.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      terminal.move_to(17, 12);
      terminal.print("                                     │");
      return get(terminal)
    },
  };
  amount
}
