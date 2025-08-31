use zellij_tile::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::process::exit;

register_plugin!(State);

#[derive(Default, Serialize, Deserialize, Clone)]
struct FormState {
    program: String,
    branch: String,
    worktree_path: String,
    base: String,
    create_worktree: bool,
    status: String,
}

pub struct State {
    form: FormState,
}

impl Default for State {
    fn default() -> Self {
        let username = std::env::var("USER").unwrap_or_else(|_| "user".into());
        State {
            form: FormState {
                program: "claude".into(),
                branch: format!("{}/zellij-plugin", username),
                worktree_path: String::new(),
                base: "origin/main".into(),
                create_worktree: true,
                status: String::new(),
            },
        }
    }
}

impl ZellijPlugin for State {
    fn load(&mut self, _: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ChangeApplicationState,
            PermissionType::RunCommands,
            PermissionType::OpenTerminalsOrPlugins,
        ]);
        subscribe(&[EventType::Key]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Key(key_with_mod) => {
                if key_with_mod.bare_key == BareKey::Char('q') {
                    close_self();
                }
                if key_with_mod.bare_key == BareKey::Enter {
                    self.launch();
                }
                true
            }
            _ => false,
        }
    }

    fn render(&mut self, rows: usize, _cols: usize) {
        let title = "Agent Fleet";
        let options = ["claude", "aider", "gemini", "custom"];

        let mut lines: Vec<String> = vec![];
        lines.push(format!("=== {} ===", title));
        lines.push(String::from(""));
        lines.push(format!("Program: {}", self.form.program));
        lines.push(format!("Branch:  {}", self.form.branch));
        lines.push(format!("Worktree: {}", if self.form.worktree_path.is_empty() { "<auto>".into() } else { self.form.worktree_path.clone() }));
        lines.push(format!("Base:     {}", self.form.base));
        lines.push(format!("Create worktree: {}", if self.form.create_worktree { "yes" } else { "no" }));
        lines.push(String::from(""));
        lines.push(String::from("Press Enter to Create worktree + Launch"));
        lines.push(String::from("Press q to quit"));
        lines.push(String::from(""));
        if !self.form.status.is_empty() {
            lines.push(format!("Status: {}", self.form.status));
        }

        let max = rows.min(lines.len());
        for i in 0..max { println!("{}", lines[i]); }
    }
}

impl State {
    fn launch(&mut self) {
        // infer repo name for default worktree path if not set
        if self.form.worktree_path.is_empty() {
            // this will expand to ../<repo>-af-<pid>
            let cwd = std::env::var("PWD").unwrap_or_else(|_| ".".into());
            let repo_name = std::path::Path::new(&cwd)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("repo");
            let short_id = std::process::id();
            let parent = std::path::Path::new(&cwd).parent().unwrap_or(std::path::Path::new("."));
            let default_path = parent.join(format!("{}-af-{}", repo_name, short_id));
            self.form.worktree_path = default_path.to_string_lossy().into_owned();
        }

        // prepare command strings
        let worktree_cmd = format!(
            "git worktree add -B {} '{}' {}",
            self.form.branch,
            self.form.worktree_path,
            self.form.base
        );

        let program_cmd = if self.form.program == "custom" || self.form.program.is_empty() {
            "bash".to_string()
        } else {
            self.form.program.clone()
        };

        // open a pane to create the worktree (optional)
        if self.form.create_worktree {
            let mut context = BTreeMap::new();
            context.insert("purpose".into(), "create_worktree".into());
            open_command_pane_near_plugin(
                CommandToRun::new_with_args("bash", vec!["-lc", &worktree_cmd]),
                context,
            );
        }

        // open a pane in the new worktree to launch the program
        open_terminal_near_plugin(&self.form.worktree_path);
        let mut context = BTreeMap::new();
        context.insert("purpose".into(), "launch_agent".into());
        let (cmd_path, cmd_args): (String, Vec<String>) = match shlex::split(&program_cmd) {
            Some(mut parts) if !parts.is_empty() => {
                let path = parts.remove(0);
                (path, parts)
            }
            _ => ("bash".to_string(), vec![]),
        };
        open_command_pane_near_plugin(
            CommandToRun::new_with_args(cmd_path, cmd_args),
            context,
        );

        self.form.status = format!("Launched in {} on {}", self.form.worktree_path, self.form.branch);
    }
}
