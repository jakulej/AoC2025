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

fn count_fresh_id(fresh_ids: Vec<(usize,usize)>) -> usize {

    (0..(get_highest_id(&fresh_ids)+1))
        .filter(|product| is_fresh(*product, &fresh_ids))
        .count()
}

fn is_fresh(product_id: usize, fresh_ids: &Vec<(usize,usize)>) -> bool {
    fresh_ids.iter().any(|range| (range.0..(range.1+1)).contains(&product_id))
}
fn get_highest_id(fresh_ids: &Vec<(usize,usize)>) -> usize {
    fresh_ids
        .iter()
        .map(|range| range.1)
        .max()
        .unwrap()
}
