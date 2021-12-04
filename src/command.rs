use crate::location::Location;

#[derive(Debug)]
pub enum Request {
    Lookup {
        key: String,
    },
    Join {
        location: Location,
    },
}

#[derive(Debug)]
pub enum Response {
    Lookup {
        location: Location,
    },
    Join {
        location: Location,
    },
}