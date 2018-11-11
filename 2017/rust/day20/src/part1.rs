use super::*;

pub fn solve(particles: &Vec<Particle>) -> usize {
    // should be long-term enough
    let time = 1_000_000;

    let mut min_dist = std::i64::MAX;
    let mut min_particle = 0;
    for ii in 0..particles.len() {
        let Coordinate {xx, yy, zz} = particles[ii].get_pos_at(time);

        let dist = i64::abs(xx) + i64::abs(yy) + i64::abs(zz);
        if dist < min_dist {
            min_dist = dist;
            min_particle = ii;
        }
    }

    min_particle
}
