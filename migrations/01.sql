PRAGMA foreign_keys = ON;

PRAGMA journal_mode = WAL;

-- 1. Wire specifications (reusable, referenced by harness kind pins)
CREATE TABLE wireKind (
    awg INTEGER PRIMARY KEY,  -- e.g. 18, 20, 22
    g_per_m REAL NOT NULL  -- grams per metre, for mass BOM
);

-- 2. Vehicle sections / zones (optional but useful for large harnesses)
CREATE TABLE section (id TEXT PRIMARY KEY, notes TEXT);

-- 3. Device categories and kinds
CREATE TABLE deviceKind (
    id TEXT PRIMARY KEY,
    category TEXT,
    full_name TEXT,
    -- NOT NULL CHECK(
    --     category IN (
    --         'board',
    --         'sensor',
    --         'actuator',
    --         'interconnect',
    --     )
    -- ),
    notes TEXT
);

-- 4. Connector types (physical shell + pin count)
CREATE TABLE connectorKind (
    id TEXT PRIMARY KEY,  -- e.g. 'MF4', 'MF8', 'JSTGH4'
    full_name TEXT NOT NULL,
    pin_count INTEGER NOT NULL CHECK(pin_count > 0),
    -- gender TEXT NOT NULL DEFAULT 'housing' CHECK(
    --     gender IN ('housing', 'male', 'female', 'either')
    -- ),
    notes TEXT
);

-- 5. Named connector positions defined per device kind
--    e.g. MCU board has J1 (MF8), J2 (MF4), J3 (MF4)
CREATE TABLE boardConnector (
    -- id TEXT PRIMARY KEY,
    device_kind TEXT NOT NULL,
    name TEXT NOT NULL,  -- e.g. 'J1', 'POWER_IN', 'CAN_OUT'
    connector_kind TEXT NOT NULL,
    notes TEXT,
    PRIMARY KEY (device_kind, name), -- TODO?
    FOREIGN KEY (device_kind) REFERENCES deviceKind(id) ON DELETE CASCADE,
    FOREIGN KEY (connector_kind) REFERENCES connectorKind(id) ON DELETE RESTRICT
);

-- 6. Physical devices installed in the vehicle
CREATE TABLE device (
    name TEXT PRIMARY KEY,  -- e.g. 'MCU_1', 'MotorFL'
    device_kind TEXT NOT NULL,
    section TEXT,  -- which zone this device lives in
    notes TEXT,
    FOREIGN KEY (device_kind) REFERENCES deviceKind(id) ON DELETE RESTRICT,
    FOREIGN KEY (section) REFERENCES section(id) ON DELETE
    SET
        NULL
);

-- 7. Physical connector instances on each device
--    Derived connectorKind via boardConnector — not stored redundantly
CREATE TABLE connector (
    id TEXT PRIMARY KEY,
    device TEXT NOT NULL,
    board_connector TEXT NOT NULL,
    UNIQUE(device, board_connector),
    FOREIGN KEY (device) REFERENCES device(id) ON DELETE CASCADE,
    FOREIGN KEY (board_connector) REFERENCES boardConnector(id) ON DELETE RESTRICT
);

-- 8. Electrical signals
CREATE TABLE signal (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    signal_type TEXT NOT NULL DEFAULT 'unknown' CHECK(
        signal_type IN (
            'power',
            'ground',
            'CAN',
            'PWM',
            'analog',
            'digital',
            'unknown'
        )
    ),
    voltage REAL,  -- nominal voltage, NULL if not applicable
    notes TEXT
);

-- 9. Pin definitions per connector instance
CREATE TABLE pin (
    id TEXT PRIMARY KEY,
    connector TEXT NOT NULL,
    pin_number INTEGER NOT NULL CHECK(pin_number > 0),
    signal TEXT,
    notes TEXT,
    UNIQUE(connector, pin_number),
    FOREIGN KEY (connector) REFERENCES connector(id) ON DELETE CASCADE,
    FOREIGN KEY (signal) REFERENCES signal(id) ON DELETE
    SET
        NULL
);

-- 10. Harness kind: a reusable template for a wiring assembly
--     Describes *what* the harness does, not its physical length
CREATE TABLE harnessKind (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,  -- e.g. 'MCU_to_PowerDist_CAN'
    description TEXT,
    notes TEXT
);

-- 11. Pin-level wire definitions within a harness kind
--     This is where split-connector logic lives:
--     a half-MF4 harness simply references only pin_numbers 1+2 (or 3+4)
CREATE TABLE harnessKindWire (
    id TEXT PRIMARY KEY,
    harness_kind TEXT NOT NULL,
    wire_kind TEXT NOT NULL,
    -- connector position labels (matches boardConnector.name on each end)
    from_position TEXT NOT NULL,  -- e.g. 'J2'
    from_pin INTEGER NOT NULL,
    to_position TEXT NOT NULL,
    to_pin INTEGER NOT NULL,
    signal TEXT,
    FOREIGN KEY (harness_kind) REFERENCES harnessKind(id) ON DELETE CASCADE,
    FOREIGN KEY (wire_kind) REFERENCES wireKind(id) ON DELETE RESTRICT,
    FOREIGN KEY (signal) REFERENCES signal(id) ON DELETE
    SET
        NULL
);

-- 12. Physical harness instances — one row per installed harness
CREATE TABLE harness (
    id TEXT PRIMARY KEY,
    harness_kind TEXT NOT NULL,
    from_connector TEXT NOT NULL,
    to_connector TEXT NOT NULL,
    length_mm REAL NOT NULL CHECK(length_mm > 0),
    notes TEXT,
    CHECK(from_connector != to_connector),
    FOREIGN KEY (harness_kind) REFERENCES harnessKind(id) ON DELETE RESTRICT,
    FOREIGN KEY (from_connector) REFERENCES connector(id) ON DELETE CASCADE,
    FOREIGN KEY (to_connector) REFERENCES connector(id) ON DELETE CASCADE
);

-- 13. Resolved pin mapping for a harness instance
--     Links harnessKindWire entries to the concrete pin rows of this instance
--     Generated when instantiating a harness; allows deviation from the template
CREATE TABLE harnessWire (
    id TEXT PRIMARY KEY,
    harness TEXT NOT NULL,
    harness_kind_wire TEXT,  -- NULL if this wire deviates from template
    from_pin TEXT NOT NULL,
    to_pin TEXT NOT NULL,
    wire_spec TEXT NOT NULL,
    signal TEXT,  -- default NULL
    notes TEXT,
    FOREIGN KEY (harness) REFERENCES harness(id) ON DELETE CASCADE,
    FOREIGN KEY (harness_kind_wire) REFERENCES harnessKindWire(id) ON DELETE
    SET
        NULL,
        FOREIGN KEY (from_pin) REFERENCES pin(id) ON DELETE RESTRICT,
        FOREIGN KEY (to_pin) REFERENCES pin(id) ON DELETE RESTRICT,
        FOREIGN KEY (wire_spec) REFERENCES wireSpec(id) ON DELETE RESTRICT,
        FOREIGN KEY (signal) REFERENCES signal(id) ON DELETE
    SET
        NULL
);
