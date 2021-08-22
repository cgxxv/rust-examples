#![allow(dead_code)]

struct Person{
    job: Option<Job>,
}

#[derive(Clone,Copy)]
struct Job{
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone,Copy)]
struct PhoneNumber{
    area_code: Option<u8>,
    number: i32,
}

impl Person{
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

fn main() {
    let p = Person{
        job: Some(Job{
            phone_number: Some(PhoneNumber{
                area_code: Some(61),
                number: 123123231,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}
