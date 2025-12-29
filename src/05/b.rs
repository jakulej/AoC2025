pub fn resolve(input: &String) -> String {


    let (fresh_ids, _product_ids) = input
        .split_once("\n\n")
        .unwrap();

    let fresh_ids: Vec<(usize,usize)> = fresh_ids
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (first, second) = line.split_once("-").unwrap();
            (first.parse::<usize>().unwrap()
             , second.parse::<usize>().unwrap())
        }).collect();

    count_fresh_id(fresh_ids).to_string()
}
fn count_fresh_id(mut fresh_ids: Vec<(usize,usize)>) -> usize {

    fresh_ids.sort_by_key(|range| range.0);

    let mut available_id_count = 0usize;
    let mut current_range = fresh_ids[0];

    for i in 0..fresh_ids.len() {

        if fresh_ids[i].1 > current_range.1 {
            current_range.1 = fresh_ids[i].1;        
        }
        if i+1 >= fresh_ids.len(){
            available_id_count += current_range.1 - current_range.0 + 1;
            continue;
        }

        if fresh_ids[i+1].0 > current_range.1 {
            available_id_count += current_range.1 - current_range.0 + 1;
            current_range.0 = fresh_ids[i+1].0;           
        }
    }
    available_id_count
}
