use std::{collections::HashMap, sync::Mutex};

use delay_init::delay;

static FLAG: Mutex<bool> = Mutex::new(false);

delay! {
    static S : u8 = {
        *FLAG.lock().unwrap() = true;
        0
    };
}

#[test]
fn test() {
    assert_eq!(*FLAG.lock().unwrap(), false);
    assert_eq!(*S, 0);
    assert_eq!(*FLAG.lock().unwrap(), true);
}

delay! {
    static MAP : HashMap<i8,String> = {
        let mut map = HashMap::new();
        map.insert(1,"1".to_owned());
        map.insert(-1,"-1".to_owned());
        map
    };
}

#[test]
fn map() {
    assert_eq!(MAP.get(&1).unwrap(), "1");
    assert_eq!(MAP.get(&-1).unwrap(), "-1");
}
