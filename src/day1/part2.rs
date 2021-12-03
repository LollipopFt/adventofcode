pub fn finale(contents: Vec<u16>) {
    let mut vector: Vec<u32> = Vec::new();
    for i in 0..(contents.len()-2) {
        vector.push(contents[i] as u32
                  + contents[i+1] as u32
                  + contents[i+2] as u32
        );
    }
    let mut count: u16 = 0;
    for i in 1..vector.len() {
        if vector[i] > vector[i-1] {
            count += 1;
        } else {}
    }
    print!("{}", count);
}