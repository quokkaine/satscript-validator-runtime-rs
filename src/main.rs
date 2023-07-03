mod lib;
use lib::{encode_fn_sig_and_args, Arg};
mod consts;
use consts::PATH_ROSHAMBO_CONTRACT;
mod utils;
use utils::read_file;

fn main() {
    let fn_sig = "sayHello(string,string)";
    let args = vec![
        Arg::String(b"Hello".to_vec()),
        Arg::String(b"World".to_vec()),
    ];
    let tx_data = encode_fn_sig_and_args(fn_sig, args);
    println!("Txn Data: {:x?}", tx_data);

    // let parsed_data = parse_tx_data(&tx_data);
    // println!("Parsed Data: {:?}", parsed_data);

    // let roshambo = read_file(PATH_ROSHAMBO_CONTRACT).unwrap();
    // let roshambo_bytecode = to_bytecode(&roshambo);
    // println!("Roshambo Contract Bytes: {}", roshambo_bytecode);
}
