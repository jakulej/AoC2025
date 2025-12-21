use std::iter;

pub fn resolve(input: &[u8]) -> String {
    input
        .split(|b| b == &b',')
        .filter(|id_range| filter_correct_numbers(id_range))
        .map(|id_range| {
            let mut iterator = id_range.splitn(2, |b| b==&b'-');
            let range = (iterator.next().unwrap(),iterator.next().unwrap());
            let range = (split_half(range.0), split_half(range.1));          


        });

    String::new()
}



/*
 * Steps:
 * 1. Check if in range is odd numbers of digits
 * 2. Check how many wrong id have
 *
 *
 *
 */

fn sum_invalid_id(range: ((&[u8],&[u8]),(&[u8],&[u8]))) -> usize {
    
    let invalid_possibilites = (range.1.0) as usize - range.0.0 as usize;


    5
}


fn split_half(number: &[u8]) -> (usize,usize){
    let split_point = number.len()/2 + number.len()%2;

    let splitted = number.split_at(split_point);
    
        (std::str::from_utf8(splitted.0).unwrap().parse::<usize>().unwrap(),
        std::str::from_utf8(splitted.0).unwrap().parse::<usize>().unwrap())

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
