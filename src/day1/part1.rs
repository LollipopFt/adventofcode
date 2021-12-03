pub fn finale(contents: Vec<u16>) {
    let mut count: u16 = 0;
    for i in 1..contents.len() {
        if contents[i] > contents[i-1] {
            count += 1;
        } else {}
    }
    print!("{}", count);
}