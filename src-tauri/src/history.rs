use crate::commands::{Command, CommandOutput};
use crate::db::DatabaseState;
use crate::Error;
use std::cmp::max;
use tauri::State;

pub struct History {
    command_history: Vec<Box<dyn Command>>,
    index: usize,
    can_redo: bool,
}

impl History {
    pub fn new() -> Self {
        Self {
            command_history: vec![],
            index: 0,
            can_redo: false,
        }
    }

    /// Executes a command and pushes it to history
    pub fn add_command(
        &mut self,
        db: State<'_, DatabaseState>,
        mut command: impl Command + 'static,
    ) -> Result<CommandOutput, Error> {
        match command.execute(db) {
            Ok(output) => {
                // push to end of history or replace until we're at end
                if self.command_history.len() == self.index {
                    self.command_history.push(Box::from(command));
                } else {
                    self.command_history[self.index] = Box::from(command);
                }

                self.index += 1;
                self.can_redo = false;
                Ok(output)
            }
            Err(error) => Err(error),
        }
    }

    pub fn undo(&mut self, db: State<'_, DatabaseState>) -> Option<Result<CommandOutput, Error>> {
        let maybe_command =
            self.command_history
                .get_mut(if self.index == 0 { 0 } else { self.index - 1 });

        if let Some(command) = maybe_command {
            self.index = if self.index == 0 { 0 } else { self.index - 1 };
            self.can_redo = true;
            Some(command.undo(db))
        } else {
            None
        }
    }

    pub fn redo(&mut self, db: State<'_, DatabaseState>) -> Option<Result<CommandOutput, Error>> {
        if !self.can_redo {
            return None;
        }

        // check to see if there is future command to redo
        if let Some(command) = self.command_history.get_mut(self.index + 1) {
            self.index += 1;
            let output = command.execute(db);

            Some(output)
        } else {
            // No more commands to redo
            self.can_redo = false;
            None
        }
    }
}
