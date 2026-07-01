-- Tabela principal de dispositivos (Caixas IoT)
-- Relação 1:1 com users (UNIQUE on user_id)
CREATE TABLE devices (
    id VARCHAR(50) PRIMARY KEY,
    user_id INTEGER UNIQUE REFERENCES users(id) ON DELETE SET NULL,
    api_key_hash TEXT NOT NULL UNIQUE,
    firmware_version VARCHAR(20),
    last_heartbeat_at TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Telemetria: eventos físicos reportados pela caixa
CREATE TABLE device_events (
    id SERIAL PRIMARY KEY,
    device_id VARCHAR(50) NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    event_type VARCHAR(50) NOT NULL,
    event_timestamp TIMESTAMPTZ NOT NULL,
    metadata JSONB,
    received_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Logs de debug do firmware
CREATE TABLE device_logs (
    id SERIAL PRIMARY KEY,
    device_id VARCHAR(50) NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    level VARCHAR(10) NOT NULL,
    component VARCHAR(100),
    message TEXT NOT NULL,
    event_timestamp TIMESTAMPTZ NOT NULL,
    received_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_device_events_device_id ON device_events(device_id);
CREATE INDEX idx_device_events_timestamp ON device_events(event_timestamp);
CREATE INDEX idx_device_logs_device_id ON device_logs(device_id);
