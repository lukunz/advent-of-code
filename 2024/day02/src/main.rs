fn main() {
    let data = include_str!("../day02.txt");
    let reports = data
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut safe_reports = 0;
    let mut safe_reports_with_margin = 0;

    for report in reports {
        if is_report_safe(&report) {
            safe_reports += 1;
        } else {
            for i in 0..report.len() {
                let mut tmp_report = report.clone();
                tmp_report.remove(i);

                if is_report_safe(&tmp_report) {
                    safe_reports_with_margin += 1;
                    break;
                }
            }
        }
    }

    println!("Day 02 Part 1: {}", safe_reports);
    println!("Day 02 Part 2: {}", safe_reports + safe_reports_with_margin);
}

fn is_report_safe(report: &[u32]) -> bool {
    if report.is_sorted() || report.iter().rev().is_sorted() {
        for numbers in report.windows(2) {
            assert_eq!(numbers.len(), 2);

            let diff = numbers[0].abs_diff(numbers[1]);
            if diff == 0 || diff > 3 {
                return false;
            }
        }

        true
    } else {
        false
    }
}
