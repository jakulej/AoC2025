pub fn resolve(input: &[u8]) -> String {
    let sollution: usize = input
            .split(|b| b == &b',')
            .filter(|id_range| filter_correct_numbers(id_range))
            .map(|id_range| {
                let mut iterator = id_range.splitn(2, |b| b==&b'-');
                let first =  parse_to_usize(iterator.next().unwrap());
                let second = parse_to_usize(iterator.next().unwrap());                
                let range = (first,second);

                sum_invalid_id(range)


            }).sum();

    sollution.to_string()
}

fn parse_to_usize(number: &[u8]) -> usize {
    std::str::from_utf8(number).unwrap().trim().parse::<usize>().unwrap()
}



/*
 * Steps:
 * 1. Check if in range is odd numbers of digits
 * 2. Check how many wrong id have
 *
 *
 *
 */

fn sum_invalid_id(range: (usize,usize)) -> usize {
    let range_splitted = split_half(range);
    let mut acc = 0usize;
    for i in range_splitted.0.0 .. ( range_splitted.1.0 + 1) {

        let invalid_id = generate_invalid_id(i);
        if (range.0..(range.1+1)).contains(&invalid_id){
            acc = acc + invalid_id
        }
        
    }
    acc
}

fn generate_invalid_id(half_digit: usize) -> usize {
    if half_digit != 0 {
        let digits = half_digit.checked_ilog10().unwrap_or(0) + 1;
        return half_digit * 10_usize.pow(digits as u32) + half_digit
    }
    else {
        return 0
    }
}


fn split_half(number: (usize, usize)) -> ((usize,usize),(usize,usize)){
    
    let number = (number.0.to_string(),number.1.to_string());
    let mut split_point = (number.0.len()/2, number.1.len()/2);
    if number.0.len() != number.1.len() {
        split_point.1 = split_point.1 + number.1.len() % 2;
    }

    let splitted = (number.0.split_at(split_point.0), number.1.split_at(split_point.1));
    let first = (splitted.0.0.parse::<usize>().unwrap_or(0usize), splitted.0.1.parse::<usize>().unwrap());
    let second = (splitted.1.0.parse::<usize>().unwrap(), splitted.1.1.parse::<usize>().unwrap());
    (first, second)   
}

fn is_in_range_odd_digit_numbers(range: (&[u8],&[u8])) -> bool {
    if range.0.len() % 2 == 0 {
        return true
    }
    else if range.1.len() - range.0.len() > 0 {
        return true
    }

    false
}

fn filter_correct_numbers(id_range: &[u8]) -> bool{
    let mut iterator = id_range.splitn(2, |b| b==&b'-');
        let first_number = iterator.next().unwrap();
        let second_numer = iterator.next().unwrap();

        is_in_range_odd_digit_numbers((first_number,second_numer))
}
