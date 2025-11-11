use gloo::console::log;

pub fn log(fmt: &str, args: &[&str]) -> () {
    let mut to_return = fmt.to_string();
    for (i, arg) in args.iter().enumerate() {
        to_return = to_return.replace(
            &format!("${}", i.to_string()),
            &("`".to_string() + (*arg) + "`"),
        );
    }
    log!(to_return);
}
