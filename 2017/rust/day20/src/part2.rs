use super::*;

pub fn solve(particles: &Vec<Particle>) -> usize {

    let mut particles = (*particles).clone();

    for _ in 0..1_000 {
        for ii in 0..particles.len() {
            particles[ii].update();
        }

        particles.sort();
        let mut colliding_items: Vec<Particle> = particles.windows(2).filter_map(|items| {
            if items[0] == items[1] {
                Some(items[0].clone())
            } else {
                None
            }
        })
        .collect();
        colliding_items.dedup();

        if colliding_items.len() > 0 {
            particles.drain_filter(|particle| {
                colliding_items.contains(particle)
            });
        }
    }

    particles.len()
}
