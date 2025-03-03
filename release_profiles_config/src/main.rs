
slint::slint!{
    import { Button } from "std-widgets.slint";

    export component MainWindow inherits Window {
        title: "留白";
        min-height: 600px;
        min-width: 900px;
        VerticalLayout {
            HorizontalLayout {
                Rectangle {
                    height: 20px;
                    width: 50px;
                    drop-shadow-blur: 20px;
                    drop-shadow-color: lightgrey;
                    Button {
                        text: "文件";
                    }
                }
                Rectangle {
                    height: 20px;
                    width: 50px;
                    drop-shadow-blur: 20px;
                    drop-shadow-color: lightgrey;
                    Button {
                        text: "编辑";
                    }
                }
                Rectangle {
                    height: 20px;
                    width: 50px;
                    drop-shadow-blur: 20px;
                    drop-shadow-color: lightgrey;
                    Button {
                        text: "帮助";
                    }
                }
            }
            Rectangle {
                Text {
                    horizontal-alignment: center;
                    vertical-alignment: center;
                    text: "都门帐饮无绪，\n杨柳岸，晓风残月。";
                    color: lightgrey;
                    font-size: 70px;
                    font-family: "Songti SC";
                }
            }
            Rectangle {
                height: 40px;
                width: 70px;
                drop-shadow-blur: 20px;
                drop-shadow-color: lightgrey;
                Button {
                    text: "↻刷新";
                }
            }
        }
    }
}

fn main() {
    println!("release_profile_config");
    MainWindow::new().unwrap().run().unwrap();
}
