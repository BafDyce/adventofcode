use super::*;


pub(crate) fn solve(components: &[Component]) -> usize {
    let bridge = Bridge {
        last_port: 0,
        strength: 0,
        length: 0,
    };

    build_bridge(bridge, components).0
}

fn build_bridge(bridge: Bridge, remaining_components: &[Component]) -> (usize, usize) {
    let matching_components = remaining_components.iter()
        .filter(|comp| comp.has_port(&bridge.last_port))
        .collect::<Vec<&Component>>();

    if matching_components.is_empty() {
        return (bridge.strength, bridge.length);
    }

    let mut max_length = 0;
    let mut max_strength = 0;
    for component in matching_components {
        let mut new_components = remaining_components.to_vec();
        new_components.remove_item(component);
        let new_bridge = Bridge {
            strength: bridge.strength + component.weight(),
            last_port: component.weight() - bridge.last_port,
            length: bridge.length + 1,
        };

        let (res_length, res_strength) = build_bridge(new_bridge, &new_components);
        if res_length > max_length || res_strength > max_strength {
            max_length = res_length;
            max_strength = res_strength;
        }
    }

    (max_length, max_strength)
}
