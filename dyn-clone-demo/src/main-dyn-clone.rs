use dyn_clone::DynClone;

trait Animal: DynClone {
    fn speak(&self);
}

// 告诉 dyn-clone 为 Animal 增加 vtable 支持
dyn_clone::clone_trait_object!(Animal);

#[derive(Clone)]
struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

fn main() {
    let dog: Box<dyn Animal> = Box::new(Dog);
    let dog2 = dyn_clone::clone_box(&*dog); // ✅ clone 成功

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
