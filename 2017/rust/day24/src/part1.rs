use super::*;


pub(crate) fn solve(components: &[Component]) -> usize {
    let bridge = Bridge {
        last_port: 0,
        strength: 0,
        length: 0,
    };

    build_bridge(bridge, components)
}

fn build_bridge(bridge: Bridge, remaining_components: &[Component]) -> usize {
    let matching_components = remaining_components.iter()
        .filter(|comp| comp.has_port(&bridge.last_port))
        .collect::<Vec<&Component>>();

    if matching_components.is_empty() {
        return bridge.strength;
    }

    let mut max = 0;
    for component in matching_components {
        let mut new_components = remaining_components.to_vec();
        new_components.remove_item(component);
        let new_bridge = Bridge {
            strength: bridge.strength + component.weight(),
            last_port: component.weight() - bridge.last_port,
            length: 0,
        };

        let res = build_bridge(new_bridge, &new_components);
        if res > max {
            max = res;
        }
    }

    max
}
