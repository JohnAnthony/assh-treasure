extern crate rand;

use rand::Rng;

#[derive(Debug)]
enum CoinType {
    Copper,
    Silver,
    Electrum,
    Gold,
    Platinum
}

#[derive(Debug)]
enum Treasure {
    Coins { coin_type: CoinType, quantity: u64 },
}

fn dice_roll(quant: u64, sz: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let mut ret = 0;

    for _ in 0..quant {
        ret += rng.gen::<u64>()%sz + 1;
    }

    ret
}

fn percent_chance(chance: u8) -> bool {
    let mut rng = rand::thread_rng();
    rng.gen::<u8>()%100 < chance
}

fn main() {
    let code = 'p';
    let mut treasure = Vec::new();

    match code {
        'j' => treasure.push(Treasure::Coins {
            coin_type: CoinType::Copper,
            quantity: dice_roll(4, 6)
        }),
        'k' => treasure.push(Treasure::Coins {
            coin_type: CoinType::Silver,
            quantity: dice_roll(4, 4)
        }),
        'l' => treasure.push(Treasure::Coins {
            coin_type: CoinType::Electrum,
            quantity: dice_roll(3, 4)
        }),
        'm' => treasure.push(Treasure::Coins {
            coin_type: CoinType::Gold,
            quantity: dice_roll(1, 8)
        }),
        'n' => treasure.push(Treasure::Coins {
            coin_type: CoinType::Platinum,
            quantity: dice_roll(1, 4) + 1
        }),
        'o' => {
            if percent_chance(25) {
                treasure.push(Treasure::Coins {
                    coin_type: CoinType::Copper,
                    quantity: dice_roll(2, 4)
                })
            }
            if percent_chance(20) {
                treasure.push(Treasure::Coins {
                    coin_type: CoinType::Silver,
                    quantity: dice_roll(1, 6)
                })
            }
        },
        'p' => {
            if percent_chance(30) {
                treasure.push(Treasure::Coins {
                    coin_type: CoinType::Silver,
                    quantity: dice_roll(2, 6)
                })
            }
            if percent_chance(20) {
                treasure.push(Treasure::Coins {
                    coin_type: CoinType::Electrum,
                    quantity: dice_roll(1, 4)
                })
            }
        },
        'y' => {
            if percent_chance(70) {
                treasure.push(Treasure::Coins {
                    coin_type: CoinType::Gold,
                    quantity: dice_roll(4, 12)
                })
            }
        }
        _ => panic!("Unknown treasure code")
    };

    println!("TREASURE");
    println!("========");
    println!("{:?}", treasure);
}
