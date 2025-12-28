const PAPER:u8 = b'@';
const EMPTY:u8 = b'.';

pub fn resolve(input: &[u8]) -> String {
    let mut map: Vec<Vec<u8>> = input
            .split(|b| b == &b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.to_vec())
            .collect();
    let mut removed_paper_count = 0usize;

    loop {
        let paper_to_remove = get_avaiable_papers(&map);
        removed_paper_count += paper_to_remove.len();

        remove_paper(&mut map, &paper_to_remove);
        
        if paper_to_remove.len() == 0 {
            break;
        }
    }
    removed_paper_count.to_string()
}
fn remove_paper(map: &mut Vec<Vec<u8>>, paper_to_remove: &Vec<(usize,usize)>){
    paper_to_remove
        .iter()
        .for_each(|(x,y)| {
            map[*y][*x] = EMPTY;
        });

}
fn get_avaiable_papers(map: &Vec<Vec<u8>>) -> Vec<(usize,usize)> {
    let mut available_papers: Vec<(usize,usize)> = Vec::new(); 
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != PAPER{
                continue;
            }
            if is_paper_avaiable(map.clone(), (j,i)){
                available_papers.push((j,i));
            }
        }
    }
    available_papers
}
fn is_paper_avaiable(map: Vec<Vec<u8>>, (x, y): (usize,usize)) -> bool {
    let mut adjecent_papers = 0u8;
    for dy in [-1,0,1] {
        for dx in [-1,0,1] {
            if dx == 0 && dy ==0 {
                continue;
            }

            let ny = match y.checked_add_signed(dy) {
                Some(v) => v,
                None => continue,
            };

            let nx = match x.checked_add_signed(dx) {
                Some(v) => v,
                None => continue,
            };
            if map.get(ny).and_then(|r| r.get(nx)) == Some(&PAPER) {
                adjecent_papers += 1;
            } 

        }
    }

    adjecent_papers < 4
}
