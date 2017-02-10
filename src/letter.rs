use log::*;


pub fn get(letter: char, options: &super::Options) -> String {
    return match letter {
        'm' => {
            return m(options);
        },
        'x' => {
            return x(options);
        },
        _ => return {
            let mut output = letter.to_string();
            output.push_str("\n");
            return output;
        },
    }
}

fn x(options: &super::Options) -> String {
    vv(format!("Printing an X"), options);
    let num_rows = 8;
    let num_columns = num_rows * 2;
    let mut output = String::new();
    for r in 0..num_rows + 1 {
        for c in 0..num_columns + 1 {

            if c/2 == r && c % 2 == 0 {
                output.push_str("X");
                continue;
            }

            if num_columns - r * 2 == c {
                output.push_str("X");
                continue;
            }

            output.push_str(" ");
        }
        output.push_str("\n");
    }

    return output;
}

fn m(options: &super::Options) -> String {
    vv(format!("Printing an M"), options);
    let num_rows = 8;
    let num_columns = num_rows * 2;
    let mut output = String::new();
    for r in 0..num_rows + 1 {
        for c in 0..num_columns + 1 {
            if c == 0 {
                output.push_str("X");
                continue;
            }

            if c == num_columns {
                output.push_str("X");
                continue;
            }

            if c == r {
                output.push_str("X");
                continue;
            }

            if num_columns - r == c {
                output.push_str("X");
                continue;
            }

            output.push_str(" ");
        }
        output.push_str("\n");
    }

    return output;
}
