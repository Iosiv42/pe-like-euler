use std::collections::HashMap;
use num::integer::gcd;

fn add_perimiter(perimeters: &mut HashMap<isize, usize>, perimeter: isize) {
    if perimeter <= 1000 {
        if let Some(ent) = perimeters.get_mut(&perimeter) {
            *ent += 1;
        } else {
            perimeters.insert(perimeter, 0);
        }
    }
}

fn main() {
    // Ahh, maybe pretty wierd solution, but it's kinda best I can do.
    let mut perimeters = HashMap::new();
    for ap in 1..1000_isize {
        for bp in 1..1000_isize {
            let (a, b, c) = (
                (ap.pow(2) - bp.pow(2)).abs(),
                2*ap*bp,
                ap.pow(2) + bp.pow(2),
            );

            if a == 0 || b == 0 {
                continue;
            }

            let p = a + b + c;
            
            add_perimiter(&mut perimeters, p);

            let f = |x: isize| b as f32 / a as f32 * x as f32;
            let step = a / gcd(a, b);
            for a in (step..).step_by(step as usize) {
                if a > 500 {
                    break;
                }

                let b = f(a) as isize;
                let c = num::integer::sqrt(a.pow(2) + b.pow(2));
                let p = a + b + c;
                add_perimiter(&mut perimeters, p);
            }
        }
    }

    println!("{:?}", perimeters.iter().max_by_key(|ent| ent.1).unwrap());
}
