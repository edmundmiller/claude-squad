use zellij_tile::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
    repo_root: Option<String>,
    log_path: Option<String>,
}

impl Default for State {
    fn default() -> Self {
        let username = std::env::var("USER").unwrap_or_else(|_| "user".into());
        let short_id = std::process::id();
        State {
            form: FormState {
                program: "claude".into(),
                branch: format!("{}/af-{}", username, short_id),
                worktree_path: String::new(),
                base: "origin/main".into(),
                create_worktree: true,
                status: String::new(),
            },
            repo_root: None,
            log_path: None,
        }
    }
}

impl ZellijPlugin for State {
    fn load(&mut self, _: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ChangeApplicationState,
            PermissionType::RunCommands,
            PermissionType::OpenTerminalsOrPlugins,
            PermissionType::OpenFiles,
        ]);
        subscribe(&[EventType::Key, EventType::RunCommandResult]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Key(key_with_mod) => {
                if key_with_mod.bare_key == BareKey::Char('q') {
                    close_self();
                }
                if key_with_mod.bare_key == BareKey::Char('l') {
                    if let Some(log_path) = &self.log_path {
                        open_file_near_plugin(FileToOpen::new(log_path), BTreeMap::new());
                    }
                    return true;
                }
                if key_with_mod.bare_key == BareKey::Enter {
                    self.launch();
                }
                true
            }
            Event::RunCommandResult(exit_code, stdout, stderr, context) => {
                let stage = context.get("stage").cloned().unwrap_or_default();
                match stage.as_str() {
                    "detect_root" => {
                        let root = if exit_code == Some(0) {
                            String::from_utf8(stdout.clone()).unwrap_or_default().trim().to_string()
                        } else {
                            String::from(".")
                        };
                        self.repo_root = Some(root.clone());
                        self.log_path = Some(format!("{}/af_zellij_plugin.log", root));

                        // now create worktree
                        let branch = self.form.branch.clone();
                        let worktree_path = self.form.worktree_path.clone();
                        let base = self.form.base.clone();
                        let mut ctx = BTreeMap::new();
                        ctx.insert("stage".into(), "create_worktree".into());
                        let log_target = self
                            .log_path
                            .clone()
                            .unwrap_or_else(|| format!("{}/af_zellij_plugin.log", root));
                        let cmd_str = format!(
                            "git worktree add -B {} '{}' {} 2>&1 | tee -a '{}'",
                            branch, worktree_path, base, log_target
                        );
                        run_command_with_env_variables_and_cwd(
                            &["bash", "-lc", &cmd_str],
                            BTreeMap::new(),
                            std::path::PathBuf::from(root),
                            ctx,
                        );
                        true
                    }
                    "create_worktree" => {
                        if exit_code == Some(0) {
                            // open panes now that worktree exists
                            open_terminal_near_plugin(&self.form.worktree_path);
                            let mut ctx = BTreeMap::new();
                            ctx.insert("purpose".into(), "launch_agent".into());
                            let program_cmd = if self.form.program == "custom" || self.form.program.is_empty() {
                                "bash".to_string()
                            } else {
                                self.form.program.clone()
                            };
                            let (cmd_path, cmd_args): (String, Vec<String>) = match shlex::split(&program_cmd) {
                                Some(mut parts) if !parts.is_empty() => {
                                    let path = parts.remove(0);
                                    (path, parts)
                                }
                                _ => ("bash".to_string(), vec![]),
                            };
                            let mut cmd = CommandToRun::new_with_args(cmd_path, cmd_args);
                            cmd.cwd = Some(std::path::PathBuf::from(self.form.worktree_path.clone()));
                            open_command_pane_near_plugin(cmd, ctx);
                            self.form.status = format!(
                                "Launched in {} on {}",
                                self.form.worktree_path, self.form.branch
                            );
                        } else {
                            let err = String::from_utf8(stderr.clone()).unwrap_or_default();
                            self.form.status = format!("Failed to create worktree: {}", err.trim());
                        }
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn render(&mut self, rows: usize, _cols: usize) {
        let title = "Agent Fleet";

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

        // create the worktree first, then launch upon success
        if self.form.create_worktree {
            let mut ctx = BTreeMap::new();
            ctx.insert("stage".into(), "detect_root".into());
            run_command_with_env_variables_and_cwd(
                &["git", "rev-parse", "--show-toplevel"],
                BTreeMap::new(),
                std::path::PathBuf::from("."),
                ctx,
            );
        } else {
            // no worktree creation, just launch in current folder
            open_terminal_near_plugin(".");
            let mut ctx = BTreeMap::new();
            ctx.insert("purpose".into(), "launch_agent".into());
            let (cmd_path, cmd_args): (String, Vec<String>) = match shlex::split(&program_cmd) {
                Some(mut parts) if !parts.is_empty() => {
                    let path = parts.remove(0);
                    (path, parts)
                }
                _ => ("bash".to_string(), vec![]),
            };
            let mut cmd = CommandToRun::new_with_args(cmd_path, cmd_args);
            cmd.cwd = Some(std::path::PathBuf::from("."));
            open_command_pane_near_plugin(cmd, ctx);
            self.form.status = "Launched agent without creating worktree".into();
        }
    }
}
