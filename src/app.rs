use gtk::glib;
use gtk::prelude::*;

pub fn run() -> glib::ExitCode {
    // GTK4 アプリケーションを作成
    let app = gtk::Application::builder()
        .application_id("org.example.GtkUi")
        .build();

    app.connect_activate(|app| {
        // UI をファイルから読み込む
        let builder = gtk::Builder::from_file("resources/window.ui");

        // ウィンドウを取得
        let window: gtk::ApplicationWindow = builder
            .object::<gtk::ApplicationWindow>("main_window")
            .expect("main_window not found in UI")
            .downcast()
            .expect("Failed to downcast main_window");

        // Application をセット
        window.set_application(Some(app));

        // ウィンドウを表示
        window.present();
    });

    app.run()
}
