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
struct Coins {
    coin_type: CoinType,
    quantity: u64,
}

#[derive(Debug)]
enum Treasure {
    Coins,
}

fn dice_roll(quant: u64, sz: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let mut ret = 0;

    for _ in 0..quant {
        ret += rng.gen::<u64>()%sz + 1;
    }

    ret
}

fn main() {
    let code = 'j';

    let treasure = match code {
        'j' => Coins {
            coin_type: CoinType::Copper,
            quantity: dice_roll(4, 6)
        },
        'k' => Coins {
            coin_type: CoinType::Silver,
            quantity: dice_roll(4, 4)
        },
        'l' => Coins {
            coin_type: CoinType::Electrum,
            quantity: dice_roll(3, 4)
        },
        'm' => Coins {
            coin_type: CoinType::Gold,
            quantity: dice_roll(1, 8)
        },
        'n' => Coins {
            coin_type: CoinType::Platinum,
            quantity: dice_roll(1, 4) + 1
        },
        _ => panic!("Unknown treasure code")
    };

    println!("TREASURE");
    println!("========");
    println!("{:?}", treasure);
}
