
struct Interface<'b, 'a> {
    manager: &'b Manager<'a>
}

impl <'b, 'a:'b> Interface<'b, 'a> {
    pub fn noop(&self) {
        println!("interface consumed {:?}", self.manager);
    }
}
#[derive(Debug)]
struct Manager<'a> {
    text: &'a str
}

struct List<'a> {
    manager: Manager<'a>
}
impl<'a> List<'a> {
    pub fn get_interface<'b>(& 'b mut self) -> Interface<'b, 'a> 
    where 'a: 'b {
        Interface {
            manager:&mut self.manager
        }
    }
} 

fn main () {
    let mut list = List {
        manager: Manager {
            text: "hello"
        }
    };
    list.get_interface().noop(); 

    println!("Interface should be dropped here and the borrow released");

    use_list(&list);
}

fn use_list(list: &List) {
    println!("list consumed {}", list.manager.text)
}