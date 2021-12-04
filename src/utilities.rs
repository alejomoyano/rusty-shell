pub fn parse_cmd(command: &str) -> (Vec<&str>, &str) {

    let mut temp = command.split_whitespace();
    let program = temp.next().unwrap(); 
    let args : Vec<&str> = temp.collect();

    let ret = (args,program);
    ret
}