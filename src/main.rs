use codewars::path_finder::path_finder_1::path_finder;
fn main() {
    let result = path_finder(MAZE);
    println!("{:?}", result);
}

const LIL_MAZE: &str = "\
......\n\
.WWWW.\n\
.W....\n\
.W....\n\
.....W\n\
....W.\
";
