const PAPER:u8 = b'@';

pub fn resolve(input: &[u8]) -> String {
    let map: Vec<&[u8]> = input
            .split(|b| b == &b'\n')
            .filter(|line| !line.is_empty())
            .collect();
    count_avaiable_papers(map).to_string()
}

fn count_avaiable_papers(map: Vec<&[u8]>) -> usize {
    let mut paper_count = 0usize;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != PAPER{
                continue;
            }
            if is_paper_avaiable(map.clone(), (j,i)){
                paper_count += 1;
                println!("Paper on: {},{}",j,i);
            }
        }
    }
    paper_count
}
fn is_paper_avaiable(map: Vec<&[u8]>, (x, y): (usize,usize)) -> bool {
    println!("Start x: {}, y: {}",x,y);
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
            println!("  Checking  x: {}, y: {}",nx,ny);
            if map.get(ny).and_then(|r| r.get(nx)) == Some(&PAPER) {
                adjecent_papers += 1;
            } 

        }
    }

    adjecent_papers < 4
}
