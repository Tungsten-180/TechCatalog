fn main() {
    println!("Hello, world!");
}

fn run(){
  screen(Page::HOME);
  loop{
    nav();
  }
}

fn nav(){
  get_input();
  match_input();
}

fn match_input(){}

fn get_input(){}

fn screen(page:Page){
  match(page){
    HOME => {let (one,two,three) = ("View","New","Exit");
    print!("1.{}\n2.{}\n3.{}\n",one,two, three);},
    _ =>{}
  }
}

enum Page{
  HOME,
  VIEW,
  NEW,
  EXIT,
}
