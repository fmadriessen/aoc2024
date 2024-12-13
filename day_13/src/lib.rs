use regex::Regex;

pub mod part1;
pub mod part2;

#[derive(Clone, Copy, Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

fn solve(machine: &Machine, offset: i64) -> i64 {
    // Each machine can be described as a system of linear equations,
    // Ax = b, where A = [a_x b_x; a_y b_y]; b = [p_x; p_y], x = [n_a; n_b]
    // where [a_x; a_y] and [b_x; b_y] are the translations given with each press of button A and
    // B, respectively, and n_a and n_b denote the number of presses of each button.
    //
    // x = inv(A) b
    // n_a = (p_x b_y - p_y b_x) / det(A);
    // n_b = (p_y a_x - p_x a_y) / det(A);
    let det = machine.a.0 * machine.b.1 - machine.a.1 * machine.b.0;
    let prize = (machine.prize.0 + offset, machine.prize.1 + offset);
    let n_a = (prize.0 * machine.b.1 - prize.1 * machine.b.0) / det;
    let n_b = (prize.1 * machine.a.0 - prize.0 * machine.a.1) / det;
    if (
        machine.a.0 * n_a + machine.b.0 * n_b,
        machine.a.1 * n_a + machine.b.1 * n_b,
    ) == prize
    {
        3 * n_a + n_b
    } else {
        0
    }
}

fn parse(input: &str) -> Vec<Machine> {
    let re_parameters = Regex::new(r"(?<x>\d+).*?(?<y>\d+)").unwrap();
    let input: Vec<_> = input
        .split("\n\n")
        .map(|machine| {
            let params = machine
                .split("\n")
                .map(|line| {
                    let params = re_parameters.captures(line).unwrap();
                    let x = params["x"].parse::<i64>().unwrap();
                    let y = params["y"].parse::<i64>().unwrap();
                    (x, y)
                })
                .collect::<Vec<_>>();

            Machine {
                a: params[0],
                b: params[1],
                prize: params[2],
            }
        })
        .collect();

    input
}
