use rust_decimal::{Decimal, prelude::{FromPrimitive, ToPrimitive}};

pub struct Client {
  pub name: String,
  pub address: String
}

pub struct Product {
  pub description: String,
  pub value: f64
}

pub struct Sale {
  pub client_code: usize,
  pub product: Product,
  pub amount: u32
}
impl Sale {
  pub fn total(&self) -> f64 {
    let decimal_amount = Decimal::from_f64(self.amount as f64).unwrap();
    let decimal_value = Decimal::from_f64(self.product.value).unwrap();
    let total = decimal_amount * decimal_value;
    total.to_f64().unwrap()
  }
}
