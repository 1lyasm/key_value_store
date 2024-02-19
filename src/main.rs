use std::io::BufRead;

fn put(kv: &mut Vec<Vec<i64>>, key: i64, val: i64) {
    let mut found = false;
    let mut idx = 0;
    for pair in kv.iter_mut() {
        if pair.get(0).unwrap() == &key {
            found = true;
            break;
        }
        idx += 1;
    }
    if !found {
        kv.push(vec![key, val]);
        println!("Inserted new key ({}) value ({}) pair", key, val);
    } else {
        *kv.get_mut(idx).unwrap().get_mut(1).unwrap() = val;
        println!("Updated value ({}) of old key ({})", val, key);
    }
}

fn del(kv: &mut Vec<Vec<i64>>, key: i64) -> bool {
    let mut found = false;
    let mut idx = 0;
    for pair in kv.iter_mut() {
        if pair.get(0).unwrap() == &key {
            found = true;
            break;
        }
        idx += 1;
    }
    if found {
        kv.remove(idx);
        println!("Key {} was deleted", key);
    } else {
        println!("Key {} was not in key-value pairs and could not be deleted", key);
    }

    found
}

fn get(kv: &Vec<Vec<i64>>, key: i64) -> Option<i64> {
    let mut found = false;
    let mut idx = 0;
    for pair in kv {
        if pair.get(0).unwrap() == &key {
            found = true;
            break;
        }
        idx += 1;
    }
    let mut res = None;
    if found {
        let val = kv.get(idx).unwrap().get(1).unwrap();
        println!("Value ({}) corresponds to key ({})", val, key);
        res = Some(*val);
    } else {
        println!("Key ({}) does not exist", key);
    }
    res
}

fn print(kv: &Vec<Vec<i64>>) {
    println!("Key-value pairs:");
    for pair in kv {
        println!("\t{}: {}", pair.get(0).unwrap(), pair.get(1).unwrap());
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut kv: Vec<Vec<i64>> = vec![];
    let mut lines = std::io::stdin().lock().lines();

    println!("Enter 'q' to exit");

    loop {
        print!("> ");
        std::io::Write::flush(&mut std::io::stdout()).expect("flush failed");

        let line = lines.next().unwrap().unwrap();

        let words = line.split_whitespace().collect::<Vec<&str>>();

        let first = *words.get(0).unwrap();
        match first {
            "get" => {
                let key = words.get(1);
                if key == None {
                    println!("get requires argument");
                } else {
                    let val = get(&kv, key.unwrap().parse::<i64>().unwrap());
                    if val == None {
                        println!("value: None");
                    } else {
                        println!("value: {}", val.unwrap());
                    }
                }
            },
            "put" => {
                let key = words.get(1);
                if key == None {
                    println!("put requires 2 arguments");
                } else {
                    let val = words.get(2);
                    if val == None {
                        println!("put requires 2 arguments");
                    } else {
                        put(&mut kv, key.unwrap().parse::<i64>().unwrap(),
                            val.unwrap().parse::<i64>().unwrap());
                    }
                }
            },
            "del" => {
                let key = words.get(1);
                if key == None {
                    println!("del requires argument");
                } else {
                    del(&mut kv, key.unwrap().parse::<i64>().unwrap());
                }
            }
            "print" => {
                print(&kv);
            },
            "q" => {
                break Ok(());
            },
            _ => {
                println!("command not found");
            }
        }
    }
}

