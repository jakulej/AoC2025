const MAX_DIAL: usize = 99;

pub fn resolve(input: &[u8]) -> String {
    let mut dial = 50usize;
    let mut counter = 0usize;

    input
        .split(|b| b == &b'\n') //Podzielenie pliku na linijki
        .for_each(|line| {
            if let Some((direction, distance)) = line.split_first() {
                let distance: usize = str::from_utf8(distance).unwrap().parse().unwrap();
                dial = match direction {
                    b'L' => subtract(dial, distance),
                    b'R' => add(dial, distance),
                    _ => 0usize,
                };
                if dial == 0 {
                    counter += 1;
                }
            }
        });
    counter.to_string()
}
fn subtract(dial: usize, distance: usize) -> usize {

    print!("{}\nL{} =",dial, distance);
    let distance = correct_ditance(distance);

    dial.checked_sub(distance)
        .unwrap_or_else(|| MAX_DIAL - (distance - dial) + 1)
}
fn add(dial: usize, distance: usize) -> usize {
    print!("{}\nR{} =",dial, distance);
    let distance = correct_ditance(distance);

    let result = dial + distance;
    if result > MAX_DIAL {
        result - MAX_DIAL - 1
    } else {
        result
    }
}

fn correct_ditance(distance: usize) -> usize {
    if distance > MAX_DIAL {
        distance % MAX_DIAL
    }
    else {
        distance
    }
}
