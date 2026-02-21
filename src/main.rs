use std::process::exit;

use getopts::Options;

use crate::store::{Name, Store};

mod store;

fn print_usage(program: &str, opts: Options) {
    let brief = format!(
        "Usage: {} [options]\nStore: {}\n{} v{}",
        program,
        store::get_path_of_store_file().to_string_lossy(),
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "NAME");
    opts.optopt(
        "c",
        "custom",
        "use a custom non-default name",
        "CUSTOM_NAME",
    );
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("l", "list", "list all names and their counters");
    opts.optopt(
        "d",
        "default",
        "set a new default name that is not 'sheep'",
        "NEW_DEFAULT",
    );
    opts.optopt("r", "remove", "remove a name from the store", "BAD_NAME");
    opts.optopt(
        "g",
        "get",
        "get next counter for a name without incrementing it",
        "NAME",
    );
    opts.optopt("s", "set", "set counter for a custom name", "NAME");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("{f}");
            print_usage(&program, opts);
            exit(1)
        }
    };

    if matches.opt_present("help") {
        print_usage(&program, opts);
        exit(0)
    }

    let mut store = match Store::open_or_create() {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "The store file {} could neither be found nor created: {e}",
                store::get_path_of_store_file().to_string_lossy()
            );
            exit(2)
        }
    };

    if let Some(value) = matches.opt_str("default") {
        set_default(&mut store, &value);
    } else if matches.opt_present("list") {
        show_list(&mut store)
    } else if let Some(value) = matches.opt_str("remove") {
        remove_name(&mut store, &value);
    } else if let Some(value) = matches.opt_str("get") {
        get_name(&mut store, &value);
    } else if let Some(n_value) = matches.opt_str("set") {
        if let Some(value) = matches.opt_str("custom") {
            let number = match n_value.parse() {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("'{n_value}' is not a number: {e}");
                    exit(4);
                }
            };

            set_counter(&mut store, &value, number);
        } else {
            eprintln!(
                "set option needs to be used with the custom option to specify which name to set"
            );
            print_usage(&program, opts);
            exit(1)
        }
    } else if let Some(value) = matches.opt_str("custom") {
        make_name(&mut store, Some(&value))
    } else {
        make_name(&mut store, None);
    }

    match store.save() {
        Ok(_) => (),
        Err(e) => {
            eprintln!(
                "The store file {} could not be saved: {e}",
                store::get_path_of_store_file().to_string_lossy()
            );
            exit(3)
        }
    }
}

fn get_name(store: &mut Store, name: &Name) {
    let count = store.get_number_for_key(name);
    println!("{name} is at {count}")
}

fn remove_name(store: &mut Store, name: &Name) {
    let count = store.get_number_for_key(name);
    store.remove_name(name);
    assert!(!store.name_counters().contains_key(name));
    println!("{name} ({count}) was removed")
}

fn show_list(store: &mut Store) {
    println!("{:<40} | {:<10}", "Name", "Counter");
    println!("{:=<53}", '=');
    let mut kv: Vec<(&Name, &u32)> = store.name_counters().iter().collect();
    kv.sort();
    for (key, value) in &kv {
        if *key != store.default_name() {
            println!("{:<40} | {:<10}", key, value);
        } else {
            println!("{:<40} | {:<10}", format!("{key}(d)"), value);
        }
    }
    println!("{:=<53}", '=');
    println!(
        "{:<40} | {:<10}",
        format!("{} names", kv.len()),
        format!("{} total", kv.iter().map(|(_, v)| *v).sum::<u32>())
    );
}

fn set_counter(store: &mut Store, name: &Name, number: u32) {
    store.set_counter(name, number);
    if number != 0 {
        assert_eq!(store.name_counters()[name], number);
    }
    println!("{name} is now at {number}");
}

fn make_name(store: &mut Store, name: Option<&Name>) {
    let name: Name = name.unwrap_or(store.default_name()).to_owned();

    let number = store.take_number_for_key(&name);
    println!("{name}{number}");
}

fn set_default(store: &mut Store, new_default: &Name) {
    store.set_default_name(new_default.to_string());
    assert_eq!(store.default_name(), new_default);
    println!("default is now {new_default}");
}
