# Agents

## Overview
Agent Fleet lets you orchestrate multiple AI coding agents in isolated worktrees and zellij sessions so you can parallelize tasks and context.

- Each session runs in its own git worktree and zellij session
- You can pause/resume, preview output, and push branches for review
- Default agent is `claude`; you can run others per-session

## Supported agents
- Claude Code (`claude`)
- Aider (`aider` with flags)
- Codex (`codex`)
- Gemini (`gemini`)

## Set the default program
- Run `af debug` to see the config path (e.g., `~/.agent-fleet/config.json`)
- Edit `default_program` to your preferred agent command

Example config snippet:
```json
{
  "default_program": "aider --model ollama_chat/gemma3:1b",
  "auto_yes": false,
  "daemon_poll_interval": 1000,
  "branch_prefix": "yourname/"
}
```

## Launch a session with a specific program
Use `-p` (or `--program`) to override for a single run:
```bash
af -p "aider --model ollama_chat/gemma3:1b"
af -p "gemini"
```

## Tips for prompts and workflows
- Keep prompts short and actionable; iterate inside the session
- Use pause/checkout to hand off work: commit locally and switch in your editor
- Use push to open a PR for review
- Run multiple sessions for different sub-tasks; preview and diff help you track progress

## Add a new agent command
- Most agents work as a shell command; set as `default_program` or pass via `-p`
- If the agent requires extra setup, create a wrapper script on your PATH and reference it

## Known prompts
- Claude Code: trust prompt auto-handled; we send Enter on first run
- Aider/Gemini: initial docs links prompts auto-handled with D+Enter
