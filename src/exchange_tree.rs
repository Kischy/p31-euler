struct ExchangeTree {}

struct ExchangeNode {
    amount: u128,
    children: Vector<Box<ExchangeNode>>,
}

impl ExchangeNode {
    fn new(amount: u128) -> ExchangeNode {
        ExchangeNode {
            amount,
            children: Vector::new(),
        }
    }
}
