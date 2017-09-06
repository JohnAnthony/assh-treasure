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
enum PotionType {
    Longevity,
    Love,
    Poison,
    Climbing,
    Delusion,
    Diminutiveness,
    ExtraGrowth,
    ExtraHealing,
    FireResist,
    Flying,
    GaseousForm,
    GiantPower,
    Healing,
    HeroicAction,
    Invincibility,
    Invisibility,
    Levitation,
    SuperHeroicAction,
    SuperSpeed,
    WaterBreathing,
}

impl PotionType {
    fn random() -> PotionType {
        let mut rng = rand::thread_rng();
        match rng.gen::<u64>()%20 {
            0 => PotionType::Longevity,
            1 => PotionType::Love,
            2 => PotionType::Poison,
            3 => PotionType::Climbing,
            4 => PotionType::Delusion,
            5 => PotionType::Diminutiveness,
            6 => PotionType::ExtraGrowth,
            7 => PotionType::ExtraHealing,
            8 => PotionType::FireResist,
            9 => PotionType::Flying,
            10 => PotionType::GaseousForm,
            11 => PotionType::GiantPower,
            12 => PotionType::Healing,
            13 => PotionType::HeroicAction,
            14 => PotionType::Invincibility,
            15 => PotionType::Invisibility,
            16 => PotionType::Levitation,
            17 => PotionType::SuperHeroicAction,
            18 => PotionType::SuperSpeed,
            19 => PotionType::WaterBreathing,
            _ => panic!()
        }
    }
}

#[derive(Debug)]
enum Treasure {
    Coins(CoinType),
    Potion(PotionType),
}

#[derive(Debug)]
struct TreasureEntry {
    quantity: u64,
    treasure: Treasure,
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
    let code = 's';
    let mut treasures = Vec::new();

    match code {
        'j' => treasures.push(TreasureEntry {
            treasure: Treasure::Coins(CoinType::Copper),
            quantity: dice_roll(4, 6)
        }),
        'k' => treasures.push(TreasureEntry {
            treasure: Treasure::Coins(CoinType::Silver),
            quantity: dice_roll(4, 4)
        }),
        'l' => treasures.push(TreasureEntry {
            treasure: Treasure::Coins(CoinType::Electrum),
            quantity: dice_roll(3, 4)
        }),
        'm' => treasures.push(TreasureEntry {
            treasure: Treasure::Coins(CoinType::Gold),
            quantity: dice_roll(1, 8)
        }),
        'n' => treasures.push(TreasureEntry {
            treasure: Treasure::Coins(CoinType::Platinum),
            quantity: dice_roll(1, 4) + 1
        }),
        'o' => {
            if percent_chance(25) {
                treasures.push(TreasureEntry {
                    treasure: Treasure::Coins(CoinType::Copper),
                    quantity: dice_roll(2, 4)
                })
            }
            if percent_chance(20) {
                treasures.push(TreasureEntry {
                    treasure: Treasure::Coins(CoinType::Silver),
                    quantity: dice_roll(1, 6)
                })
            }
        },
        'p' => {
            if percent_chance(30) {
                treasures.push(TreasureEntry {
                    treasure: Treasure::Coins(CoinType::Silver),
                    quantity: dice_roll(2, 6)
                })
            }
            if percent_chance(20) {
                treasures.push(TreasureEntry {
                    treasure: Treasure::Coins(CoinType::Electrum),
                    quantity: dice_roll(1, 4)
                })
            }
        },
        's' => {
            if percent_chance(40) {
                for _ in 0..dice_roll(2, 4) {
                    treasures.push(TreasureEntry {
                        treasure: Treasure::Potion(PotionType::random()),
                        quantity: 1
                    })
                }
            }
        },
        'y' => {
            if percent_chance(70) {
                treasures.push(TreasureEntry {
                    treasure: Treasure::Coins(CoinType::Gold),
                    quantity: dice_roll(4, 12)
                })
            }
        }
        _ => panic!("Unknown treasure code")
    };

    println!("TREASURE");
    println!("========");
    println!("{:?}", treasures);
}
