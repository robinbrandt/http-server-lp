#[derive(Debug, PartialEq)]
enum Lang {
  English,
  Spanish,
  Chinese,
  German,
  Texan,
}

struct Greeting {
    message: String,
    lang: Lang,
}

fn main() {
  let mut v :Vec<Greeting> = Vec::new();

  let g : Greeting = Greeting { lang: Lang::English, message: String::from("Hello WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Spanish, message: String::from("Hola WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Texan, message: String::from("Howdy WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Chinese, message: String::from("WasmEdge 你好!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::German, message: String::from("Guten Tag WasmEdge!") };
  v.push(g);

  for e in &v {
    println!("{:?} {}", e.lang, e.message);
  }

  if let Some(greeting) = v.iter().find(|g| g.lang == Lang::German) {
    println!("For German: {}", greeting.message)
  }
}
