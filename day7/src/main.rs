fn fuel_cost(crabs: &Vec<i64>, position: i64) -> i64 {
    crabs.iter().map(|c| (c - position).abs()).sum()
}

fn fuel_cost_for_move(crab: i64, position: i64) -> i64 {
    let diff = (crab - position).abs();
    diff * (diff + 1) / 2
}

fn fuel_cost_part_2(crabs: &Vec<i64>, position: i64) -> i64 {
    crabs.iter().map(|c| fuel_cost_for_move(*c, position)).sum()
}

fn main() {
    let crabs: Vec<i64> = include_str!("input.txt")
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let furthest_crab = *crabs.iter().max().unwrap();

    println!(
        "Cheapest position: {}",
        (0..furthest_crab)
            .map(|p| fuel_cost(&crabs, p))
            .min()
            .unwrap()
    );
    println!(
        "Cheapest position, part 2: {}",
        (0..furthest_crab)
            .map(|p| fuel_cost_part_2(&crabs, p))
            .min()
            .unwrap()
    );
}
