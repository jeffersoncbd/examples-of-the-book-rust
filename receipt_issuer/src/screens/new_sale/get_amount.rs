use crate::terminal::Terminal;

pub fn get_amount(terminal: &mut Terminal) -> u32 {
  terminal.move_to(17, 13);
  let amount = terminal.read_line();

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
