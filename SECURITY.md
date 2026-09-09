# Security

Diple reads your repository and can query GitHub, GitLab, or Azure DevOps through their official
CLIs. It writes only to the Git index when you explicitly stage or unstage a file, and to its
private per-worktree base-pick ref. It never edits worktree files, commits, pushes, or writes to a
forge. Any unintended write is a security bug.

Report privately through
[GitHub security advisories](https://github.com/jhao0413/diple/security/advisories/new)
rather than a public issue. You will get a response within a few days.
