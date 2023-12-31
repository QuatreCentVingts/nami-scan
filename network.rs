
use std::io::*;

use sysinfo::{Networks,
              System as SystemInfo,
};

pub fn get_receive() -> String {

    let mut receive: u64 = 0;

    let networks = Networks::new_with_refreshed_list();
    for (interface_name, network) in &networks {
        receive = network.received();
    }

    return receive.to_string();

}