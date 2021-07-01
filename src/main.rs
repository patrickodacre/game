use std::io::stdin;

fn main()
{
    loop {
        let visitor_list = [
            Visitor::new("Evan", "Hey, Evan!"),
            Visitor::new("Christian", "Hey, Christian!"),
            Visitor::new("Daniel", "Hey, Daniel!"),
        ];

        let name = get_name().to_lowercase();

        if name == "" {
            println!("Bye!");
            break;
        }

        let known_visitor = visitor_list
            .iter()
            .find(|v| v.name.to_lowercase().contains(name.as_str()));

        match known_visitor {
            Some(v) => println!("{}\n", v.greet()),
            _ => println!("You're not on the list!"),
        }
    }
}

#[derive(Debug)]
struct Visitor
{
    name: String,
    greeting: String,
}

impl Visitor
{
    fn new(name: &str, greeting: &str) -> Self
    {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet(&self) -> &String
    {
        &self.greeting
    }
}

fn get_name() -> String
{
    println!("What's your name?");

    let mut name = String::new();

    stdin().read_line(&mut name).expect("bad input");

    name.trim().to_string()
}
