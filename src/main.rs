extern crate lab5;

use lab5::worker;

fn main() {
    worker::get_repos();
    worker::get_branches();
    worker::get_commits();
}