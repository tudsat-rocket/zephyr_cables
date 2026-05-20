PRAGMA foreign_keys = ON;

PRAGMA journal_mode = WAL;

-- kinds of wires used, (may add color in the future)
CREATE TABLE wireKind (
    awg INTEGER PRIMARY KEY,  -- e.g. 20, 24, 30 
    g_per_m REAL NOT NULL  -- grams per metre, for mass BOM
);

-- Vehicle sections / zones (optional)
CREATE TABLE section (id TEXT PRIMARY KEY, notes TEXT);

-- Device categories and kinds, e.g. IO-BOARD, FC, temperature-sensor
CREATE TABLE deviceKind (
    id TEXT PRIMARY KEY,
    category TEXT, -- optional, e.g: board, interconnect, node
    full_name TEXT,
    notes TEXT
);

-- Connector types (physical shell + pin count)
CREATE TABLE connectorKind (
    id TEXT PRIMARY KEY,  -- e.g. 'M4', 'M8', 'JST-GH'
    full_name TEXT NOT NULL,
    pin_count INTEGER NOT NULL CHECK(pin_count > 0),
    notes TEXT
);

-- Named connector positions defined per device kind
CREATE TABLE deviceConnector (
    device_kind TEXT NOT NULL,
    name TEXT NOT NULL,  -- e.g. 'VB0', 'PWR', 'HC-OUT1'
    connector_kind TEXT NOT NULL,
    notes TEXT,
    PRIMARY KEY (device_kind, name), -- TODO?
    FOREIGN KEY (device_kind) REFERENCES deviceKind(id) ON DELETE CASCADE,
    FOREIGN KEY (connector_kind) REFERENCES connectorKind(id) ON DELETE RESTRICT
);

-- Physical devices installed in the vehicle
CREATE TABLE device (
    name TEXT PRIMARY KEY,  -- e.g. 'FC0', 'IO1', 'SB4'
    device_kind TEXT NOT NULL,
    section TEXT,
    notes TEXT,
    FOREIGN KEY (device_kind) REFERENCES deviceKind(id) ON DELETE RESTRICT,
    FOREIGN KEY (section) REFERENCES section(id) ON DELETE
    SET
        NULL
);

-- Physical connector instances on each device
-- Represents a unique connector (attached to a unique device) within the vehicle
-- The Primary key (at the moments) shall be a concatenation of 
-- 'device.name' + '_' + 'deviceConnector.name'
-- e.g. 'IO1_PWR' for the power connector of IO-Board nr.1
CREATE TABLE connector (
    id TEXT PRIMARY KEY,
    device TEXT NOT NULL,
    device_connector TEXT NOT NULL,
    UNIQUE(device, device_connector),
    FOREIGN KEY (device) REFERENCES device(id) ON DELETE CASCADE,
    FOREIGN KEY (device_connector) REFERENCES deviceConnector(id) ON DELETE RESTRICT
);

-- Harness kind: a reusable template for a wiring assembly
-- Describes *what* the harness does, and what wires it contains
CREATE TABLE harnessKind (
    name TEXT PRIMARY KEY,
    description TEXT,
    notes TEXT
);

-- Physical harness instances — one row per installed harness
-- multiple harnesses can connect to a single connector
CREATE TABLE harness (
    name TEXT PRIMARY KEY,
    harness_kind TEXT NOT NULL, -- may be NULL in the future to allow anonymous harnesses
    from_connector TEXT NOT NULL,
    to_connector TEXT NOT NULL,
    length_mm REAL NOT NULL CHECK(length_mm > 0),
    notes TEXT,
    CHECK(from_connector != to_connector),
    FOREIGN KEY (harness_kind) REFERENCES harnessKind(id) ON DELETE RESTRICT,
    FOREIGN KEY (from_connector) REFERENCES connector(id) ON DELETE CASCADE,
    FOREIGN KEY (to_connector) REFERENCES connector(id) ON DELETE CASCADE
);

-- NOTE: The following tables are not very important at the moment

-- Electrical signals
CREATE TABLE signal (
    name TEXT PRIMARY KEY,
    -- signal_type TEXT NOT NULL DEFAULT 'unknown' CHECK(
    --     signal_type IN (
    --         'power',
    --         'ground',
    --         'CAN',
    --         'PWM',
    --         'analog',
    --         'digital',
    --         'unknown'
    --     )
    -- ),
    full_name TEXT,
    -- voltage REAL,  -- nominal voltage, NULL if not applicable
    notes TEXT
);

-- Wire definitions within a harness kind
-- e.g. VehilceBus (VB) contains 4 20awg power wires and 4 24awg data wires
-- NOTE: maybe in the future:
-- This may be where split-connector logic lives:
-- a half-MF4 harness simply may references only pin_numbers 1+2 (or 3+4)
CREATE TABLE harnessKindWire (
    harness_kind TEXT NOT NULL,
    wire_number INTEGER NOT NULL, -- local numbering within a harnessKind
    wire_kind TEXT NOT NULL,
    wire_color TEXT NOT NULL,
    signal TEXT,
    PRIMARY KEY (wire_number, harness_kind),
    FOREIGN KEY (harness_kind) REFERENCES harnessKind(id) ON DELETE CASCADE,
    FOREIGN KEY (wire_kind) REFERENCES wireKind(id) ON DELETE RESTRICT,
    FOREIGN KEY (signal) REFERENCES signal(id) ON DELETE
    SET
        NULL
    -- NOTE: unfinished ideas:
    -- connector position labels (matches deviceConnector.name on each end)
    -- from_position TEXT NOT NULL,  -- e.g. 'J2'
    -- from_pin INTEGER,
    -- to_position TEXT NOT NULL,
    -- to_pin INTEGER,
);

-- NOTE:
-- The following coloumns are not currently used because they introduce
-- complexity that is not required yet 


-- Pin definitions per connector instance
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



-- Resolved pin mapping for a harness instance
-- Links harnessKindWire entries to the concrete pin rows of this instance
--  Generated when instantiating a harness; allows deviation from the template
CREATE TABLE harnessWire (
    id TEXT PRIMARY KEY,
    harness TEXT NOT NULL,
    harness_kind_wire TEXT,  -- NULL if this wire deviates from template
    from_pin TEXT NOT NULL,
    to_pin TEXT NOT NULL,
    wire_kind TEXT NOT NULL,
    signal TEXT,  -- default NULL
    notes TEXT,
    FOREIGN KEY (harness) REFERENCES harness(id) ON DELETE CASCADE,
    FOREIGN KEY (harness_kind_wire) REFERENCES harnessKindWire(id) ON DELETE
    SET
        NULL,
        FOREIGN KEY (from_pin) REFERENCES pin(id) ON DELETE RESTRICT,
        FOREIGN KEY (to_pin) REFERENCES pin(id) ON DELETE RESTRICT,
        FOREIGN KEY (wire_kind) REFERENCES wireKind(id) ON DELETE RESTRICT,
        FOREIGN KEY (signal) REFERENCES signal(id) ON DELETE
    SET
        NULL
);
