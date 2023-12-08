#![allow(dead_code)]
#[derive(Debug)]
enum Banknotes {
    Krasnoyarsk,
    SanktPetersburg,
    Moscow,
    Sevastopol,
    Archangelsk,
    Yaroslavl,
    Vladivostok,
    Khabarovsk,
    Olympic(Sport),
    
}

#[derive(Debug)]
enum Sport {
    Snowboard,
    Football,
}
fn value_in_rubles(city: Banknotes) -> u32 {
    match city {
        Banknotes::Krasnoyarsk => 10,
        Banknotes::SanktPetersburg => 50,
        Banknotes::Moscow => 100,
        Banknotes::Sevastopol => 200,
        Banknotes::Archangelsk=> 500,
        Banknotes::Yaroslavl => 1000,
        Banknotes::Vladivostok => 2000,
        Banknotes::Khabarovsk => 5000,
        Banknotes::Olympic(sport) => {
            println!("Lucky Olympic! {:?}", sport);
            100
        }
    }
}
fn main() {
    println!("{:?}", value_in_rubles(Banknotes::Olympic(Sport::Football)));
}
