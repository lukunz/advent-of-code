fn main() {
    // let data = include_str!("../day02.txt");
    let data = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let reports = data
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut safe_reports = 0;

    for report in reports {
        if report.is_sorted() || report.iter().rev().is_sorted() {
            let mut report_is_safe = true;
            for numbers in report.windows(2) {
                assert_eq!(numbers.len(), 2);

                let diff = numbers[0].abs_diff(numbers[1]);
                if diff == 0 || diff > 3 {
                    report_is_safe = false;
                    break;
                }
            }

            if report_is_safe {
                safe_reports += 1;
            }
        }
    }

    println!("Day 02 Part 1: {}", safe_reports);
}
