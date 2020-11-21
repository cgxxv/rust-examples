use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    simulated_expensive_calculation(1);

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
    generate_workout_withhashmap(simulated_user_specified_value, simulated_random_number);

    let simulated_user_specified_value = 'a';
    let simulated_random_number = 'b';
    generate_workout_withgeneric(simulated_user_specified_value, simulated_random_number);
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    //let expensive_result = simulated_expensive_calculation(intensity);
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating fast...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

pub fn generate_workout_withhashmap(intensity: u32, random_number: u32) {
    //let expensive_result = simulated_expensive_calculation(intensity);
    let mut expensive_result = CacherHashMap::new(|num| {
        println!("calculating fast with hashmap...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

pub fn generate_workout_withgeneric(intensity: char, random_number: char) {
    //let expensive_result = simulated_expensive_calculation(intensity);
    let mut expensive_result = CacherGeneric::new(|num| {
        println!("calculating fast with generic...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity == 'a' {
        println!(
            "Today, do plan {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do plan {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 'b' {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for plan {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

use std::collections::HashMap;
struct CacherHashMap<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> CacherHashMap<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> CacherHashMap<T> {
        CacherHashMap {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v.clone());
                v
            }
        }
    }
}

struct CacherGeneric<T, U: Copy>
where
    T: Fn(U) -> U,
{
    calculation: T,
    value: Option<U>,
}

impl<T, U: Copy> CacherGeneric<T, U>
where
    T: Fn(U) -> U,
{
    fn new(calculation: T) -> CacherGeneric<T, U> {
        CacherGeneric {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: U) -> U {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let _ = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
