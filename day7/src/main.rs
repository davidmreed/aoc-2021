fn fuel_cost(crabs: &Vec<usize>, position: usize) -> usize {
    crabs
        .iter()
        .map(|c| (*c as i64 - position as i64).abs() as usize)
        .sum()
}

fn fuel_cost_for_move(crab: usize, position: usize) -> usize {
    let diff = (crab as i64 - position as i64).abs() as usize;
    (1..=diff).sum()
}

fn fuel_cost_part_2(crabs: &Vec<usize>, position: usize) -> usize {
    crabs.iter().map(|c| fuel_cost_for_move(*c, position)).sum()
}

fn main() {
    let crabs: Vec<usize> = include_str!("input.txt")
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    println!(
        "Cheapest position: {}",
        (0..1000).map(|p| fuel_cost(&crabs, p)).min().unwrap()
    );
    println!(
        "Cheapest position, part 2: {}",
        (0..1000)
            .map(|p| fuel_cost_part_2(&crabs, p))
            .min()
            .unwrap()
    );
}
