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
