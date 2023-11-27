pub fn write_model(s: &str) {
    for i in s.lines() {
        let mut txt = i.to_string();
        txt.push('s');
    }
}
