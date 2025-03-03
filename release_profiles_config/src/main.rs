
slint::slint!{
    export component MainWindow inherits Window {
        title: "留白";
        min-height: 500px;
        min-width: 900px;
        VerticalLayout {
            Rectangle {
                TextInput {
                    horizontal-alignment: center;
                    vertical-alignment: center;
                    text: "陌上花开，可缓缓归矣。";
                    color: lightgrey;
                    font-size: 50px;
                    font-family: "Microsoft YaHei";
                }
            }
            // Button {
            //     text: "点击我";
            //     width: 120px;
            //     height: 40px;
            //     background: #4CAF50;
            //     border-radius: 4px;
            //     color: white;
            //     font-size: 16px;
                
            //     clicked => {
            //         debug("按钮被点击了");
            //     }
            // }
        }
    }
}
fn main() {
    println!("release_profile_config");
    MainWindow::new().unwrap().run().unwrap();
}
