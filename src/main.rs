use systemstat::{saturating_sub_bytes, Platform, System};

fn main() {
    let sys = System::new();

    match sys.load_average() {
        Ok(loadavg) => println!(
            "\nLoad average: {} {} {}",
            loadavg.one, loadavg.five, loadavg.fifteen
        ),
        Err(x) => println!("\nLoad average: error: {}", x),
    }

    let connection = sqlite::open(":memory:").unwrap();

    let query = "
    CREATE TABLE users (name TEXT, age INTEGER);
    INSERT INTO users VALUES ('Alice', 42);
    INSERT INTO users VALUES ('Bob', 69);
    ";
    connection.execute(query).unwrap();

    // print all roles
    let mut statement = connection.prepare("SELECT * FROM users").unwrap();
    while let sqlite::State::Row = statement.next().unwrap() {
        println!("name = {}", statement.read::<String, _>(0).unwrap());
        println!("age = {}", statement.read::<i64, _>(1).unwrap());
    }

    // run bash commands
    // redirect the stdout to this program
    let cmd_string = "echo '123 from rust' ";
    let cmd = std::process::Command::new("bash")
        .arg("-c")
        .arg(cmd_string)
        .output()
        .expect("failed to execute process");

    println!("cmd: {}", cmd_string);
    println!("stdout: {}", String::from_utf8_lossy(&cmd.stdout));
}
