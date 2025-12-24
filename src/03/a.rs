pub fn resolve(input: &[u8]) -> String {
    let sollution: usize = input
            .split(|b| b == &b'\n')
            .map(|bank| bank //Cast to Vec<usize>
                .iter()
                .map(|b| (b - b'0') as usize).collect::<Vec<usize>>())
            .map(|bank| find_largest_joltage(bank))
            .sum::<usize>();


    sollution.to_string()
}

fn find_largest_joltage(bank: Vec<usize>) -> usize {
    if bank.len() == 0 {
        return 0;
    }
    let first_index = find_largest_not_last_index(bank.clone());
    let second_index = find_largest_index_after(bank.clone(), first_index);
    let max_joltage = bank[first_index] * 10 + bank[second_index];
    max_joltage
}

fn find_largest_not_last_index(bank: Vec<usize>) -> usize {
    let mut bank_without_last = bank.clone();
    bank_without_last.pop();

    let mut max_index = 0usize;
    
    for (index, battery) in bank_without_last.iter().enumerate() {
            if bank_without_last[max_index] == 9 {
                break;
            }

            if *battery > bank_without_last[max_index] {
                max_index = index;
            }
    }

    max_index
}
fn find_largest_index_after(bank: Vec<usize>, start_index: usize) -> usize {
    let mut max_index = start_index + 1;
    let iter = bank.iter().enumerate().skip(start_index + 1);
        for (index, battery) in iter {
             if bank[max_index] == 9 {
                break;
            }

            if *battery > bank[max_index] {
                max_index = index;
            }           
        }
    max_index
}
