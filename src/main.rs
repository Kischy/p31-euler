mod coin_sums;
use coin_sums::CoinSum;

fn main() {
    println!("Calculation started");

    let cs = CoinSum::new(200, &[1,2,5,10,20,50,100,200]);
    let answer_p31 = cs.number_of_possible_ways();


    println!(
        "The answer to problem 31 of project Euler is {}.",
        answer_p31
    );
}
