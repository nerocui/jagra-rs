use iced::widget::{ button, column, text, container, self, row, horizontal_space, Column, text_input };
use iced::{ Element, Application, Settings,  Theme, executor, Command, Length };
use uuid::Uuid;
use iced_aw::Modal;
use once_cell::sync::Lazy;

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

pub fn main() -> iced::Result {
    Jagra::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Status {
    Pending,
    Active,
    Completed,
    Abandoned,
}

#[derive(Debug, Clone, Copy)]
enum Page {
    Home,
    Org(Uuid),
    Task(Uuid),
}

#[derive(Debug, Clone)]
struct Jagra {
    page: Page,
    org_name: String,
    new_org_modal_open: bool,
    new_task_modal_open: bool,
    orgs: Vec<Org>,
}

#[derive(Debug, Clone)]
struct Org {
    id: Uuid,
    name: String,
    tasks: Vec<Task>,
}

impl Org {
    fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            tasks: vec![]
        }
    }
}

#[derive(Debug, Clone)]
struct Task {
    id: Uuid,
    title: String,
    status: Status,
    assignee: String,
}

impl Task {
    fn new(title: String, status: Status, assignee: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            status,
            assignee,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    GoHome,
    GoToOrgPage(Uuid),
    GoToTaskPage(Uuid),
    OpenNewOrgModal,
    CloseNewOrgModal,
    CreateNewOrg,
    OpenNewTaskModal,
    CloseNewTaskModal,
    CreateNewTask(String, Status, String),
    NewOrgNameInputChange(String)
}

impl Application for Jagra {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flag: Self::Flags) -> (Self, Command<Message>) {
        (Self {
            page: Page::Home,
            org_name: String::from(""),
            new_org_modal_open: false,
            new_task_modal_open: false,
            orgs: vec![]
        },
        Command::none())
    }

    fn title(&self) -> String {
        String::from("Jagra")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::GoHome => {
                self.page = Page::Home;
                Command::none()
            }
            Message::GoToOrgPage(org) => {
                self.page = Page::Org(org);
                Command::none()
            }
            Message::GoToTaskPage(task) => {
                self.page = Page::Task(task);
                Command::none()
            }
            Message::OpenNewOrgModal => {
                self.new_task_modal_open = false;
                self.new_org_modal_open = true;
                widget::focus_next()
            }
            Message::CloseNewOrgModal => {
                self.new_org_modal_open = false;
                Command::none()
            }
            Message::CreateNewOrg => {
                self.orgs.push(Org::new(String::from(&self.org_name)));
                self.org_name = String::from("");
                Command::none()
            }
            Message::OpenNewTaskModal => {
                self.new_org_modal_open = false;
                self.new_task_modal_open = true;
                Command::none()
            }
            Message::CloseNewTaskModal => {
                self.new_task_modal_open = false;
                Command::none()
            }
            Message::CreateNewTask(title, status, assignee) => {
                match self.page {
                    Page::Org(id) => {
                        let selected_org = self.orgs.iter_mut().find(|org| org.id == id);
                        if let Some(org) = selected_org {
                            org.tasks.push(Task::new(title, status, assignee));
                        }
                    }
                    _ => {}
                }
                Command::none()
            }
            Message::NewOrgNameInputChange(value) => {
                self.org_name = value;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let page = match self.page {
            Page::Home => {
                container(
                    column![
                        row![
                            horizontal_space(Length::Fill),
                            button("New Org")
                            .on_press(Message::OpenNewOrgModal)],
                        Column::with_children(
                            self.orgs
                                .iter()
                                .enumerate()
                                .map(|(_i, org)| {
                                    text(String::from(&org.name)).into()
                                })
                                .collect()
                        )
                    ]
                )
                
            },
            Page::Org(org_id) => {
                container(
                    text("Org")
                )
            },
            Page::Task(task_id) => {
                container(
                    text("Task")
                )
            }
        };

        let content = container(
            column![
                text(self.title())
                    .size(24),
                page
            ]
        )
        .padding(12);

        if self.new_org_modal_open {
            let modal = Some(container(
                column![
                    text("Create new org").size(24),
                    text_input("Name", &self.org_name)
                        .id(INPUT_ID.clone())
                        .on_input(Message::NewOrgNameInputChange),
                    row![
                        button("Close Modal")
                            .on_press(Message::CloseNewOrgModal),
                        button("Create")
                            .on_press(Message::CreateNewOrg),
                    ].spacing(12),
                ].spacing(12)
            )
                .max_height(240)
                .max_width(420));

            Modal::new(content, modal)
                .backdrop(Message::CloseNewOrgModal)
                .on_esc(Message::CloseNewOrgModal)
                .into()
        } else {
            content.into()
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}