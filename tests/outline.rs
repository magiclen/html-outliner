use std::{fs::File, io::Read, path::Path};

use html_outliner::Outline;

const DATA_FOLDER: &str = "data";

#[test]
fn test_files_in_the_data_folder() {
    let data_folder = Path::new("tests").join(DATA_FOLDER);

    for dir in data_folder.read_dir().unwrap().map(|dir| dir.unwrap()) {
        if dir.file_type().unwrap().is_file() {
            let file_path = dir.path();
            let file_name = file_path.file_name().unwrap().to_str().unwrap();

            if let Some(file_name) = file_name.strip_suffix(".html") {
                let html_file_path = &file_path;
                let outline_file_path =
                    html_file_path.parent().unwrap().join(format!("{}.txt", file_name));

                let mut html_file = File::open(html_file_path).unwrap();
                let mut html = String::new();

                html_file.read_to_string(&mut html).unwrap();

                let outline = Outline::parse_html(html, 50);

                let outline_text = outline.to_string();

                let mut outline_file = File::open(outline_file_path.as_path()).unwrap();
                let mut outline_file_content = String::new();

                outline_file.read_to_string(&mut outline_file_content).unwrap();

                if outline_file_content.trim().ne(&outline_text) {
                    eprintln!("{}", outline_text);
                    panic!(
                        "The outline text above does not match the outline file: {}",
                        outline_file_path.to_str().unwrap()
                    );
                }
            }
        }
    }
}
