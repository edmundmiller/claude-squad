# Agent Fleet [![CI](https://github.com/smtg-ai/agent-fleet/actions/workflows/build.yml/badge.svg)](https://github.com/smtg-ai/agent-fleet/actions/workflows/build.yml) [![GitHub Release](https://img.shields.io/github/v/release/smtg-ai/agent-fleet)](https://github.com/smtg-ai/agent-fleet/releases/latest)

[Agent Fleet](https://smtg-ai.github.io/agent-fleet/) is a terminal app that orchestrates multiple [Claude Code](https://github.com/anthropics/claude-code), [Codex](https://github.com/openai/codex), [Gemini](https://github.com/google-gemini/gemini-cli) (and other local agents including [Aider](https://github.com/Aider-AI/aider)) in separate workspaces, allowing you to work on multiple tasks simultaneously.


![Agent Fleet Screenshot](assets/screenshot.png)

### Highlights
- Complete tasks in the background (including yolo / auto-accept mode!)
- Manage instances and tasks in one terminal window
- Review changes before applying them, checkout changes before pushing them
- Each task gets its own isolated git workspace, so no conflicts

<br />

https://github.com/user-attachments/assets/aef18253-e58f-4525-9032-f5a3d66c975a

<br />

### Installation

Both Homebrew and manual installation will install Agent Fleet as `af` on your system.

#### Homebrew

```bash
brew install agent-fleet
ln -s "$(brew --prefix)/bin/agent-fleet" "$(brew --prefix)/bin/cs"
```

#### Manual

Agent Fleet can also be installed by running the following command:

```bash
curl -fsSL https://raw.githubusercontent.com/smtg-ai/agent-fleet/main/install.sh | bash
```

This puts the `cs` binary in `~/.local/bin`.

To use a custom name for the binary:

```bash
curl -fsSL https://raw.githubusercontent.com/smtg-ai/agent-fleet/main/install.sh | bash -s -- --name <your-binary-name>
```

### Prerequisites

- [zellij](https://zellij.dev/documentation/installation.html)
- [gh](https://cli.github.com/)
- Rust toolchain + `wasm32-wasip1` target

### Zellij plugin (experimental)

A Rust/WASM Zellij tile plugin lives at `plugins/zellij/agent-fleet-tile`. It can create a git worktree from the current repo and launch an Agent Fleet-compatible program in that worktree.

Build it:

```bash
cd plugins/zellij/agent-fleet-tile
./scripts/build.sh
```

Run it via layout:

```bash
zellij --layout layouts/agent-fleet.kdl
```

### Usage

```
Usage:
  af [flags]
  af [command]

Available Commands:
  completion  Generate the autocompletion script for the specified shell
  debug       Print debug information like config paths
  help        Help about any command
  reset       Reset all stored instances
  version     Print the version number of agent-fleet

Flags:
  -y, --autoyes          [experimental] If enabled, all instances will automatically accept prompts for claude code & aider
  -h, --help             help for agent-fleet
  -p, --program string   Program to run in new instances (e.g. 'aider --model ollama_chat/gemma3:1b')
```

Run the application with `af` (or `cs` if you symlinked):

```bash
af
```
NOTE: The default program is `claude` and we recommend using the latest version.

<br />

<b>Using Agent Fleet with other AI assistants:</b>
- For [Codex](https://github.com/openai/codex): Set your API key with `export OPENAI_API_KEY=<your_key>`
- Launch with specific assistants:
   - Codex: `af -p "codex"`
   - Aider: `af -p "aider ..."`
   - Gemini: `af -p "gemini"`
- Make this the default, by modifying the config file (locate with `af debug`)
### Agents

See [AGENTS.md](AGENTS.md) for supported agents, tips, and workflows.

<br />

#### Menu
The menu at the bottom of the screen shows available commands: 

##### Instance/Session Management
- `n` - Create a new session
- `N` - Create a new session with a prompt
- `D` - Kill (delete) the selected session
- `↑/j`, `↓/k` - Navigate between sessions

##### Actions
- `↵/o` - Attach to the selected session to reprompt
- `ctrl-q` - Detach from session
- `s` - Commit and push branch to github
- `c` - Checkout. Commits changes and pauses the session
- `r` - Resume a paused session
- `?` - Show help menu

##### Navigation
- `tab` - Switch between preview tab and diff tab
- `q` - Quit the application
- `shift-↓/↑` - scroll in diff view

### FAQs

#### Failed to start new session

If you get an error like `failed to start new session: timed out waiting for zellij session`, update the
underlying program (ex. `claude`) to the latest version.

### How It Works

1. **zellij** to create isolated terminal sessions for each agent
2. **git worktrees** to isolate codebases so each session works on its own branch
3. A simple TUI interface for easy navigation and management

### License

[AGPL-3.0](LICENSE.md)

### Star History

[![Star History Chart](https://api.star-history.com/svg?repos=smtg-ai/agent-fleet&type=Date)](https://www.star-history.com/#smtg-ai/agent-fleet&Date)
