use serde_json::Value;
use std::{
    collections::BTreeMap,
    fmt::Display,
    fs::{self, File},
    io::{Read, Write},
    iter::once,
};

static EN_IND: &str = "/home/lyj/.cache/github/wudao-dict/wudao-dict/dict/en.ind";

static EN_Z: &str = "/home/lyj/.cache/github/wudao-dict/wudao-dict/dict/en.z";

#[derive(Debug)]
enum D {
    L(Value),
    Raw(String),
}

#[derive(Debug)]
struct S {
    word: String,
    pronunciation_am: String,
    pronunciation_en: String,
    paraphrase: String,
    pattern: String,
    sentence: D,
}

impl Display for S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.word)?;
        if self.pronunciation_am.is_empty() && self.pronunciation_en.is_empty() {
        } else {
            write!(
                f,
                "\\n{} {}",
                &self.pronunciation_am, &self.pronunciation_en
            )?;
        }
        if !self.paraphrase.is_empty() {
            write!(f, "\\n{}", &self.paraphrase)?;
        }
        if !self.pattern.is_empty() {
            write!(f, "\\n{}", &self.pattern)?;
        }

        write!(f, "\\n",)?;
        match &self.sentence {
            D::L(value) => {
                if let Value::Array(xs) = value {
                    for x in xs {
                        match x {
                            Value::Array(ys) => {
                                for y in ys {
                                    match y {
                                        Value::Null
                                        | Value::Bool(_)
                                        | Value::Number(_)
                                        | Value::Object(_) => {}
                                        Value::String(s) => {
                                            write!(f, "{s}\t")?;
                                        }
                                        Value::Array(zs) => {
                                            for z in zs {
                                                if let Value::String(z) = z {
                                                    write!(f, "{z}")?;
                                                }
                                            }
                                        }
                                    }
                                }
                                write!(f, "\\n")?;
                            }
                            _ => {}
                        }
                    }
                }
            }
            D::Raw(s) => {
                write!(f, "{s}",)?;
            }
        }
        Ok(())
    }
}

fn main() {
    let zdata: Vec<u8> = fs::read(EN_Z).unwrap();
    let ind_data: String = fs::read_to_string(EN_IND).unwrap();

    let x = format!("dummy|{}", zdata.len());
    let v1 = ind_data.lines();
    let v2 = v1.clone().skip(1).chain(once(&x as &str));

    let mut btree_mp: BTreeMap<(String, String), S> = BTreeMap::new();

    for (a, b) in v1.zip(v2) {
        let (word, start) = extract_word(a);
        let (_, end) = extract_word(b);

        let decompressed_data = decode(&zdata[start..end]);
        let s = reorganize(&decompressed_data);

        btree_mp.insert((word.to_lowercase(), word.to_owned()), s);
    }

    let mut file = File::create("wudao.tab").unwrap();

    for ((_, k), v) in btree_mp {
        file.write(format!("{k}\u{0009}{}\n", v.to_string()).as_bytes())
            .unwrap();
    }
}

fn extract_word(s: &str) -> (&str, usize) {
    let (x, y) = s.split_once('|').unwrap();
    (x, y.parse().unwrap())
}

fn decode(x: &[u8]) -> String {
    let mut decoder = flate2::read::ZlibDecoder::new(x);
    let mut decompressed_data = String::new();
    decoder.read_to_string(&mut decompressed_data).unwrap();
    decompressed_data
}

fn reorganize(x: &str) -> S {
    let v: Vec<_> = x.split('|').collect();
    let sentence = match serde_json::from_str(v[8]) {
        Ok(s) => D::L(s),
        Err(_) => D::Raw(v[8].to_owned()),
    };
    S {
        word: v[0].to_owned(),
        pronunciation_am: v[2].to_owned(),
        pronunciation_en: v[3].to_owned(),
        paraphrase: v[5].to_owned(),
        pattern: v[7].to_owned(),
        sentence,
    }
}
