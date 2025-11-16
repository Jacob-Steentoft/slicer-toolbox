use rfd::FileDialog;
use slint::{PlatformError, SharedString};
use std::path::PathBuf;

slint::include_modules!();

fn main() -> Result<(), PlatformError> {
    let app = App::new()?;

    let weak_app = app.as_weak();
    app.on_file_btn_clicked(move || {
        if let Some(path) = FileDialog::new()
            .set_title("Select folder to import from")
            .pick_folder()
        {
            let app = weak_app.unwrap();

            let (path, ready) = if !path.is_dir() {
                (SharedString::new(), false)
            } else {
                (path.to_str().unwrap().to_string().into(), true)
            };
            app.set_dir_path(path);
            app.set_can_start(ready);
        }
    });

    let weak_app = app.as_weak();
    app.on_start_processing(move || {
        let app_ref = weak_app.unwrap();
        let string = app_ref.get_dir_path();

        let parsed =
            slicer_toolbox_core::parse_from_slicer_data(&PathBuf::from(string.to_string()))
                .unwrap();
    });

    app.run()
}
