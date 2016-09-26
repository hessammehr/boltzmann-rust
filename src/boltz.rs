extern crate rand;

use std::collections::HashMap;
use rand::{thread_rng};
use rand::distributions::{Range, IndependentSample};

type State = HashMap<i32, bool>;

#[derive(Debug)]
struct BMachine {
    state: State,
    connections: Vec<(i32, i32, f32)>
}


fn update(machine: BMachine) -> State {
    let range: Range<f32> = Range::new(0.,1.);
    let mut rng = thread_rng();
    // let previousState = machine.state.clone();
    let mut energy: HashMap<i32, f32> = HashMap::new();

    for &i in machine.state.keys() {
        energy.insert(i, 0.);
    }

    for triples in machine.connections {
        let (i, j, w) = triples;
        let energy1 = *energy.get(&i).unwrap();
        let energy2 = *energy.get(&j).unwrap();
        let delta1 = (*machine.state.get(&j).unwrap() as i32 as f32) * w;
        let delta2 = (*machine.state.get(&i).unwrap() as i32 as f32) * w;
        println!("{}<->{}: energy[{}]: {} + {}, energy[{}]: {} + {}",
                 i, j, i, energy1, delta1, j, energy2, delta2);
        energy.insert(i, energy1 + delta1);
        energy.insert(j, energy2 + delta2);
    }

    let mut new_state = HashMap::new();
    for i in machine.state.keys() {
        let prob = 1.0 / (1.0 + (-1.0 * energy.get(&i).unwrap()).exp());
        let a = range.ind_sample(&mut rng);
        println!("{}, {}", prob, a);
        let new_val = a > prob;

        new_state.insert(*i, new_val);
    }

    return new_state;
}

fn main() {
    let mut state = HashMap::new();
    state.insert(0, true);
    state.insert(1, true);
    state.insert(2, false);
    let connections = vec![(0, 1, 0.5), (1, 2, 0.5), (0, 2, 0.5)];
    let machine = BMachine { state: state, connections: connections };

    println!("State before");
    println!("{:?}", machine.state);
    let new_state = update(machine);

    println!("State after");
    println!("{:?}", new_state);
}
