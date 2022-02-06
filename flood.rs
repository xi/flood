use std::io;
use std::io::Write;

const WIDTH: usize = 18;
const HEIGHT: usize = 18;

extern "C" {
    fn getrandom(buf: *mut u8, buflen: usize, flags: u32) -> isize;
}

fn entropy() -> u64 {
    let mut bytes = [0; 8];
    unsafe {
        getrandom(bytes.as_mut_ptr(), 8, 0x0001);
    }
    return u64::from_ne_bytes(bytes);
}

struct Rng {
    state: u64,
}

impl Rng {
    fn gen(&mut self) -> u64 {
        // https://github.com/smol-rs/fastrand/blob/master/src/lib.rs
        let s = self.state.wrapping_add(0xA0761D6478BD642F);
        self.state = s;
        let t = u128::from(s) * u128::from(s ^ 0xE7037ED1A0B428DB);
        return (t as u64) ^ (t >> 64) as u64
    }

    fn gen_range(&mut self, low: u8, high: u8) -> u8 {
        return low + (self.gen() % (high - low) as u64) as u8;
    }
}

fn input(prompt: &str) -> Result<String, io::Error> {
    let mut line = String::new();

    print!("{}", prompt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut line)?;

    return Ok(line.trim().to_string());
}

fn input_num(prompt: &str, low: u8, high: u8) -> u8 {
    loop {
        match input(prompt) {
            Ok(s) => match s.parse::<u8>() {
                Ok(a) if a >= low && a < high => {
                    return a;
                },
                _ => {},
            },
            _ => {},
        }
        println!("invalid input. try again");
    }
}

fn set_color(c: u8) {
    // use different bold values to increase contrast
    print!("\x1b[{};{}m", match c {
        3|4 => 0,
        _ => 1,
    }, 31 + c);
}

fn render(map: &[u8; WIDTH * HEIGHT]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let c = map[y * WIDTH + x];
            set_color(c);
            print!("██");
        }
        println!("\x1b[0m");
    }
}

fn render_status(step: u8) {
    print!("{}/{}", step, 32);
    // move to column
    print!("\x1b[{}G", WIDTH * 2 - 6 * 3 + 2);
    for i in 0..6 {
        set_color(i);
        print!("■\x1b[0m{} ", i + 1);

    }
    println!("\x1b[0m");

}

fn flood(map: &mut [u8; WIDTH * HEIGHT], new: u8) {
    let old = map[0];
    let mut queue = vec![0];
    while queue.len() > 0 {
        let i = queue.pop().unwrap();
        if map[i] == new {
            continue;
        }
        map[i] = new;
        if i % WIDTH != 0 && map[i - 1] == old {
            queue.push(i - 1);
        }
        if i % WIDTH != WIDTH - 1 && map[i + 1] == old {
            queue.push(i + 1);
        }
        if i / WIDTH != 0 && map[i - WIDTH] == old {
            queue.push(i - WIDTH);
        }
        if i / WIDTH != HEIGHT - 1 && map[i + WIDTH] == old {
            queue.push(i + WIDTH);
        }
    }
}

fn play(seed: u64) -> bool {
    let mut rng = Rng {state: seed};
    let mut map = [0; WIDTH * HEIGHT];

    for i in 0..WIDTH * HEIGHT {
        map[i] = rng.gen_range(0, 6);
    }

    for step in 0..32 {
        // clear screen and move cursor to top left
        print!("\x1b[2J\x1b[H");
        render_status(step);
        render(&map);
        println!("");
        if map.iter().all(|x| *x == map[0]) {
            return true;
        }
        let new = input_num("> ", 1, 7) - 1;
        flood(&mut map, new);
    }
    return false;
}

fn main() {
    let seed = entropy();
    loop {
        if play(seed) {
            println!("good job!");
            break;
        }
    }
}
