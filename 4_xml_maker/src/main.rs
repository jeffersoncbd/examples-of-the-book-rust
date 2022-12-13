use std::io;

fn main() {
  println!("Cole o conteúdo JSON:");
  let mut json = String::new();
  io::stdin().read_line(&mut json)
    .expect("Falha ao ler json!");
  // let json = String::from("{ \"id\": 1, \"name\":\"Johnson, Smith, and Jones Co.\", \"amount\":345.33,    \"Remark\":\"Pays on time\" }");;

  let json = json.trim().to_string();

  println!("\n\n<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n<root>");
  convert_json(&json);
  println!("</root>");

  let mut any = String::new();
  io::stdin().read_line(&mut any).expect("any");
}

fn convert_json(json: &str) {
  convert_object(json[1..json.len() - 1].trim(), 0);
}

fn convert_object(object: &str, jump: usize) {
  let key_map = get_first_key(object, jump);
  let content_map = get_first_content(object, jump);

  let key = &object[(key_map.0)..(key_map.1)];
  let content = &object[(content_map.0)..(content_map.1)];

  println!("  <{}>{}</{}>", key, content.trim(), key);

  let comma = find_char(object, b',', content_map.1);
  if comma < object.len() {
    convert_object(object, comma + 1);
  }
}

fn find_char(object: &str, find: u8, jump: usize) -> usize {
  let bytes = object[jump..].as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == find {
      return i + jump
    }
  }
  object.len()
}

fn get_first_key(object: &str, jump: usize) -> (usize, usize) {
  let start  = find_char(object, b'"', jump) + 1;
  let end = find_char(object, b'"', start);
  (start, end)
}

fn get_first_content(object: &str, jump: usize) -> (usize, usize) {
  let colon = find_char(object, b':', jump);
  let first_dq = find_char(object, b'"', colon);
  let comma = find_char(object, b',', colon);

  if first_dq < comma {
    let second_dq = find_char(object, b'"', first_dq + 1);
    return (first_dq + 1, second_dq)
  }

  (colon + 1, comma)
}
