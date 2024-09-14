use iced::alignment::Alignment;
use iced::widget::{Button, Checkbox, Column, Row, Text, TextInput};
use iced::{Application, Command, Element, Settings};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read};

#[derive(Serialize, Deserialize)]
struct HabitTracker {
    habits: Vec<Habit>,
    new_habit_name: String,
}

#[derive(Serialize, Deserialize)]
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

impl HabitTracker {
    fn save_to_file(&self) -> io::Result<()> {
        let serialized = serde_json::to_string(self)?;
        fs::write("habit_data.json", serialized)?;
        Ok(())
    }

    fn load_from_file() -> io::Result<Self> {
        let mut file = File::open("habit_data.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let deserialized: Self = serde_json::from_str(&contents)?;
        Ok(deserialized)
    }
}

impl Application for HabitTracker {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let instance = Self::load_from_file().unwrap_or_else(|_| Self {
            habits: vec![],
            new_habit_name: String::new(),
        });
        (instance, Command::none())
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
        if let Err(e) = self.save_to_file() {
            eprintln!("Failed to save data: {}", e);
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let add_button = Button::new(Text::new("Add Habit")).on_press(Message::AddHabit);

        let habit_input = TextInput::new("Enter new habit...", &self.new_habit_name)
            .on_input(Message::UpdateNewHabitName)
            .on_submit(Message::SubmitHabit);

        let habits = self.habits.iter().enumerate().fold(
            Column::new().spacing(10).width(iced::Length::Fill),
            |column, (habit_index, habit)| {
                let checkboxes =
                    habit.completed.iter().enumerate().fold(
                        Row::new().spacing(5),
                        |row, (checkbox_index, &completed)| {
                            row.push(Checkbox::new("", completed).on_toggle(move |_| {
                                Message::ToggleHabit(habit_index, checkbox_index)
                            }))
                        },
                    );
                column.push(
                    Row::new()
                        .spacing(10)
                        .push(Text::new(&habit.name))
                        .push(checkboxes),
                )
            },
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
