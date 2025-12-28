pub fn resolve(input: &[u8]) -> String {
    let map: Vec<&[u8]> = input
            .split(|b| b == &b'\n')
            .filter(|line| !line.is_empty())
            .collect();
    count_avaiable_papers(map).to_string()
}
