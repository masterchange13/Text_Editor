use std::fmt::format;
use iced::{Element, Settings, Theme, Length, Application};
use iced::widget::{text_editor, TextEditor, Row, Column, Text, Button};
use iced::{executor, command, Command};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::io::ErrorKind;
use text_editor::Content;
use rfd::FileDialog;
use std::fs::File;
use std::io::Write;

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor {
    content: Content,
    error: Option<Error>,  // 修正: 指定类型为 Option<Error>
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    FileOpened(Result<Arc<String>, Error>),  // 修正: 添加正确的 Result 类型
    OpenFile,
    SaveFile,
}

impl Application for Editor {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flag: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                content: Content::with_text(include_str!("./main.rs")),
                error: None,
            },
            Command::perform(load_file(default_load_file()), Message::FileOpened),  // 修正: 添加逗号
        )
    }

    fn title(&self) -> String {
        String::from("Text Editor")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
                Command::none()
            }
            Message::FileOpened(Ok(contents)) => {
                self.content = Content::with_text(&contents);
                Command::none()
            }
            Message::FileOpened(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }  // 修正: 添加关闭大括号
            Message::SaveFile => {
                save_path();
                Command::none()
            }
            Message::OpenFile => {
                let ret = open_file();
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let input_content = TextEditor::new(&self.content)
            .on_action(Message::Edit)  // 设置消息处理
            .height(Length::Fill);

        // 顶部状态栏
        let open_file = Button::new(Text::new("Open")).on_press(Message::OpenFile);
        // set depart line
        let depart_line = Text::new(" ");
        let depart_line2 = Text::new(" ");
        let save_file = Button::new(Text::new("Save")).on_press(Message::SaveFile);
        let top_bar = Row::new()
            .push(open_file)
            .push(depart_line)
            .push(save_file)
            .push(depart_line2);

        let (line, column) = self.content.cursor_position();
        let position = Text::new(format!("{} : {}", line + 1, column + 1));
        let status_bar = Row::new().push(position); // 创建行并添加位置文本

        let container = Column::new()
            .push(top_bar)
            .push(input_content)
            .padding(20)  // 将输入内容添加到列中
            .push(status_bar);  // 将状态栏添加到列中

        container.into()  // 将容器转换为 Element
    }

    fn theme(&self) -> Theme {
        Theme::Dark  // 使用预定义的主题
    }

}

fn open_file() -> Option<String> {
    if let Some(path) = FileDialog::new()
        .set_title("Open File")
        .pick_file()
    {
        // 使用 .display() 方法
        Some(path.display().to_string())
    } else {
        None
    }
}

fn save_path() -> Option<String> {
    if let Some(path) = FileDialog::new()
        .set_title("Save")
        .save_file()
    {
        let file_path = path.display().to_string();

        // 创建或打开文件
        if let Ok(mut file) = File::create(&path) {
            // 写入数据到文件
            if let Err(e) = file.write_all(b"Your content goes here") {
                eprintln!("Failed to write to file: {}", e);
                return None;
            }
            Some(file_path)
        } else {
            eprintln!("Failed to create file.");
            None
        }
    } else {
        None
    }
}


#[derive(Debug, Clone)]
enum Error {
    IOFailed(ErrorKind),
}

async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, Error> {
    tokio::fs::read_to_string(path).await
        .map(Arc::new)
        .map_err(|e| Error::IOFailed(e.kind()))
}

fn default_load_file() -> PathBuf {  // 修正: 返回 PathBuf
    PathBuf::from(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR")))
}
