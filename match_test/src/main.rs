#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nikel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin{
        Coin::Penny => {
            println!("Luck penny!");
            1
        },
        Coin::Nikel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let coin = Coin::Penny;
    println!("this value {}", value_in_cents(coin));

    let coin = Coin::Quarter(UsState::Alabama);
    println!("this value {}", value_in_cents(coin));

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
        // _ => (),  无事发生
        //other =>  move_player(other),
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maxium is configured to be {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maxium is configured to be {}", max);
    }

    
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll(){}