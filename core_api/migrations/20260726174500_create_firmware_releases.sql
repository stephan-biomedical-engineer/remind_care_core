-- Releases de firmware da caixa IoT (OTA)
-- Cada linha representa um build publicado. O dispositivo sempre baixa o de maior build_number.
CREATE TABLE firmware_releases (
    id SERIAL PRIMARY KEY,
    version VARCHAR(20) NOT NULL,
    build_number INTEGER NOT NULL UNIQUE,
    filename VARCHAR(255) NOT NULL,
    sha256 VARCHAR(64) NOT NULL,
    release_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_firmware_releases_build_number ON firmware_releases(build_number DESC);
