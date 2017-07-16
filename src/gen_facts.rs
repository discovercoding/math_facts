use rand::thread_rng;
use rand::distributions::{Range, IndependentSample};
use std::cmp;

fn gen_values(num_values: usize, min_value: u32, max_value: u32) -> Vec<u32> {
    let mut rng = thread_rng();
    let mut values = Vec::with_capacity(num_values);
    let between = Range::new(min_value, max_value + 1);
    for _ in 0..num_values {
        values.push(between.ind_sample(&mut rng));
    }
    values
}

fn top_cell(number: u32) -> String {
    format!(
        "<td></td> <td>{}</td> <td style=\"padding:0 0 0 3em;\"></td>",
        number
    )
}

fn op_cell(op: char) -> String {
    format!(
        "<td>{}</td> <td></td> <td style=\"padding:0 0 0 3em;\"></td>",
        op
    )
}

fn bot_cell(number: u32) -> String {
    format!(
        "<td></td> <td>{}</td> <td style=\"padding:0 0 0 3em;\"></td>",
        number
    )
}
fn underline() -> &'static str {
    "<td colspan=\"2\" style=\"padding:0 0 4em 0;\"><hr></td> \
     <td style=\"padding:0 0 0 3em;\"></td>"
}

fn make_row(top: &[u32], op: &[char], bot: &[u32]) -> String {
    let mut row = String::from("<tr>\n");
    for n in top {
        row.push_str(&top_cell(*n));
    }
    row.push_str("\n</tr>\n<tr>\n");
    for o in op {
        row.push_str(&op_cell(*o));
    }
    row.push_str("\n</tr>\n<tr>\n");
    for n in bot {
        row.push_str(&bot_cell(*n));
    }
    row.push_str("\n</tr>\n<tr>\n");
    for _ in top {
        row.push_str(underline());
    }
    row.push_str("\n</tr>\n");
    row
}

fn header() -> &'static str {
    "<!DOCTYPE html>
<html>
<head>
<meta charset=\"UTF-8\">
<style>
body {
    font-family: \"Lucida Console\", Monospace;
    font-size:14pt;
}
td
{
    padding:0 0.5em 0 0.5em;
    text-align: right;
}
</style>
<title>Math Facts</title>
</head>
<body>
<table>
"
}

fn footer() -> &'static str {
    "</table>
</body>
</html>
"
}

fn fix_min_max(min_value: u32, max_value: u32, is_division: bool) -> (u32, u32) {
    if is_division {
        (cmp::max(1, min_value), (max_value as f64).sqrt() as u32)
    } else {
        (min_value, max_value)
    }
}

fn fix_tops(tops: Vec<u32>, bots: &[u32], is_division: bool) -> Vec<u32> {
    if is_division {
        let tops: Vec<_> = tops.iter().zip(bots.iter()).map(|(x, y)| x * y).collect();
        tops
    } else {
        tops
    }
}

pub fn generate_page(
    min_value: u32,
    max_value: u32,
    op: char,
    num_rows: usize,
    num_cols: usize,
) -> String {
    let mut html = String::from(header());

    let (min_value, max_value) = fix_min_max(min_value, max_value, op == '÷');

    let ops = vec![op; num_cols];
    for _ in 0..num_rows {
        let tops = gen_values(ops.len(), min_value, max_value);
        let bots = gen_values(ops.len(), min_value, max_value);
        html.push_str(&make_row(&fix_tops(tops, &bots, op == '÷'), &ops, &bots));
    }

    html.push_str(footer());

    html
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_gen_values() {
        let num_values = 9;
        let min_value = 3;
        let max_value = 77;
        let v = gen_values(num_values, min_value, max_value);
        assert_eq!(v.len(), num_values);
        let v2 = gen_values(num_values, min_value, max_value);
        assert!(v != v2);
        for elem in v {
            assert!(elem >= min_value && elem <= max_value);
        }
        for elem in v2 {
            assert!(elem >= min_value && elem <= max_value);
        }
    }

    #[test]
    fn test_fix_min_max() {
        let min = 0;
        let max = 16;
        let (new_min, new_max) = fix_min_max(min, max, false);
        assert_eq!(min, new_min);
        assert_eq!(max, new_max);

        let (new_min, new_max) = fix_min_max(min, max, true);
        assert_eq!(1, new_min);
        assert_eq!(4, new_max);

        let min = 2;
        let max = 15;
        let (new_min, new_max) = fix_min_max(min, max, false);
        assert_eq!(min, new_min);
        assert_eq!(max, new_max);

        let (new_min, new_max) = fix_min_max(min, max, true);
        assert_eq!(min, new_min);
        assert_eq!(3, new_max);

        let min = 1;
        let max = 10;
        let (new_min, new_max) = fix_min_max(min, max, false);
        assert_eq!(min, new_min);
        assert_eq!(max, new_max);

        let (new_min, new_max) = fix_min_max(min, max, true);
        assert_eq!(min, new_min);
        assert_eq!(3, new_max);
    }

    #[test]
    fn test_fix_tops() {
        let tops1: Vec<u32> = vec![1, 2, 9, 10, 51, 99, 100, 101, 999];
        let tops2 = tops1.clone();
        let bots: Vec<u32> = vec![81, 72, 63, 54, 45, 36, 27, 18, 9];

        let expected_1 = tops1.clone();
        let expected_2: Vec<u32> = vec![81, 144, 567, 540, 2295, 3564, 2700, 1818, 8991];

        let tops1 = fix_tops(tops1, &bots, false);
        assert_eq!(expected_1, tops1);

        let tops2 = fix_tops(tops2, &bots, true);
        assert_eq!(expected_2, tops2);
    }
}
