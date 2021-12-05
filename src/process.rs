use num::bigint::BigUint;

use crate::arithmetic;
use crate::location::Location;

pub fn find_successor(location: &Location, id: &BigUint) -> Location {
    let pred = find_predecessor(location, id);
    return get_successor(&pred);
}

fn find_predecessor(location: &Location, id: &BigUint) -> Location {
    let mut location = location.clone();
    while !arithmetic::is_in_range(
        id,
        (&location.identifier, false),
        (&get_successor(&location).identifier, true)
    ) {
        location = find_closest_preceding_finger(&location, &id);
    }
    location
}

fn get_successor(location: &Location) -> Location {
    location.clone()
}

fn find_closest_preceding_finger(location: &Location, id: &BigUint) -> Location {
    location.clone()
}