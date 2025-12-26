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
    let mut batteries_index: Vec<usize> = vec![0;12];
    batteries_index[0] = find_first_largest_index(bank.clone());

    for i in 1..12 {
        let mut sliced_bank= bank.clone();
        let iter_to = bank.len() - (12-i-1);
        sliced_bank.drain(iter_to..);
        batteries_index[i] = find_largest_index_after(sliced_bank, batteries_index[i-1]);
    }

    let score= batteries_index
        .iter()
        .map(|i| bank[*i])
        .fold(0, |acc, digit| acc*10 + digit);
    score
}

fn find_first_largest_index(bank: Vec<usize>) -> usize {
    let mut bank_without_last = bank.clone();
    bank_without_last.drain((bank.len()-11)..);
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
