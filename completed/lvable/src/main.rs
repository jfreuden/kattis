fn prompt() -> String {
    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to get input");
    response.trim_end().to_string()
}


fn main() {
    let len = prompt().parse::<u64>().expect("not a number");
    let word = prompt();

    // 2 - contains("l") - contains("lv")
    // neither = 2
    // one of = 1
    // both, correct = 0
    // both, not lv (vl or l***********v) = 1
    println!("{}",
             2 -
                 word.contains("l") as u8 -
                 word.contains("lv") as u8
    );
}
