fn is_year_leap(year: usize) -> bool {
    let ret = year % 4 == 0;
    return ret ^ (year % 100 == 0 && year % 400 != 0);
}

/// Returns from which day of week given month will start at the next year.
fn next_start_day(
    curr_year: usize, month_no: usize, curr_week_day_no: usize
) -> usize {
    let y;
    if month_no == 0 || month_no == 1 {
        y = curr_year;
    } else {
        y = curr_year + 1;
    }
    
    return curr_week_day_no + 1 + is_year_leap(y) as usize;
}

fn main() {
    // Sorry for bad naming. It represents from which week day number
    // each month of 1901 starts.
    let week_day_starts_of_1901 = [2, 5, 5, 1, 3, 6, 1, 4, 0, 2, 5, 0];
    let mut count = 0;
    for (month_no, start) in week_day_starts_of_1901.iter().enumerate() {
        let mut curr_year = 1901;
        let mut week_day_no = *start;
        while curr_year <= 2000 {
            if week_day_no % 7 == 0 {
                count += 1;
            }
            week_day_no = next_start_day(curr_year, month_no, week_day_no);

            curr_year += 1;
        }
    }
    
    println!("{count}");
}
