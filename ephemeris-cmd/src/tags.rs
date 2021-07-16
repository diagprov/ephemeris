
#[inline(always)]
pub fn tag_to_string(tlist: &Vec<String>) -> String {
    let mut s : String = String::from("");
    for t in tlist {
        if &s == "" {
            s = format!("{}", t);
        } else {
            s = format!("{}, {}", s, t);
        }
    }
    s
}
