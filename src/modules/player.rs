use crate::modules::consts::{SCORE_REPEATING_PRIME_COST_MODIFIER, SCORE_NEW_PRIME_COST_MODIFIER, WEALTH_NEW_PRIME_COST_MODIFIER};
pub struct Player {
    name: String,
    score: u64,
    wealth: u64,
    collected_primes: Vec<u64>,
}

impl Player {

    pub fn new() -> Player {
        Player { name: "".to_string(), score: 0, wealth: 0, collected_primes: Vec::new() }
    }

    /// Проверят, собирал ли игрок уже переданное простое число.
    /// В случае если простое число не было собрано, возввращается false и число заносится в вектор собранных чисел
    /// В случае если простое число было собрано, возвращается true
    pub fn is_prime_collected(&mut self, prime : u64) -> bool {
        if self.collected_primes.contains(&prime) {
            true
        } else {
            //println!("Wow, you have collected {}", prime);
            self.collected_primes.push(prime);
            false
        }
    }

    /// Увеличивает количество очков игрока в зависимости от переданного в функцию простого числа
    /// При просчёте очков применяет модификаторы константы для новых и повторяющихся простых чисел
    pub fn add_score_wealth(&mut self, prime : u64) {
        //println!("adding score for {}", prime);
        if self.is_prime_collected(prime) {
            self.score += (prime as f64 * SCORE_REPEATING_PRIME_COST_MODIFIER).ceil() as u64;
        } else {
            self.score += (prime as f64 * SCORE_NEW_PRIME_COST_MODIFIER).ceil() as u64;
            self.wealth += (prime as f64 * WEALTH_NEW_PRIME_COST_MODIFIER).ceil() as u64;
        }
    }

    fn print_colledted(&self) {
        let mut first = true;
        for prime in &self.collected_primes {
            if first {
                first = !first;
                print!("{prime}");
            } else {
                print!(", {prime}");
            }
        }
    }
}