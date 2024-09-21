use iced::{Element, Sandbox, Settings, Theme, Length};
use iced::widget::{container, text_editor, TextEditor, Row, Column, Text, horizontal_rule, horizontal_space};

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
            Message::Edit(action) => {
                self.content.perform(action);
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let input_content = TextEditor::new(&self.content)
            .on_action(Message::Edit)  // 设置消息处理
            .height(Length::Fill);

        let (line, column) = self.content.cursor_position();
        let position = Text::new(format!("{} : {}", line + 1, column + 1));
        let status_bar = Row::new().push(position); // 创建行并添加位置文本

        let container = Column::new()
            .push(input_content)
            .padding(20)// 将输入内容添加到列中
            .push(status_bar);    // 将状态栏添加到列中

        container.into()  // 将容器转换为 Element
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
