INSERT INTO
    wireKind(awg, g_per_m)
VALUES
    (20, 7.0), (24, 4.0), (30, 1.0);

INSERT INTO
    section(id)
VALUES
    ('combustion'), ('pressurization'), ('payvionics'), ('parabrakes'), ('main');

INSERT INTO
    deviceKind(id, full_name, category)
VALUES
    ('FC', 'Flight Computer', 'board'),
    ('CATS', 'Cats Vega Flight Computer', 'board'),
    ('IO', 'IO Board', 'board'),
    ('EB', 'Engine Board', 'board'),
    ('PB', 'Power Board', 'board'),
    ('SB', 'Sensor Board', 'board'),
    ('AB', 'Arming Board', 'board'),
    ('TL', 'Tank level Indicator', 'board'),
    ('XB', 'Blind Mating Interconnect 10 pin', 'interconnect'),
    ('XG', 'Ground Support Interconnect 4 pin, magnetic', 'interconnect'),
    ('WAGO', 'Wago connector', 'interconnect'),
    ('P_S', 'Pressure Sensor', 'node'),
    ('T_S', 'Temperature Sensor', 'node'),
    ('SOL_V', 'Pressure Sensor', 'node'),
    ('SER_V', 'Pressure Sensor', 'node'),
    ('HEAT_T', 'Heat pad temperature sensor', 'node'),
    ('HEAT_P', 'Heat pad power connections', 'node');


INSERT INTO
    connectorKind(id, full_name, pin_count)
VALUES
    ('M4', 'Microfit 2x2', 4),
    ('M6', 'Microfit 2x3', 6),
    ('M8', 'Microfit 2x4', 8),
    ('M10B', 'Microfit 2x10 blind mating', 10),
    ('M12', 'Microfit 2x6', 12),
    ('Mag4', 'Symetric Magnetic 4 pin connecor', 4),
    ('GH', 'JST-GH 4 pin', 4);

INSERT INTO
    boardConnector(device_kind, name, connector_kind)
VALUES
    ('FC', 'VB0', 'M8'),
    ('FC', 'VB1', 'M8'),
    ('FC', 'RECOV', 'M12'),
    ('FC', 'COM_A', 'JST-GH'),
    ('FC', 'COM_UART', 'JST-GH'),
    -- CATS: TODO
    ('IO', 'VB0', 'M8'),
    ('IO', 'VB1', 'M8'),
    ('IO', 'PWR', 'M4'),
    ('IO', 'O12', 'M4'),
    ('IO', 'O34', 'M4'),
    ('IO', 'COM1', 'JST-GH'),
    ('IO', 'COM2', 'JST-GH'),
    ('IO', 'COM3', 'JST-GH'),
    ('IO', 'COM4', 'JST-GH'),
    ('IO', 'COM5', 'JST-GH'),
    ('IO', 'COM6', 'JST-GH'),

    ('EB', 'VB0', 'M8'),
    ('EB', 'VB1', 'M8'),
    ('EB', 'SERV1', 'M4'),
    ('EB', 'SERV2', 'M4'),
    ('EB', 'SERV3', 'M4'),
    ('EB', 'SERV4', 'M4'),
    ('EB', 'IGN', 'M4'),  -- TODO: M4 ?
    ('EB', 'GSE', 'M4'),
    ('EB', 'CHG', 'M4'),
    ('EB', 'PWR1', 'M4'),
    ('EB', 'PWR2', 'M4'),
    ('EB', 'SENS1', 'JST-GH'),
    ('EB', 'SENS2', 'JST-GH'),

    ('PB', 'VB0', 'M8'),
    ('PB', 'VB1', 'M8'),
    ('PB', 'O1', 'M4'),
    ('PB', 'O2', 'M4'),
    ('PB', 'PRE', 'M4'),
    ('PB', 'POST', 'M4'),

    ('SB', 'S', 'JST-GH'),
    ('SB', 'D0', 'JST-GH'),
    ('SB', 'D1', 'JST-GH'),  

    ('AB', 'PRE', 'M4'),
    ('AB', 'POST', 'M4'),
    ('AB', 'CTRL', 'M6'),
    -- NOTE: TL
    ('XB', 'F', 'M10B'),
    ('XB', 'M', 'M10B'),

    ('XG', 'F', 'Mag4'),
    ('XG', 'M', 'Mag4'); -- NOTE: not finished

INSERT INTO
    device(name, device_kind, section, notes)
VALUES
    ('FC0', 'FC', 'payvionics', NULL),
    ('CATS', 'CATS', 'payvionics', NULL),

    ('IO0', 'IO', 'main', 'controls cameras and sensors'),
    ('IO1', 'IO', 'payvionics', 'may not exist! controls payloads, if needed'),
    ('IO2', 'IO', 'pressurization', 'controls valves in press and sensors in comb'),

    ('EB', 'EB', 'pressurization', 'controls sensors, valves and ignition'),

    ('PB0', 'PB', 'payvionics', 'supplies power to FC / recovery'),
    ('PB1', 'PB', 'payvionics', 'supplies power to CATS / recovery'),
    ('PB2', 'PB', 'payvionics', 'supplies power valves'),

    ('AB0', 'AB', 'payvionics', 'arms FC'),
    ('AB1', 'AB', 'payvionics', 'arms CATS'),
    ('AB2', 'AB', 'payvionics', 'arms vales'),

    -- sensor boards + sensors

    ('SB0', 'SB', 'main', NULL),
    ('p-sens-nosecone', 'P_S', 'main', 'Pressure sensor nosecone'),

    ('SB1', 'SB', 'pressurization', NULL),
    ('p-sens-n2-tank', 'P_S', 'pressurization', 'Pressure sensor N2 tank'),

    ('SB2', 'SB', 'pressurization', NULL),
    ('p-sens-reg2', 'P_S', 'pressurization', 'Pressure sensor regulator 2, higher than 1'),

    ('SB3', 'SB', 'pressurization', NULL),
    ('p-sens-reg1', 'P_S', 'pressurization', 'Pressure sensor regulator 1, position lower than 2'),

    ('SB4', 'SB', 'pressurization', NULL),
    ('p-sens-ox-upper', 'P_S', 'pressurization', 'Pressure sensor upper oxidizer tank'),

    ('SB5', 'SB', 'combustion', NULL),
    ('p-sens-ox-lower', 'P_S', 'combustion', 'Pressure sensor lower oxidizer tank'),

    ('SB6', 'SB', 'combustion', NULL),
    ('p-sens-comb-ch', 'P_S', 'combustion', 'Pressure sensor combustion chamber'),

    ('SB7', 'SB', 'combustion', NULL),
    ('t-sens-ox-valve', 'P_S', 'combustion', 'Pressure sensor combustion chamber'),

    -- interconnects
    ('TL', 'TL', 'pressurization', 'measures temperature of tank'),
    ('X31', 'XB', 'payvionics', 'connects payvionics and pressurization'),
    ('X32', 'XB', 'payvionics', 'connects payvionics and pressurization'),
    ('X33', 'XB', 'payvionics', 'connects payvionics and pressurization'),
    ('X41', 'XB', 'main', 'connects main and payvionics'),
    ('X42', 'XB', 'main', 'connects main and payvionics'),
    ('X43', 'XB', 'main', 'connects main and payvionics'),
    -- vales
    ('OX-Down-Valve', 'SOL_V', 'pressurization', NULL),
    ('OX-Up-Valve', 'SOL_V', 'pressurization', NULL),
    ('Vent-Valve', 'SER_V', 'pressurization', NULL),
    ('Press-Valve', 'SER_V', 'pressurization', NULL),

    ('Main-Valve', 'SER_V', 'combustion', NULL),
    ('OX-Fill-Dump-Valve', 'SER_V', 'combustion', NULL),
    -- sensors 


    ('t-sens-ox', 'T_S', 'combustion', 'Temperature sensor lower oxidizer tank, connected with cots board -> i2c')

    --TODO: initors, dc-dc converter, endswitches, heat pad
;


INSERT INTO connector (id, device, board_connector) 
SELECT 
    d.name || '_' || bc.name,
    d.name,
    bc.name
FROM device d
JOIN boardConnector bc ON bc.device_kind = d.device_kind;

-- INSERT OR IGNORE INTO connector (id, device, board_connector)
-- SELECT
--     d.name || '_' || bc.name,
--     d.name,
--     bc.id
-- FROM device d
-- JOIN boardConnector bc ON bc.device_kind = d.device_kind;


