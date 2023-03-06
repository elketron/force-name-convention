use clap::Parser;
use std::fs;

mod nameing_conventions;
mod words_finder;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    string: String,
    #[arg(short, long)]
    convention: NamingConvention,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum NamingConvention {
    None,
    Flat,
    Upper,
    Camel,
    Pascal,
    Snake,
    ScreamingSnake,
    CamelSnake,
    PascalSnake,
    Kebab,
    ScreamingKebab,
    Train,
}

fn main() {
    let args = Args::parse();
    let file_contents = fs::read_to_string("english_words.txt").expect("failed to read file");

    let builder = words_finder::make_builder(file_contents);
    let trie = builder.build();

    let matches = words_finder::get_words(args.string, trie);

    let output: String = match args.convention {
        NamingConvention::None => nameing_conventions::no_convention(matches),
        NamingConvention::Flat => nameing_conventions::flat(matches),
        NamingConvention::Upper => nameing_conventions::upper(matches),
        NamingConvention::Camel => nameing_conventions::lower_camel(matches),
        NamingConvention::Snake => nameing_conventions::snake(matches),
        NamingConvention::Kebab => nameing_conventions::kebab(matches),
        NamingConvention::Train => nameing_conventions::train(matches),
        NamingConvention::Pascal => nameing_conventions::pascal(matches),
        NamingConvention::CamelSnake => nameing_conventions::camel_snake(matches),
        NamingConvention::PascalSnake => nameing_conventions::pascal_snake(matches),
        NamingConvention::ScreamingSnake => nameing_conventions::screaming_snake(matches),
        NamingConvention::ScreamingKebab => nameing_conventions::screaming_kebab(matches),
    };

    println!("{:?}", output);
}
