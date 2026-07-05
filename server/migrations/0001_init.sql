CREATE TABLE IF NOT EXISTS repositories (
    id BIGSERIAL PRIMARY KEY,
    provider TEXT NOT NULL,
    owner TEXT NOT NULL,
    name TEXT NOT NULL,
    default_branch TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (provider, owner, name)
);

CREATE TABLE IF NOT EXISTS scans (
    id BIGSERIAL PRIMARY KEY,
    repository_id BIGINT NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    commit_sha TEXT,
    score INTEGER NOT NULL,
    passed INTEGER NOT NULL,
    warnings INTEGER NOT NULL,
    failed INTEGER NOT NULL,
    report JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS scans_repository_created_idx
    ON scans (repository_id, created_at DESC);

CREATE TABLE IF NOT EXISTS findings (
    id BIGSERIAL PRIMARY KEY,
    scan_id BIGINT NOT NULL REFERENCES scans(id) ON DELETE CASCADE,
    rule_id TEXT NOT NULL,
    harness TEXT NOT NULL,
    severity TEXT NOT NULL,
    status TEXT NOT NULL,
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    evidence JSONB NOT NULL,
    source TEXT NOT NULL,
    remediation TEXT
);

CREATE INDEX IF NOT EXISTS findings_scan_idx ON findings (scan_id);
