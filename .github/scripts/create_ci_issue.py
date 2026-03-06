"""Create or update a GitHub issue when a CI job fails.

Usage:
    python .github/scripts/create_ci_issue.py --label LABEL --title TITLE

Reads workflow context from standard GitHub Actions environment variables:
  GH_TOKEN, GITHUB_REPOSITORY, GITHUB_SERVER_URL, GITHUB_RUN_ID,
  GITHUB_REF_NAME, GITHUB_SHA, GITHUB_EVENT_NAME
"""

import argparse
import os
import subprocess
import sys


# --- helpers -------------------------------------------------------------------


def run_gh(*args: str) -> str:
    """Run a gh CLI command and return stdout (stripped). Raises on non-zero exit."""
    result = subprocess.run(
        ["gh", *args],
        capture_output=True,
        text=True,
        check=True,
    )
    return result.stdout.strip()


def ensure_label(repo: str, label: str, description: str) -> None:
    """Create the label if it does not already exist. Logs to stderr on failure."""
    result = subprocess.run(
        [
            "gh", "label", "create", label,
            "--repo", repo,
            "--description", description,
            "--color", "d73a4a",
        ],
        capture_output=True,
        text=True,
    )
    if result.returncode != 0 and "already exists" not in result.stderr:
        print(f"Warning: failed to create label '{label}': {result.stderr.strip()}", file=sys.stderr)


def find_existing_open_issue(repo: str, label: str) -> str:
    """Return the issue number of the first open issue with *label*, or ''."""
    return run_gh(
        "issue", "list",
        "--repo", repo,
        "--label", label,
        "--state", "open",
        "--limit", "1",
        "--json", "number",
        "--jq", ".[0].number // empty",
    )


def build_body(title: str, run_url: str, branch: str, sha: str, event: str) -> str:
    """Return the Markdown body for an issue or comment."""
    return (
        f"## {title}\n\n"
        f"**Workflow run:** {run_url}\n"
        f"**Branch:** {branch}\n"
        f"**Commit:** {sha}\n"
        f"**Triggered by:** {event}\n\n"
        "Please investigate the workflow logs for details."
    )


def add_comment(repo: str, issue_number: str, body: str) -> None:
    """Add a comment to an existing issue."""
    subprocess.run(
        ["gh", "issue", "comment", issue_number, "--repo", repo, "--body", body],
        check=True,
    )


def create_issue(repo: str, label: str, title: str, body: str) -> None:
    """Create a new issue with the given label and title."""
    subprocess.run(
        [
            "gh", "issue", "create",
            "--repo", repo,
            "--label", label,
            "--title", title,
            "--body", body,
        ],
        check=True,
    )


# --- main ----------------------------------------------------------------------


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--label", required=True, help="GitHub issue label")
    parser.add_argument("--title", required=True, help="Issue title (also used as body heading)")
    return parser.parse_args()


def build_run_url(server_url: str, repository: str, run_id: str) -> str:
    return f"{server_url}/{repository}/actions/runs/{run_id}"


def load_env() -> dict:
    """Load and validate required GitHub Actions environment variables."""
    required = [
        "GITHUB_REPOSITORY",
        "GITHUB_RUN_ID",
        "GITHUB_REF_NAME",
        "GITHUB_SHA",
        "GITHUB_EVENT_NAME",
    ]
    missing = [k for k in required if not os.environ.get(k)]
    if missing:
        print(f"Error: missing required environment variables: {', '.join(missing)}", file=sys.stderr)
        sys.exit(1)
    return {
        "repo": os.environ["GITHUB_REPOSITORY"],
        "server_url": os.environ.get("GITHUB_SERVER_URL", "https://github.com"),
        "run_id": os.environ["GITHUB_RUN_ID"],
        "branch": os.environ["GITHUB_REF_NAME"],
        "sha": os.environ["GITHUB_SHA"],
        "event": os.environ["GITHUB_EVENT_NAME"],
    }


def main() -> None:
    args = parse_args()
    env = load_env()

    run_url = build_run_url(env["server_url"], env["repo"], env["run_id"])

    ensure_label(env["repo"], args.label, f"Auto-created on {args.title}")
    existing = find_existing_open_issue(env["repo"], args.label)

    body = build_body(
        title=args.title + (" again" if existing else ""),
        run_url=run_url,
        branch=env["branch"],
        sha=env["sha"],
        event=env["event"],
    )

    if existing:
        add_comment(env["repo"], existing, body)
    else:
        create_issue(env["repo"], args.label, args.title, body)


if __name__ == "__main__":
    main()
