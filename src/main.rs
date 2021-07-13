fn multi_table(n: u64) -> String {
    let mut result: String = String::new();

    for i in 1..11 {
        result.insert_str(result.len(), &format!("{} * {} = {}\n", i, n, i * n));
    }
    result
}

//        assert_eq!(multi_table(5), "1 * 5 = 5\n2 * 5 = 10\n3 * 5 = 15\n4 * 5 = 20\n5 * 5 = 25\n6 * 5 = 30\n7 * 5 = 35\n8 * 5 = 40\n9 * 5 = 45\n10 * 5 = 50");

fn main() {
    let foo = multi_table(7);
    println!("{}", foo);
}
