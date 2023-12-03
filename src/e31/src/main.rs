fn main() {
    let mut ways = [0_usize; 201];
    ways[0] = 1;

    let coins: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
    for coin in coins {
        for idx in coin..201 {
            ways[idx] += ways[idx - coin]
        }
    }

    println!("{}", ways[200]);
}
