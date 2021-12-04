use crate::location::Location;

#[derive(Debug)]
pub enum Request {
    Get{
        key: String,
    },
    Join{
        location: Location,
    },
}

#[derive(Debug)]
pub enum Response {
    Get{
        location: Location,
    },
    Join{
        location: Location,
    },
}