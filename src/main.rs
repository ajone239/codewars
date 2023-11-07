use codewars::path_finder::path_finder_2::path_finder;
fn main() {
    let result = path_finder(LIL_MAZE);
    println!("{:?}", result);
}

const LIL_MAZE: &str = "\
......\n\
.WWWW.\n\
.W....\n\
.W.WWW\n\
..W...\n\
....W.\
";
