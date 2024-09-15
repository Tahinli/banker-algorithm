use rand::Rng;
use std::io;

const PROCESS_COUNT: usize = 5;
const RESOURCE_COUNT: usize = 3;
fn main() {
    println!("Hello, world!");

    let mut restart: bool = true;
    while restart {
        println!("Max A Resource");
        let mut resource_input: String = String::new();
        if let Err(err_val) = io::stdin().read_line(&mut resource_input) {
            println!("Failed to Read: {}", err_val);
            return;
        }
        let a: u8 = match resource_input.trim().parse::<u8>() {
            Ok(value) => value,
            Err(err_val) => {
                println!("Failed to Convert: {}", err_val);
                return;
            }
        };

        println!("Max B Resource");
        let mut resource_input: String = String::new();
        if let Err(err_val) = io::stdin().read_line(&mut resource_input) {
            println!("Failed to Read: {}", err_val);
            return;
        }
        let b = match resource_input.trim().parse::<u8>() {
            Ok(value) => value,
            Err(err_val) => {
                println!("Failed to Convert: {}", err_val);
                return;
            }
        };

        println!("Max C Resource");
        let mut resource_input: String = String::new();
        if let Err(err_val) = io::stdin().read_line(&mut resource_input) {
            println!("Failed to Read: {}", err_val);
            return;
        }
        let c = match resource_input.trim().parse::<u8>() {
            Ok(value) => value,
            Err(err_val) => {
                println!("Failed to Convert: {}", err_val);
                return;
            }
        };

        println!("\n\tA = {}\n\tB = {}\n\tC = {}", a, b, c);
        if a == 0 || b == 0 || c == 0 {
            println!("Exit: Zero as a Input Invalid");
            return;
        }

        let mut max_needs_matrix = [[0_u8; RESOURCE_COUNT]; PROCESS_COUNT];
        let mut assigned_resources_matrix = [[0_u8; RESOURCE_COUNT]; PROCESS_COUNT];
        let mut info: (bool, Vec<u8>) = (false, Vec::with_capacity(PROCESS_COUNT));
        while !info.0 {
            for i in 0..PROCESS_COUNT {
                let mut rng = rand::thread_rng();
                let random = rng.gen_range(0..a);
                max_needs_matrix[i][0] = random;
                if random != 0 {
                    assigned_resources_matrix[i][0] = rng.gen_range(0..random);
                } else {
                    assigned_resources_matrix[i][0] = 0;
                }
            }
            for i in 0..PROCESS_COUNT {
                let mut rng = rand::thread_rng();
                let random = rng.gen_range(0..b);
                max_needs_matrix[i][1] = random;
                if random != 0 {
                    assigned_resources_matrix[i][1] = rng.gen_range(0..random);
                } else {
                    assigned_resources_matrix[i][1] = 0;
                }
            }
            for i in 0..PROCESS_COUNT {
                let mut rng = rand::thread_rng();
                let random = rng.gen_range(0..c);
                max_needs_matrix[i][2] = random;
                if random != 0 {
                    assigned_resources_matrix[i][2] = rng.gen_range(0..random);
                } else {
                    assigned_resources_matrix[i][2] = 0;
                }
            }
            info = banker(a, b, c, max_needs_matrix, assigned_resources_matrix);
        }
        println!("Max Needs Matrix");
        print_matrix(max_needs_matrix);
        println!("Assigned Resources Matrix");
        print_matrix(assigned_resources_matrix);
        let mut answers: [u8; PROCESS_COUNT] = [0; PROCESS_COUNT];
        for (answer, correct_answer) in answers.iter_mut().zip(&info.1) {
            println!("Which Process Should be Done Now ?");
            let mut input = String::new();
            if let Err(err_val) = io::stdin().read_line(&mut input) {
                eprintln!("Error: Reading User Input | {}", err_val);
                return;
            }
            let input: u8 = match input.trim().parse() {
                Ok(parsed) => parsed,
                Err(err_val) => {
                    eprintln!("Error: Converting User Input | {}", err_val);
                    return;
                }
            };
            *answer = input;
            if *correct_answer == input {
                println!("Correct");
            } else {
                println!("Wrong it should be = {}", correct_answer);
            }
        }
        println!("Your Answers");
        for answer in answers {
            println!("P{}", answer);
        }
        println!("Correct Answers");
        for correct_answer in info.1 {
            println!("P{}", correct_answer);
        }

        let mut resource_input: String = String::new();
        println!("Press 'r' to Restart");
        if let Err(err_val) = io::stdin().read_line(&mut resource_input) {
            println!("Failed to Read: {}", err_val);
            return;
        }
        resource_input = resource_input.trim().to_string();
        match resource_input.as_str() {
            "r" => {
                restart = true;
                println!("-------------------------------");
            }
            _ => {
                return;
            }
        }
    }
}

fn print_matrix(matrix: [[u8; RESOURCE_COUNT]; PROCESS_COUNT]) {
    for (i, matrix_column) in matrix.iter().enumerate() {
        print!("\n\t Process {}:   ", i);
        for matrix_value in matrix_column {
            if *matrix_value > 99 {
                print!(" ");
            } else if *matrix_value > 9 {
                print!("  ");
            } else {
                print!("   ");
            }
            print!("{}", *matrix_value);
        }
        println!();
    }
}

fn banker(
    a: u8,
    b: u8,
    c: u8,
    max_needs_matrix: [[u8; RESOURCE_COUNT]; PROCESS_COUNT],
    assigned_resources_matrix: [[u8; RESOURCE_COUNT]; PROCESS_COUNT],
) -> (bool, Vec<u8>) {
    let mut a_remaining: u8 = 0;
    let mut b_remaining: u8 = 0;
    let mut c_remaining: u8 = 0;
    let mut remaining_needs_matrix: [[u8; RESOURCE_COUNT]; PROCESS_COUNT] =
        [[0; RESOURCE_COUNT]; PROCESS_COUNT];
    for i in 0..PROCESS_COUNT {
        match a_remaining.checked_add(assigned_resources_matrix[i][0]) {
            Some(result) => {
                a_remaining = result;
            }
            None => {
                return (false, vec![]);
            }
        }
        match b_remaining.checked_add(assigned_resources_matrix[i][1]) {
            Some(result) => {
                b_remaining = result;
            }
            None => {
                return (false, vec![]);
            }
        }
        match c_remaining.checked_add(assigned_resources_matrix[i][2]) {
            Some(result) => {
                c_remaining = result;
            }
            None => {
                return (false, vec![]);
            }
        }
        remaining_needs_matrix[i][0] = max_needs_matrix[i][0] - assigned_resources_matrix[i][0];
        remaining_needs_matrix[i][1] = max_needs_matrix[i][1] - assigned_resources_matrix[i][1];
        remaining_needs_matrix[i][2] = max_needs_matrix[i][2] - assigned_resources_matrix[i][2];
    }
    if a_remaining > a || b_remaining > b || c_remaining > c {
        return (false, vec![]);
    }
    a_remaining = a - a_remaining;
    b_remaining = b - b_remaining;
    c_remaining = c - c_remaining;
    let mut infinite_detection: u8 = 2;
    let mut done: [bool; PROCESS_COUNT] = [false; PROCESS_COUNT];
    let mut q: Vec<u8> = Vec::with_capacity(PROCESS_COUNT);
    while !done[0] || !done[1] || !done[2] || !done[3] || !done[4] {
        infinite_detection -= 1;

        for i in 0..PROCESS_COUNT {
            if !done[i]
                && a_remaining >= remaining_needs_matrix[i][0]
                && b_remaining >= remaining_needs_matrix[i][1]
                && c_remaining >= remaining_needs_matrix[i][2]
            {
                match a_remaining.checked_sub(remaining_needs_matrix[i][0]) {
                    Some(result) => {
                        a_remaining = result;
                    }
                    None => {
                        return (false, vec![]);
                    }
                }
                match a_remaining.checked_add(max_needs_matrix[i][0]) {
                    Some(result) => {
                        a_remaining = result;
                    }
                    None => {
                        return (false, vec![]);
                    }
                }
                match b_remaining.checked_sub(remaining_needs_matrix[i][1]) {
                    Some(result) => {
                        b_remaining = result;
                    }
                    None => {
                        return (false, vec![]);
                    }
                }
                match b_remaining.checked_add(max_needs_matrix[i][1]) {
                    Some(result) => {
                        b_remaining = result;
                    }
                    None => {
                        return (false, vec![]);
                    }
                }
                match c_remaining.checked_sub(remaining_needs_matrix[i][2]) {
                    Some(result) => {
                        c_remaining = result;
                    }
                    None => {
                        return (false, vec![]);
                    }
                }
                match c_remaining.checked_add(max_needs_matrix[i][2]) {
                    Some(result) => {
                        c_remaining = result;
                    }
                    None => {
                        return (false, vec![]);
                    }
                }
                done[i] = true;
                q.push(i as u8);
                infinite_detection = 2;
            }
        }
        if infinite_detection == 0 {
            return (false, q);
        }
    }
    (true, q)
}
