mod reader;
use reader::*;

fn main() {
    println!("Usage: {} o2ma[xxxx]", std::env::args().nth(0).unwrap().as_str());
    let filename: String = std::env::args().nth(1).unwrap();
    let mut ojn_path: String = filename.clone(); ojn_path.push_str(".ojn");
    let mut ojm_path: String = filename.clone(); ojm_path.push_str(".ojm");

    println!("OJN Path: {:}", ojn_path.as_str());
    println!("OJM Path: {:}", ojm_path.as_str());

    ojn::open_ojn(ojn_path.as_str());
    ojm::open_ojm(ojm_path.as_str());
}
