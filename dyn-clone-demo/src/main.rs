trait Cloneable {
    fn clone_box(&self) -> Box<dyn Cloneable>;
    fn speak(&self);
}

impl<T> Cloneable for T
where
    T: Clone + 'static + Speak,
{
    fn clone_box(&self) -> Box<dyn Cloneable> {
        Box::new(self.clone())
    }

    fn speak(&self) {
        self.speak_impl()
    }
}

trait Speak {
    fn speak_impl(&self);
}

#[derive(Clone)]
struct Dog;
impl Speak for Dog {
    fn speak_impl(&self) {
        println!("Woof!");
    }
}

fn main() {
    let dog: Box<dyn Cloneable> = Box::new(Dog);
    let dog2 = dog.clone_box();

    dog.speak();
    dog2.speak();
}

// trait Animal {
//     fn speak(&self);
// }

// #[derive(Clone)]
// struct Dog;
// impl Animal for Dog {
//     fn speak(&self) {
//         println!("Woof!");
//     }
// }

// fn main() {
//     let dog: Box<dyn Animal> = Box::new(Dog);
    
//     // ❌ 错误：clone() 方法不在 dyn Animal 的 vtable 中
//     // let dog2 = dog.clone();
// }
