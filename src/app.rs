use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

pub fn run() {
    // Application を作る
    let app = Application::builder()
        .application_id("com.example.ui_test")
        .build();

    // activate シグナルに UI を作るクロージャを登録
    app.connect_activate(|app| {
        // ウィンドウを作る
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("GTK4 Rust UI Test")
            .build();

        // シンプルなボタン
        let button = Button::with_label("Press me");
        button.connect_clicked(move |_| {
            println!("Button clicked");
        });

        // ウィンドウにセット
        window.set_child(Some(&button));

        // ウィンドウを表示
        window.present();
    });

    app.run();
}
