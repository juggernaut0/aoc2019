pub fn run1(inputs: Vec<u32>) -> u32 {
    inputs.into_iter()
        .map(to_fuel)
        .sum()
}

pub fn run2(inputs: Vec<u32>) -> u32 {
    inputs.into_iter()
        .map(to_more_fuel)
        .sum()
}

fn to_fuel(mass: u32) -> u32 {
    let fuel = ((((mass as f64) / 3.0).floor() as i32) - 2).max(0) as u32;
    log::debug!("to_fuel: {} -> {}", mass, fuel);
    fuel
}

fn to_more_fuel(mass: u32) -> u32 {
    let fuel = itertools::iterate(mass, |&m| to_fuel(m))
        .skip(1) // first element is the mass of the module itself
        .take_while(|&it| it > 0)
        .sum();
    log::debug!("to_more_fuel: {} -> {}", mass, fuel);
    fuel
}
