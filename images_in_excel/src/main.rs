use std::fs::{self, DirEntry};
use xlsxwriter::Workbook;

#[derive(Debug)]
struct ImagePath {
    id: u32,
    case: String,
    basename: String,
    path: String,
}

impl ImagePath {
    fn new(dir_entry: DirEntry) -> Self {
        let path_buf = dir_entry.path();
        let path = path_buf.to_str().unwrap();
        let basename = path_buf
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split_once(".")
            .unwrap()
            .0;
        let (id, case) = basename.split_once(" ").unwrap();

        ImagePath {
            id: id.parse::<u32>().unwrap(),
            case: case.to_string(),
            path: path.to_string(),
            basename: basename.to_string(),
        }
    }
}

fn get_images_in_order(path: &str) -> Vec<ImagePath> {
    let mut paths: Vec<ImagePath> = fs::read_dir(path)
        .unwrap()
        .map(|r| r.unwrap())
        .map(ImagePath::new)
        .collect();

    paths.sort_by_key(|image_path| image_path.id);
    paths.iter().for_each(|p| println!("{:?}", p));
    paths
}

fn create_sheet_with_images(workbook: &Workbook, img_1: &ImagePath, img_2: Option<&ImagePath>) {
    match workbook.add_worksheet(Some(&img_1.basename)) {
        Ok(mut sheet) => {
            let format_bold = workbook.add_format().set_bold();
            sheet.write_string(0, 0, &img_1.case, Some(&format_bold));

            if let Err(e) = sheet.insert_image(2, 0, &img_1.path) {
                panic!("Could not insert image because of error: {}", e);
            }

            if let Some(img) = img_2 {
                if let Err(e) = sheet.insert_image(83, 0, &img.path) {
                    panic!("Could not insert image because of error: {}", e);
                }
            }
        }
        Err(e) => panic!("Could not create sheet because of error: {}", e),
    }
}

/// Create 2 excel files that contain (2 types of) images spread over different sheets:
/// One containing only a data image on each sheet.
/// The other containing both a data and a report image on each sheet.
fn main() {
    println!("Creating excel files.");
    let data_images = get_images_in_order("./images/data");
    let report_images = get_images_in_order("./images/reports");

    match Workbook::new("output_1.xlsx") {
        Ok(workbook) => {
            data_images
                .iter()
                .for_each(|img| create_sheet_with_images(&workbook, img, None));

            if let Err(e) = workbook.close() {
                panic!("Could not close excel file because of error: {}", e);
            }
        }
        Err(e) => panic!("Could not create excel file because of error: {}", e),
    }

    match Workbook::new("output_2.xlsx") {
        Ok(workbook) => {
            data_images
                .iter()
                .zip(report_images.iter())
                .for_each(|(img1, img2)| create_sheet_with_images(&workbook, img1, Some(img2)));

            if let Err(e) = workbook.close() {
                panic!("Could not close excel file because of error: {}", e);
            }
        }
        Err(e) => panic!("Could not create excel file because of error: {}", e),
    }

    println!("Done!");
}
