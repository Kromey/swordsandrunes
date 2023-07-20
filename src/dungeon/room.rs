use bevy::prelude::*;
use delaunator::{next_halfedge, triangulate, Point};
use itertools::Itertools;
use petgraph::{algo::min_spanning_tree, data::FromElements, prelude::*};

use crate::dungeon::TilePos;

type UnGraph = Graph<RectangularRoom, u32, Undirected, u16>;
type RoomIndex = NodeIndex<u16>;

#[derive(Debug, Default, Clone)]
pub struct RoomGraph {
    rooms: UnGraph,
}

impl RoomGraph {
    pub fn from_rooms(rooms: &[RectangularRoom]) -> Self {
        let mut graph = UnGraph::default();
        for &room in rooms {
            graph.add_node(room);
        }

        Self { rooms: graph }
    }

    pub fn triangulate(&mut self) {
        let centers: Vec<_> = self
            .rooms
            .node_weights()
            .map(|room| {
                let Vec2 { x, y } = room.center().as_vec();
                Point {
                    x: x as f64,
                    y: y as f64,
                }
            })
            .collect();
        let delaunay = triangulate(&centers);

        // This is adapted from `forEachTriangleEdge` function at <https://mapbox.github.io/delaunator/>
        // Kudos to "1L-1UX" (illiux#5291) on Roguelikes Discord - Thank you!
        for e in 0..delaunay.triangles.len() {
            let o = delaunay.halfedges[e];
            if e > o || o == delaunator::EMPTY {
                let p = delaunay.triangles[e];
                let q = delaunay.triangles[next_halfedge(e)];

                let a = RoomIndex::new(p);
                let b = RoomIndex::new(q);
                let distance = self
                    .rooms
                    .node_weight(a)
                    .unwrap()
                    .center()
                    .distance(self.rooms.node_weight(b).unwrap().center());

                self.rooms.add_edge(a, b, distance);
            }
        }
    }

    pub fn to_min_spanning_tree(&mut self) {
        self.rooms = UnGraph::from_elements(min_spanning_tree(&self.rooms));
    }

    pub fn rooms(&self) -> impl Iterator<Item = &RectangularRoom> {
        self.rooms.node_weights()
    }

    pub fn edges(&self) -> impl Iterator<Item = (&RectangularRoom, &RectangularRoom)> + '_ {
        self.rooms.node_indices().flat_map(move |node| {
            self.rooms.neighbors(node).filter_map(move |neighbor| {
                if neighbor > node {
                    let room = self.rooms.node_weight(node).unwrap();
                    let other = self.rooms.node_weight(neighbor).unwrap();
                    Some((room, other))
                } else {
                    None
                }
            })
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RectangularRoom {
    start: TilePos,
    end: TilePos,
}

impl RectangularRoom {
    pub fn new(from: TilePos, width: u32, height: u32) -> Self {
        Self {
            start: from,
            end: TilePos {
                x: from.x + width - 1, // End is inclusive, but adding width gets an exclusive point
                y: from.y + height - 1, // Ditto
            },
        }
    }

    pub fn center(&self) -> TilePos {
        (self.start + self.end) / 2
    }

    pub fn intersects(&self, other: Self) -> bool {
        self.start.x <= other.end.x
            && self.end.x >= other.start.x
            && self.start.y <= other.end.y
            && self.end.y >= other.start.y
    }

    pub fn iter(&self) -> impl Iterator<Item = TilePos> {
        // Iterate across the _floor_ tiles within this room
        ((self.start.x + 1)..self.end.x)
            .cartesian_product((self.start.y + 1)..self.end.y)
            .map(TilePos::from)
    }
}
