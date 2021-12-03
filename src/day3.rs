pub fn day3_task1(mut input: String) {
    input.retain(|c| c != '\n');

    const PACKET_LEN:usize = 12;
    let packets = input.len() / PACKET_LEN;
    let mut gamma = String::new();

    for i in 0..PACKET_LEN {
        let mut zero_count = 0;
        for j in 0..packets {
            match input.as_str().bytes().nth(i + j*PACKET_LEN).unwrap() {
                48 => zero_count = zero_count + 1,  //bytes ~100x faster than chars()
                49 => zero_count = zero_count - 1,
                _ => panic!("invalid input")
            }
        }
        gamma.push(if zero_count > 0 {'0'} else {'1'});
    }

    let gamma_i = u32::from_str_radix(gamma.as_str(), 2).unwrap();
    println!("day 3 task 1: {}", gamma_i * (((1<<PACKET_LEN) - 1) - gamma_i));
}

pub fn day3_task2(input: String) {
}
