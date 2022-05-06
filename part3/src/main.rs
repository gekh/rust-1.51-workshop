enum CitySize {
    Town,       // approximate residents: 1_000
    City,       // approximate residents: 10_000
    Metropolis, // approximate residents: 1_000_000
}

struct City {
    description: String,
    residents: u64,
}

impl City {
    fn new(city_size: CitySize) -> City {
        let (description, residents) = match city_size {
            CitySize::Town => {
                let residents = 1_000;

                (
                    format!("a *town* of approximately {} residents", residents),
                    residents,
                )
            }
            CitySize::City => {
                let residents = 10_000;

                (
                    format!("a *city* of approximately {} residents", residents),
                    residents,
                )
            }
            CitySize::Metropolis => {
                let residents = 1_000_000;

                (
                    format!("a *metropolis* of approximately {} residents", residents),
                    residents,
                )
            }
        };

        City {
            description,
            residents,
        }
    }
}

fn main() {
    // 👉 TODO Use City::new() to create a Metropolis-sized city here
    let mut rustville = City::new(CitySize::City);
    println!("This city is {}", rustville.description);

    rustville = City::new(CitySize::Town);
    println!("This city is {}", rustville.description);

    rustville = City::new(CitySize::Metropolis);
    println!("This city is {}", rustville.description);

    if rustville.residents > 100_000 {
        println!("Wow!");
    }
}
