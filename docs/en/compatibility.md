# Agent compatibility matrix

All host integration tests are UNVERIFIED; verified agent version: none. Official documentation confirms a discovery convention, not a live installation test. Windows itself requires CI and actual Windows 11 acceptance.

| Agent | User skill root | Method | Invocation | Reload | Status / version / limitation |
|---|---|---|---|---|---|
| FreePi | ~/.free-pi/agent/skills | -Agent FreePi, confirm actual host path | /skill:northstar-re if supported | /reload if supported | UNVERIFIED / none / user-observed convention; upstream not independently established |
| Codex | ~/.agents/skills | -Agent Codex | $northstar-re or /skills | auto discovery; restart if absent | UNVERIFIED / none / confirm Windows host support and installed version |
| Pi Coding Agent | ~/.pi/agent/skills | -Agent Pi | /skill:northstar-re | /reload; confirm version | UNVERIFIED / none / recursive discovery; installer internalizes specialists |
| Claude Code | ~/.claude/skills | -Agent Claude | /northstar-re | restart if absent | UNVERIFIED / none / do not assume FreePi slash syntax |
| Other | Explicit -SkillRoot | inspect host docs first | host-specific | host-specific | UNVERIFIED / none / no settings mutation |

Sources checked 2026-11-25: [OpenAI skills](https://learn.chatgpt.com/docs/build-skills), [OpenAI path reference](https://developers.openai.com/api/reference/cli), [Pi upstream](https://github.com/badlogic/pi-mono/blob/main/packages/coding-agent/docs/skills.md), [Claude Code skills](https://code.claude.com/docs/en/skills). Source evidence retained as URLs; no remote content is treated as installation authorization.

Only one source master is registered. Specialists remain internal GUIDE.md references in installed copies. No host config is changed, no provider/account credentials are read.
