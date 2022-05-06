enum CitySize {
    Town,       // approximate residents: 1_000
    City,       // approximate residents: 10_000
    Metropolis, // approximate residents: 1_000_000
    Area { residents: u64 },
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
            CitySize::Area { residents } => (
                format!("an *area* of exactly {} residents", residents),
                residents,
            ),
        };

        City {
            description,
            residents,
        }
    }
}

fn main() {
    let mut rustville = City::new(CitySize::City);
    println!("This city is {}", rustville.description);

    rustville = City::new(CitySize::Area {
        residents: 12_232_023,
    });
    println!("This city is {}", rustville.description);

    rustville = City::new(CitySize::Town);
    println!("This city is {}", rustville.description);

    rustville = City::new(CitySize::Metropolis);
    println!("This city is {}", rustville.description);

    if rustville.residents > 100_000 {
        println!("Wow!");
    }
}
