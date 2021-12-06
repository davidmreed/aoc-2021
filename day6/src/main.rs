fn step(fish: &mut [u64; 9]) {
    *fish = [
        fish[1],           // 0 days
        fish[2],           // 1 day
        fish[3],           // 2 days
        fish[4],           // 3 days
        fish[5],           // 4 days
        fish[6],           // 5 days
        fish[7] + fish[0], // 6 days
        fish[8],           // 7 days
        fish[0],           // new fish: 8 days
    ]
}

fn main() {
    let mut fish: [u64; 9] = [0; 9];

    for starting_fish in include_str!("input.txt")
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
    {
        fish[starting_fish] += 1;
    }

    for _ in 0..80 {
        step(&mut fish);
    }
    println!("I have {} fish", fish.iter().sum::<u64>());

    for _ in 80..256 {
        step(&mut fish);
    }
    println!("I have {} fish", fish.iter().sum::<u64>());
}
