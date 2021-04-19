use std::mem::MaybeUninit;   // Can also use lazy_static macro from lazy_static crate
use std::sync::{Mutex, Once};

/*

public class Singleton {

    private Singleton() {}

    private static Singleton instance = new Singleton();

    public static Singleton getInstance() {
        return instance;
    }
}

*/


#[derive(Debug)]
struct Foo {
    test_sr: String,
}

fn get_config() -> &'static Mutex<Foo> {
    static mut CONF: MaybeUninit<Mutex<Foo>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    ONCE.call_once(|| unsafe {
        CONF.as_mut_ptr().write(Mutex::new(Foo {
            test_sr: "test foo called once".to_string(),
        }));
    });

    unsafe { &*CONF.as_ptr() }
}

fn main() {
    let f1 = get_config();
    println!("{:?} first init", f1);
    {
        let mut foo_obj = f1.lock().unwrap();
        foo_obj.test_sr = "test2".to_string();
    }

    let f2 = get_config();
    println!("{:?}, after modified singleton", f2);

    let foo_obj_2 = f2.lock().unwrap();

    assert_eq!(foo_obj_2.test_sr, "test2".to_string())
}
