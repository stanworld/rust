mod lib;

use lib::GreenBadFunitureFactory;
use lib::produce;
use lib::WhiteGoodFunitureFactory;
fn main() {
    let x = WhiteGoodFunitureFactory;
    produce(x);
    let y = GreenBadFunitureFactory;
    produce(y);
}