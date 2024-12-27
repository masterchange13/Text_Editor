/**
@Created by Mao on 2024/12/27
@Author:mao
@Github:https://github.com/masterchange13?tab=projects
@Gitee:https://gitee.com/master_change13
 
@File: main.rs
@IDE: RustRover
 
@Time: 2024/12/27 21:29
@Motto:不积跬步无以至千里，不积小流无以成江海，程序人生的精彩需要坚持不懈地积累！
@target: 大厂offer，高年薪

@@ written by GuangZhi Mao

@from:
@code target:
**/

use iced::{Element, Sandbox, Settings};
use iced::widget::{text};
use iced::widget::text_editor::Action::Edit;

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor {

}

#[derive(Debug)]
enum Message {

}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self {

        }
    }

    fn title(&self) -> String {
        String::from("Text Editor")
    }

    fn update(&mut self, message: Message) {
        match message { _ => {} }
    }

    fn view(&self) -> Element<'_, Message> {
        text("Hello, world!").into()
    }
}