use apples_core::cards::red_card::RedCard;

#[tokio::main]
async fn main() {
    let redcard = RedCard::new("abc".to_string(), "bcd".to_string(), 130);
    println!("Hello, world! {}", redcard);
}
