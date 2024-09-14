use iced::{Application, Command, Element, Settings};
use iced::widget::{Button, Text, TextInput, Column, Checkbox, Row};
use iced::alignment::Alignment;

struct HabitTracker {
    habits: Vec<Habit>,
    new_habit_name: String,
}

struct Habit {
    name: String,
    completed: [bool; 20],
}

#[derive(Debug, Clone)]
enum Message {
    AddHabit,
    ToggleHabit(usize, usize),
    UpdateNewHabitName(String),
    SubmitHabit,
}

impl Application for HabitTracker {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            HabitTracker {
                habits: vec![],
                new_habit_name: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Habit Tracker")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AddHabit => {
                if !self.new_habit_name.is_empty() {
                    self.habits.push(Habit {
                        name: self.new_habit_name.clone(),
                        completed: [false; 20],
                    });
                    self.new_habit_name.clear();
                }
            }
            Message::ToggleHabit(habit_index, checkbox_index) => {
                if let Some(habit) = self.habits.get_mut(habit_index) {
                    habit.completed[checkbox_index] = !habit.completed[checkbox_index];
                }
            }
            Message::UpdateNewHabitName(name) => {
                self.new_habit_name = name;
            }
            Message::SubmitHabit => {
                if !self.new_habit_name.is_empty() {
                    self.habits.push(Habit {
                        name: self.new_habit_name.clone(),
                        completed: [false; 20],
                    });
                    self.new_habit_name.clear();
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let add_button = Button::new(Text::new("Add Habit"))
            .on_press(Message::AddHabit);

        let habit_input = TextInput::new(
            "Enter new habit...",
            &self.new_habit_name,
        )
        .on_input(Message::UpdateNewHabitName)
        .on_submit(Message::SubmitHabit);

        let habits = self.habits.iter().enumerate().fold(
            Column::new().spacing(10).width(iced::Length::Fill),
            |column, (habit_index, habit)| {
                let checkboxes = habit.completed.iter().enumerate().fold(
                    Row::new().spacing(5),
                    |row, (checkbox_index, &completed)| {
                        row.push(
                            Checkbox::new("",completed)
                                .on_toggle(move |_| Message::ToggleHabit(habit_index, checkbox_index))
                        )
                    }
                );
                column.push(
                    Row::new()
                        .spacing(10)
                        .push(Text::new(&habit.name))
                        .push(checkboxes)
                )
            }
        );

        Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(Text::new("Habit Tracker").size(30))
            .push(habits)
            .push(habit_input)
            .push(add_button)
            .into()
    }
}

fn main() -> iced::Result {
    HabitTracker::run(Settings::default())
}
