pub fn verse(n: u32) -> String {
    let msg_bottle_number: String = if n == 0 {
        "No more bottles".to_string()
    } else {
        format!("{} {}", n, if n == 1 { "bottle" } else { "bottles" })
    };

    let line1: String = format!(
        "{} of beer on the wall, {} of beer.",
        msg_bottle_number, msg_bottle_number.to_lowercase()
    );

    let line2: String = if n == 0 {
        "Go to the store and buy some more, 99 bottles of beer on the wall.".to_string()
    } else if n == 1 {
        "Take it down and pass it around, no more bottles of beer on the wall.".to_string()
    } else {
        format!(
            "Take one down and pass it around, {} bottle{} of beer on the wall.",
            n - 1,
            if n > 2 { "s" } else { "" }
        )
    };

    format!("{}\n{}\n", line1, line2)
}

pub fn sing(start: u32, end: u32) -> String {
    let mut ret: String = "".to_string();
    for i in (end..=start).rev() {
        ret = format!("{}{}{}", ret, verse(i), if i != end { "\n" } else { "" });
    }

    ret
}
