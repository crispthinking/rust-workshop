trait Greetable {
    fn name(&self) -> String;
}

struct Person {
    first: String,
    last: String
}

impl Greetable for Person {
    fn name(&self) -> String {
        format!("{} {}", self.first, self.last)
    }
}

impl Person {
    pub fn new(first: &str, last: &str) -> Self {
        Person {
            first: first.to_string(),
            last: last.to_string(),
        }
    }
}

fn greet<T>(greetee: &T)
    where T: Greetable + ?Sized
{
    println!("Hello, {}", greetee.name());
}

impl Greetable for str {
    fn name(&self) -> String {
        self.to_string()
    }
}

fn main() {
    greet("World");
    let me = Person::new("Crisp", "Thinking");
    greet(&me);
}
