const MAX_DIAL: usize = 99;

pub fn resolve(input: &[u8]) -> String {
    let mut dial = 50usize;
    let mut counter = 0usize;

    input
        .split(|b| b == &b'\n') //Podzielenie pliku na linijki
        .for_each(|line| {
            if let Some((direction, distance)) = line.split_first() {
                let distance: usize = str::from_utf8(distance).unwrap().parse().unwrap();
                let (pointing_0_times, dial_result) = match direction {
                    b'L' => subtract(dial, distance),
                    b'R' => add(dial, distance),
                    _ => (0usize, 0usize),
                };
                dial = dial_result;
                print!("Rotation {:?}{}, pointed {} times",*direction as char,distance,pointing_0_times);
                print!(" - Ended on {}",dial);
                counter += pointing_0_times;
                if dial == 0 {
                    counter += 1;
                }
                print!(" - Counter: {}\n",counter);
            }
        });
    counter.to_string()
}
fn subtract(dial: usize, distance: usize) -> (usize, usize) {

    let (mut pointing_0_times, distance) = correct_ditance(distance);
    let new_dial= dial.checked_sub(distance)
        .unwrap_or_else(|| {
            pointing_0_times += 1;
            MAX_DIAL - (distance - dial) + 1
        });
    (pointing_0_times, new_dial)
}
fn add(dial: usize, distance: usize) -> (usize, usize) {
    let (mut pointing_0_times, distance) = correct_ditance(distance);

    let result = dial + distance;
    if result > MAX_DIAL {
        pointing_0_times += 1;
        (pointing_0_times,result - MAX_DIAL - 1)
    } else {
        (0,result)
    }
}

fn correct_ditance(distance: usize) -> (usize,usize) {
    (   distance.div_ceil(MAX_DIAL) - 1, //Times pointer pointed at 0
        distance % (MAX_DIAL + 1)) // Distance to move pointer
}
