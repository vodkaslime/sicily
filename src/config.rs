use structopt::StructOpt;

use crate::constants::*;
use crate::result::Result;

/*
 * The Params struct is to gather params input given by commandline
 * when starting the sicily program.
 */
#[derive(StructOpt, Debug)]
#[structopt(
    name = "sicily",
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = "sicily server"
)]
struct Params {
    #[structopt(
        name = "port",
        long = "--port",
        about = "Port of the sicily server."
    )]
    pub port: Option<u16>,

    #[structopt(
        name = "input buffer size",
        long = "--input-buffer-size",
        about = "Server read buffer size in bytes. Must be larger than 0."
    )]
    pub input_buffer_size: Option<usize>,

    #[structopt(
        name = "Identifier bits",
        long = "--id-bits",
        about = "Identifier bits. Must be an integer between 8 to 255."
    )]
    pub id_bits: Option<u8>,
    
    #[structopt(
        name = "virtual node number",
        long = "--virtual-node-number",
        about = "Virtual node number. Must be an integer between 1 to 32."
    )]
    pub virtual_node_number: Option<u8>,
}

/*
 * The Config struct is actually the configurations after parsing from Param.
 */
#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub input_buffer_size: usize,
    pub id_bits: u8,
    pub virtual_node_number: u8,
}

pub fn parse_params() -> Result<Config> {
    let params = Params::from_args();

    /* Parse port. */
    let port = match params.port {
        Some(port) => port,
        None => PORT,
    };

    /* Parse input buffer size. */
    let input_buffer_size = match params.input_buffer_size {
        Some(input_buffer_size) => {
            if input_buffer_size == 0 {
                return Err("Input buffer size cannot be 0.".into());
            }
            input_buffer_size
        },
        None => INPUT_BUFFER_SIZE
    };

    let id_bits = match params.id_bits {
        Some(id_bits) => {
            if id_bits < 8 {
                return Err("Identifier bits cannot be smaller than 8.".into());
            }
            id_bits
        }
        None => ID_BITS
    };

    /* Parse virtual node number. */
    let virtual_node_number = match params.virtual_node_number {
        Some(virtual_node_number) => {
            if virtual_node_number == 0 {
                return Err("Virtual node number cannot be 0.".into());
            }
            if virtual_node_number > 32 {
                return Err("Virtual node number cannot be larger than 32.".into());
            }
            virtual_node_number
        },
        None => VIRTUAL_NODE_NUMBER
    };
    let config = Config {
        port,
        input_buffer_size,
        id_bits,
        virtual_node_number,
    };
    Ok(config)
}