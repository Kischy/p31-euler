pub struct CoinSum {
    sum: i128,
    coins: Vec<i128>,
}

impl CoinSum {
    pub fn new(sum: i128, coins: &[i128]) -> CoinSum {
        let mut coins = coins.to_vec();
        coins.sort_by(|a, b| b.cmp(a));
        CoinSum { sum, coins }
    }

    pub fn number_of_possible_ways(&self) -> i128 {
        CoinSum::num_of_ways(self.sum, &self.coins)
    }

    fn num_of_ways(sum: i128, coins: &[i128]) -> i128 {
        if sum < 0 {
            return 0;
        } else if sum == 0 {
            return 1;
        }

        let mut counter: i128 = 0;
        let mut coin_index = 0;

        while coin_index < coins.len() {
            counter += CoinSum::num_of_ways(sum - coins[coin_index], &coins[coin_index..]);
            coin_index += 1;
        }

        return counter;
    }
}

#[cfg(test)]
mod tests {
    use super::CoinSum;

    #[test]
    fn simple_coin_case_coins_1() {
        let cs = CoinSum::new(10, &[1]);
        assert_eq!(cs.number_of_possible_ways(), 1);
    }

    #[test]
    fn simple_coin_case_coins_2() {
        let cs = CoinSum::new(10, &[2]);
        assert_eq!(cs.number_of_possible_ways(), 1);
    }

    #[test]
    fn simple_coin_case_coins_3() {
        let cs = CoinSum::new(10, &[3]);
        assert_eq!(cs.number_of_possible_ways(), 0);
    }

    #[test]
    fn simple_coin_case_coins_1_2() {
        let cs = CoinSum::new(10, &[1, 2]);
        assert_eq!(cs.number_of_possible_ways(), 6);
        let cs = CoinSum::new(10, &[2, 1]);
        assert_eq!(cs.number_of_possible_ways(), 6);
    }

    #[test]
    fn simple_coin_case_coins_2_3() {
        let cs = CoinSum::new(10, &[3, 2]);
        assert_eq!(cs.number_of_possible_ways(), 2);
    }

    #[test]
    fn simple_coin_case_coins_1_3() {
        let cs = CoinSum::new(10, &[3, 1]);
        assert_eq!(cs.number_of_possible_ways(), 4);
    }

    #[test]
    fn simple_coin_case_coins_1_2_3() {
        let cs = CoinSum::new(10, &[3, 1, 2]);
        assert_eq!(cs.number_of_possible_ways(), 6 + 8);
    }
}
