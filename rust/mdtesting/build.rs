#![feature(let_chains)]

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let articles_path = Path::new("..");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("md_tests_generated.rs");

    let vec_of_md_articles = fs::read_dir(articles_path)
        .unwrap()
        .map(|e| e.unwrap())
        .filter(|e| e.file_type().unwrap().is_file())
        .filter(|e| e.file_name().into_string().unwrap().ends_with(".md"))
        .map(|e| {
            (
                e.file_name()
                    .to_str()
                    .unwrap()
                    .to_owned()
                    .replace(".md", ""),
                std::fs::read_to_string(e.path()).unwrap(),
            )
        })
        .collect::<Vec<(String, String)>>();

    for (_, art) in vec_of_md_articles.iter() {
        dbg!(format!("{}", art));
    }

    let vec_of_rust_code = vec_of_md_articles
        .into_iter()
        .map(|(name, article)| (name, extract_code_from_article(&article)))
        .collect::<Vec<(String, Vec<String>)>>();

    dbg!(&vec_of_rust_code);

    let docs: Vec<(String, Vec<String>)> = vec_of_rust_code
        .into_iter()
        .map(|(name, codes)| {
            (
                name,
                codes
                    .into_iter()
                    .map(|s| {
                        let mut string = String::from("/// ```rust\n".to_owned());
                        string.extend(s.lines().skip(1).map(|line| format!("/// {}\n", line)));
                        string.push_str("/// ```\n///\n");
                        string
                    })
                    .collect(),
            )
        })
        .collect::<Vec<_>>();

    let mut final_string = String::new();

    for (name, docs) in docs {
        for doc in docs {
            final_string.push_str(&doc);
        }
        final_string.push_str(&format!("fn helper_{}() {{}}\n\n", name));
    }

    fs::write(dest_path, final_string).expect("Unable to write md_tests_generated.rs");

    println!("cargo:rerun-if-changed=__non_existing");
}

fn extract_code_from_article(article: &str) -> Vec<String> {
    let mut vec = Vec::new();

    let strings: Vec<String> = article.split("```").map(|s| s.to_string()).collect();

    for (i, st) in strings.iter().enumerate() {
        if i != 0
            && let Some(prev) = strings.get(i - 1)
            && i % 2 == 1
        {
            if !(prev.ends_with("<!--ignore-->\r\n")
                || prev.ends_with("<!-- ignore -->\r\n")
                || prev.ends_with("<!--ignore-->\n")
                || prev.ends_with("<!--ignore-->\r\n")
                || prev.ends_with("<!-- ignore -->\n"))
            {
                vec.push(st.clone())
            }
        }
    }
    vec
}
