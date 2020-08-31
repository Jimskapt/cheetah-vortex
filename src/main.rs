#![allow(clippy::needless_return)]

fn main() {
	let default_items: u32 = 2;
	let default_items_str = default_items.to_string();
	let default_rows: u32 = 10;
	let default_rows_str = default_rows.to_string();
	let default_separator = String::from(" ");

	let mut app = clap::App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.arg(
			clap::Arg::with_name("items_per_row")
				.short('i')
				.long("items")
				.about("Number of items combined per row")
				.takes_value(true)
				.required(false)
				.default_value(&default_items_str),
		)
		.arg(
			clap::Arg::with_name("rows_generation_limit")
				.short('r')
				.long("rows")
				.about("Number of rows generated on each run")
				.takes_value(true)
				.required(false)
				.default_value(&default_rows_str),
		)
		.arg(
			clap::Arg::with_name("separator")
				.long("separator")
				.about("The separator between words")
				.takes_value(true)
				.required(false)
				.default_value(&default_separator),
		)
		.arg(
			clap::Arg::with_name("no-restart")
				.short('s')
				.long("no-restart")
				.about("Does not ask for \"regenerate a new set\" at the end (it closes the program directly)")
				.takes_value(false)
				.required(false)
		)
		.arg(
			clap::Arg::with_name("quiet")
				.short('q')
				.long("quiet")
				.about("Only prompt generated result, without further text (except errors and restart)")
				.takes_value(false)
				.required(false)
		)
		.arg(
			clap::Arg::with_name("INPUT")
				.about("Path of the input file (which contains words)")
				.required(false)
				.default_value("./list.txt"),
		);

	let mut help_text = Vec::new();
	app.write_help(&mut help_text).unwrap();

	let matches = app.get_matches();

	let items_per_row = matches.value_of("items_per_row").unwrap_or_default();
	let generation_limit = matches
		.value_of("rows_generation_limit")
		.unwrap_or_default();
	let separator = matches.value_of("separator").unwrap_or_default();
	let file_path = matches.value_of("INPUT").unwrap_or_default();

	if !matches.is_present("about") {
		if !matches.is_present("quiet") {
			println!(
				"Starting {} V{}",
				env!("CARGO_PKG_NAME"),
				env!("CARGO_PKG_VERSION")
			);
		}

		let items_per_row: u32 = items_per_row.parse::<u32>().unwrap_or_else(|_| {
		eprintln!("\n⚠ WARNING : can not cast items_per_row = {} as unsigned integer, using default value {} instead.", items_per_row, default_items);
		default_items
	});

		let generation_limit: u32 = generation_limit.parse::<u32>().unwrap_or_else(|_| {
		eprintln!("\n⚠ WARNING : can not cast rows_generation_limit = {} as unsigned integer, using default value {} instead.", generation_limit, default_rows);
		default_rows
	});

		let input_requested = std::path::Path::new(&file_path);
		if !input_requested.exists() {
			panic!("the requested file \"{}\" does not exists", &file_path);
		}

		if !matches.is_present("quiet") {
			println!(
				"\nCurrently using input file : {}\n",
				dunce::canonicalize(&input_requested)
					.unwrap_or_default()
					.as_path()
					.to_string_lossy()
			);
		}

		let mut raw_list = std::fs::File::open(file_path)
			.unwrap_or_else(|_| panic!("Cannot open the source file {}", file_path));
		let mut raws_buffer: String = String::new();
		std::io::Read::read_to_string(&mut raw_list, &mut raws_buffer)
			.unwrap_or_else(|_| panic!("Cannot read the source file {}", file_path));
		let raws = raws_buffer.as_str();

		let list: Vec<&str> = raws
			.split('\n')
			.map(|v| {
				return v.trim();
			})
			.filter(|v| !v.is_empty() && !v.starts_with('#'))
			.collect();

		if list.is_empty() {
			panic!("The file {} should not be empty", file_path);
		}

		let mut instruction: String = String::new();

		while instruction == "" || instruction == "ENTER" {
			for _ in 0..generation_limit {
				let mut name: String = "".to_string();
				for _ in 0..items_per_row {
					let id: f64 = rand::random();
					let id: usize = (id * list.len() as f64) as usize;

					if name != "" {
						name.push_str(separator);
					}
					name.push_str(list.get(id).unwrap());
				}
				println!("{}", name);
			}

			if !matches.is_present("no-restart") {
				println!(
				"\nPlease press ENTER to regenerate a new set,\nor something else to gracefully stop this program : "
			);

				std::io::stdin()
					.read_line(&mut instruction)
					.expect("can not read user input");

				instruction = String::from(instruction.trim());
			} else {
				instruction = String::from("q");
			}
		}

		if !matches.is_present("quiet") {
			println!(
				"\nProgram terminated. Thank you for using {} !",
				env!("CARGO_PKG_NAME")
			);
		}
	} else {
		println!("{}", std::str::from_utf8(&help_text).unwrap());
	}
}
