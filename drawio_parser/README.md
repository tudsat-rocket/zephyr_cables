# Introduction

This program parses our System Overview Diagram. It requires the diagram
elements to follow a very specific pattern to tell appart decorative shapes
from boards, connectors, wires and section markers. Using drawio key-value
property is required for adding meata information. Hover over an element to
see its properties. Press <CTRL-M> to edit properties.

## Usage

Export drawio diagram to uncompressed xml via: 

'Extras' -> 'Edit Diagramm...' -> 'Copy Diagram to Clipboard'

Save as diag.xml

'cargo run diag.xml'

## DrawIo Representation

Each object is a either an edge (wires, lines) or a vertex (rectangles).
Marked in xml with `edge="1"`, `vertex="1"`.

Example: 
```xml
<mxCell id="6" parent="1" style="text;html=1;align=left;verticalAlign=middle;whiteSpace=wrap;rounded=0;" value="VB" vertex="1">
  <mxGeometry height="15" width="60" x="637.5" y="410" as="geometry" />
</mxCell>
```
The mxGeometry tag contains position relative to the parent cell. In this case,
parent=1 means global coordinates.


A drawio **group** is an invisible non-connectable shape, that groups other
objects together and makes them move together. Internally the mxCell of every
group member links back to the group mxCell via `parent="[groupMxCellId]"`

# Part Types

## Board 
Marked by either property:`type: board` or by being the largest rectangle in a
group marked with `board_group: 1`.

## Connector
Smaller object in group marked `board_group: 1`.

## Wire  
Edge connection two of Board, Sensor, Interconnect.
Possible properties:
- `harness_kind`
- `length` in millimeters

## Interconnect
`type=interconnect`

## Sensor 
`type=sensor`


## Section
Shape marked with `section_marker: [section_name]`.


# Code

Instead of using a Database, the Data is collected in a number of HashMaps that
map the uniquie drawio id to a corresponding struct.

You can do you custom data processing at the end of the main function.

