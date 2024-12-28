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

use iced::{Element, Length, Sandbox, Settings, Theme};
use iced::widget::{row, text, text_editor, container, column, horizontal_space};
use iced::widget::text_editor::Action::Edit;

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor {
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self {
            content: text_editor::Content::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Text Editor")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => self.content.perform(action),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let input_content = text_editor(&self.content).on_action(Message::Edit).height(Length::Fill);

        let position = {
            let (line, column) = &self.content.cursor_position();
            text(format!("row:{}, col:{}", line+1, column+1))
        };
        let status_bar = row!(horizontal_space(), position);

        container(column!(input_content, status_bar)).padding(10).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}