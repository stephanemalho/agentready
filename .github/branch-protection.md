# Recommended Branch Protection

Configure this manually in GitHub repository settings for `main`.

Recommended rules:

- Require a pull request before merging.
- Require approvals.
- Require status checks to pass before merging.
- Require `Agent control gates`.
- Require branches to be up to date before merging.
- Restrict who can push to matching branches.
- Do not allow force pushes on `main`.
- Do not allow deletions of `main`.

Human policy:

- Agents may open or update PRs.
- The human maintainer compares against `main`.
- The human maintainer performs the final merge.

