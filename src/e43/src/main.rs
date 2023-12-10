use std::collections::HashSet;

fn digitize(num: usize) -> Vec<usize> {
    if num == 0 {
        return vec![0];
    }

    let mut num_cpy = num;
    let mut digits = Vec::new();
    while num_cpy > 0 {
        let digit = num_cpy % 10;
        num_cpy /= 10;
        digits.push(digit);
    }

    digits
}

fn main() {
    // The idea is to find all 3-digit multiplies of 17 and 13 and
    // then find such that they have 2 identical digits. I.e. a % 100 == b / 10.
    // then for given set of numbers repeat it for 13, 11, 7, etc. to 2.
    // then filter numbers and build it to pandigital form.

    let T = |n: usize| -> Vec<usize> {
        let lower = (10. / n as f32).ceil() as usize;
        let upper = (1000. / n as f32).ceil() as usize;
        (lower..upper).map(|i| n*i).collect()
    };

    let mut da: Vec<_> = T(17).into_iter().map(|m| (m, m % 10)).collect();
    let mut num_len = 1;
    for n in [13, 11, 7, 5, 3, 2] {
        let curr_multiplies = T(n);
        let mut new_da = Vec::new();
        for (div, num) in &da {
            for multiple in &curr_multiplies {
                if multiple % 100 == div / 10 {
                    let append_num = (div / 10) % 10;
                    new_da.push((
                        *multiple,
                        append_num * 10_usize.pow(num_len) + num
                    ));
                }
            }
        }

        da = new_da;
        num_len += 1;
    }

    let mut final_nums = HashSet::new();
    for (div, num) in da {
        let mut cand = (div / 10) * 10_usize.pow(num_len) + num;
        let cand_digs = digitize(cand);
        let set_cand_digs: HashSet<usize> = cand_digs.iter().cloned().collect();

        if set_cand_digs.len() == cand_digs.len() && cand_digs.len() == 9 {
            // As soon as all our numbers should be 10-digit,
            // we clearly know that there's now 9-digit number.
            let digit_to_push: usize = (0..=9)
                .filter(|d| !cand_digs.contains(d))
                .collect::<Vec<usize>>()[0];
            cand += 10_usize.pow(9) * digit_to_push;
            final_nums.insert(cand);
        }
    }

    println!("{}\n{:?}", final_nums.iter().sum::<usize>(), final_nums);
}
