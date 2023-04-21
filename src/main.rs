use ncurses::*;
use rand::Rng;
use std::{char, thread, time};
use std::sync::mpsc;

fn main() {
    /* Setup ncurses. */
    initscr();
    raw();

    keypad(stdscr(), true);
    noecho();
    
    let time_per_tile = time::Duration::from_millis(200); 
    let game_length = 20;
    let game_height = 10;
    let mut snake_vec: Vec<SnakeNode> = Vec::new();
    snake_vec.push(SnakeNode { x: 5, y: 5 });
    snake_vec.push(SnakeNode { x: 4, y: 5 });
    snake_vec.push(SnakeNode { x: 3, y: 5 });
    snake_vec.push(SnakeNode { x: 2, y: 5 });
    snake_vec.push(SnakeNode { x: 1, y: 5 });
    let mut display_vec: Vec<char> = Vec::new();
    for _ in 0..game_height {
        for _ in 0..game_length {
            display_vec.push('·');
        }
    }
    let mut money = Money { x: game_length - 1, y: game_height / 2 };
    let mut direction = Direction::Right;
    let mut eating = false;
    let mut rng = rand::thread_rng();
    let mut death = false;

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        let mut val = Direction::None;
        while val == Direction::None {
            val = match char::from_u32(getch() as u32).expect("Invalid char") {
                'w' => Direction::Up,
                's' => Direction::Down,
                'a' => Direction::Left,
                'd' => Direction::Right,
                _ => Direction::None,
            };
        }

        tx.send(val).expect("could not send value");
    });

    loop {
        if let Ok(val) = rx.try_recv() {
            direction = val;
        }

        clear();

        let new_snake_node = match direction {
            Direction::Up => SnakeNode {
                x: snake_vec[0].x,
                y: snake_vec[0].y - 1,
            },
            Direction::Down => SnakeNode {
                x: snake_vec[0].x,
                y: snake_vec[0].y + 1,
            },
            Direction::Left => SnakeNode {
                x: snake_vec[0].x - 1,
                y: snake_vec[0].y,
            },
            Direction::Right => SnakeNode {
                x: snake_vec[0].x + 1,
                y: snake_vec[0].y,
            },
            _ => {panic!("unexpected direction")}
        };
        if new_snake_node.x > game_length
            || new_snake_node.x <= 0
            || new_snake_node.y > game_height
            || new_snake_node.y <= 0
            || does_snake_die(&snake_vec)
        {
            death = true;
        
        } else {
            if !eating {
                snake_vec.pop();
            } else {
                eating = false;
            }
            snake_vec.insert(0, new_snake_node);
            if yum_yum(&snake_vec, &money) {
                eating = true;
                let mut values: Vec<(i32, i32)> = Vec::new();
                for node in &snake_vec {
                    values.push((node.x, node.y));
                }
                let mut new_x = rng.gen_range(1..=game_length);
                let mut new_y = rng.gen_range(1..=game_height);
                while is_in_vec(&(new_x, new_y), &values) {
                    new_x = rng.gen_range(1..=game_length);
                    new_y = rng.gen_range(1..=game_height);
                }
                money = Money { x: new_x, y: new_y };
            }
        }
        addstr(format!("{:?}\nScore: {}\n", direction, snake_vec.len()).as_ref());
        let print_out = &mut snake_to_display(&display_vec, &snake_vec);
        print_out[(((money.y - 1) * game_length + money.x) - 1) as usize] = '$';

        addstr(format!("{}\n", "-".repeat((game_length + 2) as usize)).as_ref());
        for number in 1..=game_height {
            let num_1 = ((number - 1) * game_length) as usize;
            let num_2 = (number * game_length) as usize;
            addstr(format!("|{}|\n", print_out[num_1..num_2].iter().collect::<String>()).as_ref());
        }
        addstr(format!("{}\n", "-".repeat((game_length + 2) as usize)).as_ref());
        /* Refresh, showing the previous message. */
        refresh();
        thread::sleep(time_per_tile);
        if death {
            addstr("You Died");
            refresh();
            getch();
            break;
        }
    }
    endwin();
}

#[derive(Debug)]
struct SnakeNode {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Money {
    x: i32,
    y: i32,
}

fn is_in_vec(values: &(i32, i32), vec: &Vec<(i32, i32)>) -> bool {
    for vec_thing in vec.iter() {
        if values == vec_thing {
            return true;
        }
    }
    return false;
}

fn snake_to_display(display_vec: &Vec<char>, snake_vec: &Vec<SnakeNode>) -> Vec<char> {
    let mut new_vec = display_vec.clone();
    let mut first = true;
    let mut first_node: usize = 69;
    for block_thing in snake_vec.iter() {
        if first {
            first_node = (((block_thing.y - 1) * 20 + block_thing.x) - 1) as usize;
            first = false;
        } else {
            new_vec[(((block_thing.y - 1) * 20 + block_thing.x) - 1) as usize] = '@';
        }
    }
    new_vec[first_node] = '&';
    new_vec
}

fn does_snake_die(snake: &Vec<SnakeNode>) -> bool {
    let mut done_ur_mum: Vec<(i32, i32)> = Vec::new();
    for node in snake.iter() {
        for values in done_ur_mum.iter() {
            if &(node.x, node.y) == values {
                return true;
            }
        }
        done_ur_mum.push((node.x, node.y));
    }
    return false;
}

fn yum_yum(snake: &Vec<SnakeNode>, money: &Money) -> bool {
    for node in snake {
        if node.x == money.x && node.y == money.y {
            return true;
        }
    }
    return false;
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}
