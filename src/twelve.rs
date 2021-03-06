extern crate regex;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;

fn key_to_string(bs: &[bool; 5]) -> String {
    let mut res = String::new();
    for i in 0..5 {
        if bs[i] {
            res += "#";
        } else {
            res += ".";
        }
    }
    res
}

fn to_bit_array(key: &str) -> [bool; 5] {
    let x = key.as_bytes();
    
    [x[0] == '#' as u8, x[1] == '#' as u8, x[2] == '#' as u8, x[3] == '#' as u8, x[4] == '#' as u8]
}

fn to_bit_state(state: String) -> VecDeque<bool> {
    state.into_bytes()
        .into_iter()
        .map(|b| { b == '#' as u8})
        .collect()
}

impl Debug for Window
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut bs = [ false, false, false, false, false ];
        self.copy_to(&mut bs);

        let res = key_to_string(&mut bs);
        f.write_str(&res)
    }
}


pub struct Window
{
    pieces: [bool; 5],
    write_index: usize,
    read_index: usize
}

pub struct GrowOp
{
    body: VecDeque<bool>,
    prefix: VecDeque<bool>,
    suffix: VecDeque<bool>,
    new_front_pot: bool,
    new_back_pots: isize
}

impl Window {
    /*

    00000

    shove 1.

    10000 . want to read 00001, so rI = 1, wI = 1

    shove 0

    10000 . want to read 00010, so rI = 2, wI = 2

    shove 1

    10100 . want to read 00101, so rI =

    */

    pub fn reset(&mut self) {
        self.pieces[0] = false;
        self.pieces[1] = false;
        self.pieces[2] = false;
        self.pieces[3] = false;
        self.pieces[4] = false;
        self.write_index = 0;
        self.read_index = 0;
    }

    pub fn shove(&mut self, b: bool) {
        self.pieces[self.write_index] = b;
        if self.write_index == 4 {
            self.write_index = 0;
        } else {
            self.write_index += 1;
        }
        if self.read_index == 4 {
            self.read_index = 0;
        } else {
            self.read_index += 1;
        }
    }

    pub fn copy_to(&self, k: &mut [bool; 5]) {
        let mut reading_index = self.read_index;
        for i in 0..5 {
            k[i] = self.pieces[reading_index];
            if reading_index == 4 {
                reading_index = 0;
            } else {
                reading_index += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use Window;

    #[test]
    fn shoving() {
        let mut window = Window {
            pieces: [false, false, false, false, false],
            write_index: 0,
            read_index: 0
        };

        window.shove(true);
        window.shove(false);
        window.shove(true);

        assert_eq!([true, false, true, false, false], window.pieces);
    }

    #[test]
    fn copying() {
        let mut window = Window {
            pieces: [false, true, false, true, false],
            write_index: 0,
            read_index: 0
        };

        window.shove(true);
        window.shove(false);
        window.shove(true);

        let mut output = [false, false, false, false, false];

        window.copy_to(&mut output);

        assert_eq!([true, false, true, false, true], output);
    }
}


fn main() {
    let initial_state =
        //String::from(/* ..... */"#..#.#..##......###...###" /* ..... */);
        String::from("######....##.###.#..#####...#.#.....#..#.#.##......###.#..##..#..##..#.##..#####.#.......#.....##..");
    let mut evolute = HashMap::new();


//    evolute.insert(to_bit_array("...##"), true);
//    evolute.insert(to_bit_array("..#.."), true);
//    evolute.insert(to_bit_array(".#..."), true);
//    evolute.insert(to_bit_array(".#.#."), true);
//    evolute.insert(to_bit_array(".#.##"), true);
//    evolute.insert(to_bit_array(".##.."), true);
//    evolute.insert(to_bit_array(".####"), true);
//    evolute.insert(to_bit_array("#.#.#"), true);
//    evolute.insert(to_bit_array("#.###"), true);
//    evolute.insert(to_bit_array("##.#."), true);
//    evolute.insert(to_bit_array("##.##"), true);
//    evolute.insert(to_bit_array("###.."), true);
//    evolute.insert(to_bit_array("###.#"), true);
//    evolute.insert(to_bit_array("####."), true);

    evolute.insert(to_bit_array("...##"), true);
    evolute.insert(to_bit_array("###.."), false);
    evolute.insert(to_bit_array("#.#.#"), false);
    evolute.insert(to_bit_array("#####"), false);
    evolute.insert(to_bit_array("....#"), false);
    evolute.insert(to_bit_array("##.##"), false);
    evolute.insert(to_bit_array("##.#."), true);
    evolute.insert(to_bit_array("##..."), true);
    evolute.insert(to_bit_array("#..#."), true);
    evolute.insert(to_bit_array("#.#.."), false);
    evolute.insert(to_bit_array("#.##."), false);
    evolute.insert(to_bit_array("....."), false);
    evolute.insert(to_bit_array("##..#"), false);
    evolute.insert(to_bit_array("#..##"), false);
    evolute.insert(to_bit_array(".##.#"), true);
    evolute.insert(to_bit_array("..###"), true);
    evolute.insert(to_bit_array("..#.#"), true);
    evolute.insert(to_bit_array(".####"), true);
    evolute.insert(to_bit_array(".##.."), false);
    evolute.insert(to_bit_array(".#..#"), true);
    evolute.insert(to_bit_array("..##."), false);
    evolute.insert(to_bit_array("#...."), false);
    evolute.insert(to_bit_array("#...#"), false);
    evolute.insert(to_bit_array(".###."), false);
    evolute.insert(to_bit_array("..#.."), false);
    evolute.insert(to_bit_array("####."), true);
    evolute.insert(to_bit_array(".#.##"), false);
    evolute.insert(to_bit_array("###.#"), false);
    evolute.insert(to_bit_array("#.###"), true);
    evolute.insert(to_bit_array(".#..."), true);
    evolute.insert(to_bit_array(".#.#."), false);
    evolute.insert(to_bit_array("...#."), false);

    let mut state = to_bit_state(initial_state);
    let mut new_front = VecDeque::new();
    let mut new_back = VecDeque::new();
    let mut window = Window {
        pieces: [false, false, false, false, false],
        write_index: 0,
        read_index: 0
    };
    let mut key = [false, false, false, false, false];

    /*
    0
....#..#.#..##......###...###
..#...#....#.....#..#..#..#..

    */
    print(&state);
    let mut zeroth_pot_index : i32 = 0;

    let mut g = GrowOp{
        body: state,
        prefix: new_front,
        suffix: new_back,
        new_front_pot: false,
        new_back_pots: 0
    };

    for h in 0..250 {
        window.reset();
        g.new_front_pot = false;
        g.new_back_pots = 0;

        for i in 0..g.body.len() + 4 {
            let pot_index = (i as i32) - 2;
            if i < g.body.len() {
                window.shove(g.body[i]);
            } else {
                window.shove(false);
            }

            window.copy_to(&mut key);
            let new_val = *evolute.get(&key).unwrap_or(&false);
            //println!("{} {} {} {}", key_to_string(&key), new_val, i, pot_index);

            put_pot(&mut g, pot_index, new_val)
        }

        zeroth_pot_index -= g.prefix.len() as i32;
        while !g.prefix.is_empty() {
            g.body.push_front(g.prefix.pop_back().unwrap());
        }
        //print(&new_back);

        while g.new_back_pots != 0 {
            let plant = g.suffix.pop_front().unwrap();
            g.body.push_back(plant);
            if plant {
                g.new_back_pots -= 1;
            }
        }
        g.suffix.clear();

        while !g.body[0] && zeroth_pot_index != 0 {
            g.body.pop_front();
            zeroth_pot_index += 1;
        }

        print(&g.body);
        print_total(&g.body, zeroth_pot_index, h);
    }
}

fn put_pot(state: &mut GrowOp, pot_index: i32, new_val: bool) -> () {
    if pot_index < 0 {
        if state.new_front_pot || new_val {
            state.prefix.push_back(new_val);
            state.new_front_pot = true;
        }
    } else if pot_index >= state.body.len() as i32
        {
            state.suffix.push_back(new_val);
            if new_val {
                state.new_back_pots += 1;
            }
        } else {
        state.body[pot_index as usize] = new_val;
    }
}

fn print_total(state: &VecDeque<bool>, zeroth_pot_index: i32, gen: usize) {
    let mut counting_index = zeroth_pot_index;
    let mut total = 0;
    for i in 0..state.len() {
        if state[i] {
            total += counting_index;
        }
        counting_index += 1
    }
    println!("At generation {}, total was {}", gen + 1, total);
}

fn print(state: &VecDeque<bool>) {
    let res = state_to_string(state);
    println!("{}", res);
}

fn state_to_string(state: &VecDeque<bool>) -> String {
    let mut res = String::new();
    state.iter().for_each(|b| {
        if *b {
            res += "#";
        } else {
            res += ".";
        }
    });
    res
}