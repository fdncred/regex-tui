use crate::app::{App, AppResult, CurrentField};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use regex::Regex;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if key_event.kind == KeyEventKind::Press {
        if key_event.modifiers == KeyModifiers::CONTROL {
            match key_event.code {
                KeyCode::Char('u') | KeyCode::Char('U') => {
                    match app.current_input {
                        CurrentField::Regex => {
                            app.regex.clear();
                            app.regex_cursor_pos.x = 0;
                        }
                        CurrentField::Text => {
                            app.text[app.text_cursor_pos.y as usize].clear();
                            app.text_cursor_pos.x = 0;
                        }
                        CurrentField::Matches => {
                            app.matches[app.matches_cursor_pos.y as usize].clear();
                            app.matches_cursor_pos.x = 0;
                        }
                    }
                    return Ok(());
                }
                _ => (),
            }
        }
        match key_event.code {
            // exit application on ESC
            KeyCode::Esc => {
                app.running = false;
            }
            KeyCode::Char(c) => {
                match app.current_input {
                    CurrentField::Regex => {
                        app.regex.insert(app.regex_cursor_pos.x as usize, c);
                        app.regex_cursor_pos.x += 1;
                    }
                    CurrentField::Text => {
                        app.text[app.text_cursor_pos.y as usize]
                            .insert(app.text_cursor_pos.x as usize, c);
                        app.text_cursor_pos.x += 1;
                    }
                    CurrentField::Matches => {
                        app.matches[app.matches_cursor_pos.y as usize]
                            .insert(app.matches_cursor_pos.x as usize, c);
                        app.matches_cursor_pos.x += 1;
                    }
                };
                update_output(app);
            }
            KeyCode::Enter => {
                if app.current_input.is_regex() {
                    app.regex.push_str(&String::new());
                    app.regex_cursor_pos.y += 1;
                    app.regex_cursor_pos.x = 0;
                } else if app.current_input.is_text() {
                    app.text.push(String::new());
                    app.text_cursor_pos.y += 1;
                    app.text_cursor_pos.x = 0;
                } else if app.current_input.is_matches() {
                    app.matches.push(String::new());
                    app.matches_cursor_pos.y += 1;
                    app.matches_cursor_pos.x = 0;
                }
                update_output(app);
            }
            KeyCode::Tab => {
                app.current_input.next();
                match app.current_input {
                    CurrentField::Regex => app.regex_cursor_pos.x = app.regex.len() as u16,
                    CurrentField::Text => {
                        app.text_cursor_pos.x =
                            app.text[app.text_cursor_pos.y as usize].len() as u16
                    }
                    CurrentField::Matches => {
                        if !app.matches.is_empty() {
                            app.matches_cursor_pos.x =
                                app.matches[app.matches_cursor_pos.y as usize].len() as u16
                        }
                    }
                }
            }
            KeyCode::Backspace => match app.current_input {
                CurrentField::Regex => {
                    if app.regex_cursor_pos.x > 0 {
                        app.regex.remove(app.regex_cursor_pos.x as usize - 1);
                        app.regex_cursor_pos.x -= 1;
                    }
                    update_output(app);
                }
                CurrentField::Text => {
                    let line = &mut app.text[app.text_cursor_pos.y as usize];
                    if app.text_cursor_pos.x == 0 {
                        if app.text_cursor_pos.y != 0 {
                            app.text.remove(app.text_cursor_pos.y as usize);
                            app.text_cursor_pos.y -= 1;
                            app.text_cursor_pos.x =
                                app.text[app.text_cursor_pos.y as usize].len() as u16;
                        }
                    } else {
                        line.remove(app.text_cursor_pos.x as usize - 1);
                        app.text_cursor_pos.x -= 1;
                    }
                    update_output(app);
                }
                CurrentField::Matches => {
                    let line = &mut app.matches[app.matches_cursor_pos.y as usize];
                    if app.matches_cursor_pos.x == 0 {
                        if app.matches_cursor_pos.y != 0 {
                            app.matches.remove(app.matches_cursor_pos.y as usize);
                            app.matches_cursor_pos.y -= 1;
                            app.matches_cursor_pos.x =
                                app.matches[app.matches_cursor_pos.y as usize].len() as u16;
                        }
                    } else {
                        line.remove(app.matches_cursor_pos.x as usize - 1);
                        app.matches_cursor_pos.x -= 1;
                    }
                    update_output(app);
                }
            },
            KeyCode::Up => {
                if app.current_input.is_text() && app.text_cursor_pos.y > 0 {
                    app.text_cursor_pos.y -= 1;
                    app.text_cursor_pos.x = app.text[app.text_cursor_pos.y as usize].len() as u16;
                } else if app.current_input.is_matches() && app.matches_cursor_pos.y > 0 {
                    app.matches_cursor_pos.y -= 1;
                    app.matches_cursor_pos.x =
                        app.matches[app.matches_cursor_pos.y as usize].len() as u16;
                }
            }
            KeyCode::Down => {
                if app.current_input.is_text() && app.text_cursor_pos.y < app.text.len() as u16 - 1
                {
                    app.text_cursor_pos.y += 1;
                    app.text_cursor_pos.x = app.text[app.text_cursor_pos.y as usize].len() as u16;
                } else if app.current_input.is_matches()
                    && app.matches_cursor_pos.y < app.matches.len() as u16 - 1
                {
                    app.matches_cursor_pos.y += 1;
                    app.matches_cursor_pos.x =
                        app.matches[app.matches_cursor_pos.y as usize].len() as u16;
                }
            }
            KeyCode::Left => {
                if app.current_input.is_regex() && app.regex_cursor_pos.x > 0 {
                    app.regex_cursor_pos.x -= 1;
                } else if app.current_input.is_text() && app.text_cursor_pos.x > 0 {
                    app.text_cursor_pos.x -= 1;
                } else if app.current_input.is_matches() && app.matches_cursor_pos.x > 0 {
                    app.matches_cursor_pos.x -= 1;
                }
            }
            KeyCode::Right => {
                let line = match app.current_input {
                    CurrentField::Regex => &app.regex,
                    CurrentField::Text => &app.text[app.text_cursor_pos.y as usize],
                    CurrentField::Matches => &app.matches[app.matches_cursor_pos.y as usize],
                };

                if app.current_input.is_regex() && app.regex_cursor_pos.x < line.len() as u16 {
                    app.regex_cursor_pos.x += 1;
                } else if app.current_input.is_text() && app.text_cursor_pos.x < line.len() as u16 {
                    app.text_cursor_pos.x += 1;
                } else if app.current_input.is_matches()
                    && app.matches_cursor_pos.x < line.len() as u16
                {
                    app.matches_cursor_pos.x += 1;
                }
            }
            _ => {}
        }
    }

    Ok(())
}

pub fn update_output(app: &mut App) {
    if app.current_input == CurrentField::Regex {
        if !app.regex.is_empty() {
            match Regex::new(&app.regex) {
                Ok(re) => app.re = Some(re),
                Err(e) => {
                    app.matches = vec![e.to_string()];
                    return;
                }
            }
        } else {
            app.re = None;
            app.matches.clear();
        }
    }

    if let Some(re) = &app.re {
        let mut new_output = vec![String::new()];

        let joined = app.text.join("\n");
        let captures = re.captures_iter(&joined);
        for (i, capture) in captures.enumerate() {
            new_output.push(format!("{}.\n", i));

            let mut capture_names = re.capture_names();
            for (i, m) in capture.iter().enumerate() {
                let m = m.unwrap();
                match capture_names.next().unwrap() {
                    Some(name) => new_output.push(format!("  {}: {}\n", name, m.as_str())),
                    None => new_output.push(format!("  {}: {}\n", i, m.as_str())),
                }
            }
        }

        app.matches = new_output;
    }
}
