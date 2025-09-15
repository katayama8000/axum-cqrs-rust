CREATE TABLE IF NOT EXISTS circle_events (
    id CHAR(36) NOT NULL PRIMARY KEY,
    circle_id CHAR(36) NOT NULL,
    version INT NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    payload JSON NOT NULL,
    occurred_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE IF NOT EXISTS circle_snapshots (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    circle_id CHAR(36) NOT NULL,
    version INT NOT NULL,
    state JSON NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_circle_version (circle_id, version DESC)
);