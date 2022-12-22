fn bigger<T: PartialOrd>(list: &[T]) -> &T {
  let mut bigger = &list[0];

  for item in list.iter() {
    if item > &bigger {
      bigger = item;
    }
  }

  bigger
}

fn main() {
  let numbers = vec![34, 50, 25, 100, 65];
  let result = bigger(&numbers);

  println!("{}", result)
}
