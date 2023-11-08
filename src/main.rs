use std::{
    collections::{HashMap, HashSet},
    io::stdin,
    sync::{Mutex, Once},
};

use once_cell::sync::OnceCell;

const SHENG_MU:[&str;23]=["b","p","m","f","d","t","n","l","g","k","h","j","q","x","zh","ch","sh","z","c","s","y","w","r"];

fn get_dataset() -> &'static Mutex<HashMap<(usize, String), HashSet<String>>> {
    static INSTANCE: OnceCell<Mutex<HashMap<(usize, String), HashSet<String>>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let data = include_str!("dataset/idiom.csv");
        let mut reader = csv::Reader::from_reader(data.as_bytes());

        let mut refs: HashMap<(usize, String), HashSet<String>> = HashMap::new();
        for record in reader.records() {
            let record = record.unwrap();
            let idiom = record[4].trim();
            let pinyin = &record[6];
            for (k, v) in pinyin.trim().split(" ").enumerate() {
                refs.entry((k, v.to_string()))
                    .and_modify(|x| {
                        x.insert(idiom.to_string());
                    })
                    .or_insert(HashSet::new());
            }
        }

        Mutex::new(refs)
    })
}

fn get_likely_candidates(pos: i32, chr: &str) -> HashSet<String> {
    let mut res = HashSet::new();
    let dataset = get_dataset().lock().unwrap();

    let condition1=|x:&str|{
        x.find(chr).is_some()
    };

    let condition2=|x:&str|{
        let sheng=SHENG_MU.iter().filter(|a|x.starts_with(*a)).next().unwrap_or_else(||&"");
        
        let yun=&x[sheng.len()..];
        sheng==&chr || yun==chr
    };

    for key in dataset.keys().filter(|x| condition2(&x.1)) {
        if pos >= 0 && key.0 != pos as usize {
            continue;
        }
        res.extend(dataset.get(key).unwrap().clone());
    }
    res
}

fn main() {
    println!(
        "find #{} keys from dataset",
        get_dataset().lock().unwrap().len()
    );
    loop {
        let mut res: HashSet<String> = HashSet::new();
        println!("ready to filter idioms");
        loop {
            let mut inp = String::new();
            stdin().read_line(&mut inp).unwrap();
            let cmd: Vec<_> = inp.trim().split(" ").collect();

            match cmd[0] {
                "locate" => {
                    let pos = cmd[1];
                    let chr = cmd[2];
                    let delta = get_likely_candidates(pos.parse().unwrap(), chr);
                    if res.is_empty() {
                        res = delta;
                    } else {
                        res = res.intersection(&delta).map(|x| x.to_owned()).collect();
                    }
                }
                "remove" => {
                    let chr = cmd[1];
                    let delta = get_likely_candidates(-1, chr);
                    for i in delta {
                        res.remove(&i);
                    }
                }
                "break" => {
                    break;
                },
                _=>{},
            }

            if res.len() > 10 {
                println!("too many candidates ({}), print the random 10", res.len());
            }
            if res.len()==0{
                println!("you have entered dead conditions. please try again.");
                break;
            }
            if res.len()==1{
                println!("result: {}",res.iter().next().unwrap());
                break;
            }
            for (k, v) in res.iter().enumerate() {
                println!("{}", v);
                if k == 10 {
                    break;
                }
            }
        }
    }
}
