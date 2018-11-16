use super::*;


pub(crate) fn solve(_: &[Instruction]) -> Integer {
    let mut bb = 109300;
    let mut hh = 0;

    for ii in 1..=1002 {
        if bb % 2 == 0 {
            hh += 1;
            println!("hh incremented to {} ({} @ iteration #{})", hh, bb, ii);
            bb += 17;
            continue;
        }


        'outer: for dd in 2..=bb/2 {
            for ee in 2..=bb/2 {
                if dd * ee == bb {
                    hh += 1;
                    println!("hh incremented to {} ({} @ iteration #{})", hh, bb, ii);
                    break 'outer;
                }
            }
        }

        bb += 17;
    }

    return hh;
}
