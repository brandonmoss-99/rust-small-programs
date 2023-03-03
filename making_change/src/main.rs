
/// Calculate number of coins needed to change an amount of currency, while
/// giving away the least amount of coins
fn calc_coins_used(amount: i32) -> i32 {
    let coins = [500, 100, 25, 10, 5, 1];
    let mut amount_left = amount;
    let mut coins_used = 0;

    for coin in coins {
        while amount_left - coin >= 0 {
            coins_used += 1;
            amount_left -= coin;
        }
    }
    coins_used
}

fn main() {
    let currency_1 = 0;
    let currency_2 = 12;
    let currency_3 = 468;
    let currency_4 = 123456;
    println!("Used {} coins to change {}", calc_coins_used(currency_1), currency_1);
    println!("Used {} coins to change {}", calc_coins_used(currency_2), currency_2);
    println!("Used {} coins to change {}", calc_coins_used(currency_3), currency_3);
    println!("Used {} coins to change {}", calc_coins_used(currency_4), currency_4);
}
