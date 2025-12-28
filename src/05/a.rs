pub fn resolve(input: &String) -> String {


    let (fresh_ids, product_ids) = input
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

    let product_ids: Vec<usize> = product_ids
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    count_fresh_products(product_ids, fresh_ids).to_string()
}

fn count_fresh_products(product_ids: Vec<usize>, fresh_ids: Vec<(usize,usize)>) -> usize {
    product_ids
        .iter()
        .filter(|product| is_fresh(**product, &fresh_ids))
        .count()
}

fn is_fresh(product_id: usize, fresh_ids: &Vec<(usize,usize)>) -> bool {
    fresh_ids.iter().any(|range| (range.0..(range.1+1)).contains(&product_id))
}
