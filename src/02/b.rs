pub fn resolve(input: &[u8]) -> String {
    let sollution: usize = input
            .split(|b| b == &b',')
            .map(|id_range| {
                let mut iterator = id_range.splitn(2, |b| b==&b'-');
                let first =  parse_to_usize(iterator.next().unwrap());
                let second = parse_to_usize(iterator.next().unwrap());                

                (first..(second + 1))
                    .filter(|id| is_valid_id(*id))
                    .sum::<usize>()

            }).sum();

    sollution.to_string()
}

fn is_valid_id(id: usize) -> bool{
    match 1 + id.ilog10() {
        1 => false,
        2 => id.is_multiple_of(11),
        3 => id.is_multiple_of(111),
        4 => id.is_multiple_of(101),
        5 => id.is_multiple_of(11111),
        6 => id.is_multiple_of(1001) || id.is_multiple_of(10101),
        7 => id.is_multiple_of(1111111),
        8 => id.is_multiple_of(1010101) || id.is_multiple_of(10001),
        9 => id.is_multiple_of(1001001),
        10 => id.is_multiple_of(101010101) || id.is_multiple_of(100001),
        _ => panic!(),
    }
}

fn parse_to_usize(number: &[u8]) -> usize {
    std::str::from_utf8(number).unwrap().trim().parse::<usize>().unwrap()
}

