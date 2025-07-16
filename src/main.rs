use std::{cmp::Reverse, collections::HashMap};

enum 节点 {
    叶节点 {
        频率: i32,
        字符: String,
    },

    非叶节点 {
        频率: i32,
        子节点: Vec<节点>,
    }
}

fn 频率(x: &节点) -> i32 {
    match x {
        节点::叶节点 { 频率, 字符: _ } => {
            *频率
        },
        节点::非叶节点 { 频率, 子节点: _ } => {
            *频率
        }
    }
}

lazy_static::lazy_static! {
    static ref 当量排序: HashMap<char, Vec<char>> = {
        let mut 当量 = HashMap::new();
        for line in std::fs::read_to_string("dl.txt").unwrap().lines() {
            let mut temp = line.split("\t");
            let mut chars = line.chars();
            let 第一码 = chars.next().unwrap();
            let 第二码 = chars.next().unwrap();
            let _ = temp.next();
            let _当量 = temp.next().unwrap().parse::<f64>().unwrap();
            当量.entry(第一码).or_insert(HashMap::new()).insert(第二码, _当量);
        }
        let mut result = HashMap::new();
        for i in "qwertyuiopasdfghjklzxcvbnm".chars().collect::<Vec<_>>() {
            let mut l = "qwertyuiopasdfghjklzxcvbnm".chars().collect::<Vec<_>>();
            l.sort_by(|a, b| 当量[&i].get(a).unwrap_or(&0.0).total_cmp(当量[&i].get(b).unwrap_or(&0.0)));
            result.insert(i, l);
        }
        result
    };

    static ref 用指排序: Vec<char> = {
        let mut 用指 = HashMap::new();
        for line in std::fs::read_to_string("yz.txt").unwrap().lines() {
            let mut temp = line.split("\t");
            let 分数 = temp.next().unwrap();
            let 键 = temp.next().unwrap().split(' ').collect::<Vec<_>>();
            for 键 in 键 {
                用指.insert(键.chars().next().unwrap(), 分数.parse::<f64>().unwrap());
            }
        }
        let mut result = "qwertyuiopasdfghjklzxcvbnm".chars().collect::<Vec<_>>();
        result.sort_by(|a, b| 用指.get(a).unwrap_or(&0.0).total_cmp(用指.get(b).unwrap_or(&0.0)));
        result
    };
}

fn 节点到文本(x: &节点, 父字符串: String, 根节点: bool) -> String {
    match x {
        节点::叶节点 { 频率: _, 字符 } => {
            format!("{字符}\t{父字符串}")
        },
        节点::非叶节点 { 频率: _, 子节点 } => {
            if 根节点 {
                子节点.iter().zip(用指排序.iter()).map(|(a, b)| 节点到文本(&a, b.to_string(), false)).collect::<Vec<_>>().join("\n")         
            } else {
                子节点.iter().zip(当量排序[&父字符串.chars().rev().next().unwrap()].clone()).map(|(a, b)| 节点到文本(&a, 父字符串.clone() + &b.to_string(), false)).collect::<Vec<_>>().join("\n")
            }
        },
    }
}

fn main() {
    let mut 所有节点 = Vec::new();
    for line in std::fs::read_to_string("zp.txt").unwrap().lines() {
        let mut temp = line.split("\t");
        let 字 = temp.next().unwrap();
        let 频 = temp.next().unwrap().parse::<i32>().unwrap();
        let 节点 = 节点::叶节点 {
            频率: 频,
            字符: 字.to_string(),
        };
        所有节点.push(节点);
    }
    所有节点.sort_by_key(|x| Reverse({
        频率(x)
    }));

    let 第一次构造的节点数 = if (所有节点.len() - 1) % 19 == 0 {
        20
    } else {
        (所有节点.len() - 1) % 19 + 1
    };

    let mut 子节点 = Vec::new();
    for _ in 0..第一次构造的节点数 {
        子节点.push(所有节点.pop().unwrap());
    }
    let 节点 = 节点::非叶节点 { 频率: 子节点.iter().map(|x| 频率(x)).sum::<i32>(), 子节点 };
    let i = match 所有节点.binary_search_by_key(&Reverse(频率(&节点)), |x| Reverse(频率(x))) {
        Ok(i) => i,
        Err(i) => i,
    };
    所有节点.insert(i, 节点);

    while 所有节点.len() != 1 {
        let mut 子节点 = Vec::new();
        for _ in 0..20 {
            子节点.push(所有节点.pop().unwrap());
        }
        let 节点 = 节点::非叶节点 { 频率: 子节点.iter().map(|x| 频率(x)).sum::<i32>(), 子节点 };
        let i = match 所有节点.binary_search_by_key(&Reverse(频率(&节点)), |x| Reverse(频率(x))) {
            Ok(i) => i,
            Err(i) => i,
        };
        所有节点.insert(i, 节点);
    }
    std::fs::write("code.txt", 节点到文本(&所有节点[0], String::new(), true)).unwrap();
}
