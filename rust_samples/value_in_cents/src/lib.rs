pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        _ => 25,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_in_cents_works() {
        assert_eq!(value_in_cents(Coin::Penny), 1);
        assert_eq!(value_in_cents(Coin::Nickel), 5);
        assert_eq!(value_in_cents(Coin::Dime), 10);
        assert_eq!(value_in_cents(Coin::Quarter), 25);
    }
}
