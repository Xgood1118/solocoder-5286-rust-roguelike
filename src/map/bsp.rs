use rand::Rng;
use super::map::{Tile, TileType, Rect};

pub struct BspNode {
    pub rect: Rect,
    pub left: Option<Box<BspNode>>,
    pub right: Option<Box<BspNode>>,
}

impl BspNode {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            left: None,
            right: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

pub struct BspMapResult {
    pub tiles: Vec<Tile>,
    pub rooms: Vec<Rect>,
    pub corridors: Vec<Vec<(i32, i32)>>,
}

pub fn generate_bsp_map<R: Rng>(
    width: i32,
    height: i32,
    depth: i32,
    min_room_size: i32,
    rng: &mut R,
) -> BspMapResult {
    let mut tiles = vec![Tile::wall(); (width * height) as usize];

    let root_rect = Rect::new(1, 1, width - 2, height - 2);
    let mut root = BspNode::new(root_rect);

    split_node(&mut root, depth, min_room_size, rng);

    let mut rooms = Vec::new();
    create_rooms(&root, min_room_size, rng, &mut rooms);

    for room in &rooms {
        carve_room(&mut tiles, room, width);
    }

    let corridors = connect_rooms(&mut tiles, &rooms, width, rng);

    if let Some(last_room) = rooms.last() {
        let center = last_room.center();
        let idx = (center.y * width + center.x) as usize;
        tiles[idx] = Tile::new(TileType::DownStairs);
    }

    BspMapResult { tiles, rooms, corridors }
}

fn split_node<R: Rng>(
    node: &mut BspNode,
    depth: i32,
    min_size: i32,
    rng: &mut R,
) {
    if depth <= 0 {
        return;
    }

    let rect = node.rect;
    let can_split_h = rect.height >= min_size * 2;
    let can_split_v = rect.width >= min_size * 2;

    if !can_split_h && !can_split_v {
        return;
    }

    let split_horizontally = if can_split_h && can_split_v {
        rng.gen_bool(0.5)
    } else {
        can_split_h
    };

    if split_horizontally {
        let split_y = rng.gen_range(min_size..=rect.height - min_size);

        let left_rect = Rect::new(rect.x, rect.y, rect.width, split_y);
        let right_rect = Rect::new(rect.x, rect.y + split_y, rect.width, rect.height - split_y);

        node.left = Some(Box::new(BspNode::new(left_rect)));
        node.right = Some(Box::new(BspNode::new(right_rect)));
    } else {
        let split_x = rng.gen_range(min_size..=rect.width - min_size);

        let left_rect = Rect::new(rect.x, rect.y, split_x, rect.height);
        let right_rect = Rect::new(rect.x + split_x, rect.y, rect.width - split_x, rect.height);

        node.left = Some(Box::new(BspNode::new(left_rect)));
        node.right = Some(Box::new(BspNode::new(right_rect)));
    }

    if let Some(ref mut left) = node.left {
        split_node(left, depth - 1, min_size, rng);
    }
    if let Some(ref mut right) = node.right {
        split_node(right, depth - 1, min_size, rng);
    }
}

fn create_rooms<R: Rng>(
    node: &BspNode,
    min_size: i32,
    rng: &mut R,
    rooms: &mut Vec<Rect>,
) {
    if node.is_leaf() {
        let rect = node.rect;
        let room_w = rng.gen_range(min_size.max(4)..=rect.width);
        let room_h = rng.gen_range(min_size.max(4)..=rect.height);
        let room_x = rect.x + rng.gen_range(0..=rect.width - room_w);
        let room_y = rect.y + rng.gen_range(0..=rect.height - room_h);

        rooms.push(Rect::new(room_x, room_y, room_w, room_h));
    } else {
        if let Some(ref left) = node.left {
            create_rooms(left, min_size, rng, rooms);
        }
        if let Some(ref right) = node.right {
            create_rooms(right, min_size, rng, rooms);
        }
    }
}

fn carve_room(tiles: &mut [Tile], room: &Rect, map_width: i32) {
    for y in room.y..room.y + room.height {
        for x in room.x..room.x + room.width {
            let idx = (y * map_width + x) as usize;
            if idx < tiles.len() {
                tiles[idx] = Tile::floor();
            }
        }
    }
}

fn connect_rooms<R: Rng>(
    tiles: &mut [Tile],
    rooms: &[Rect],
    map_width: i32,
    rng: &mut R,
) -> Vec<Vec<(i32, i32)>> {
    let mut corridors = Vec::new();

    if rooms.len() < 2 {
        return corridors;
    }

    let mut connected = vec![false; rooms.len()];
    connected[0] = true;
    let mut connected_count = 1;

    let mut edges: Vec<(usize, usize)> = Vec::new();
    for i in 0..rooms.len() {
        for j in i + 1..rooms.len() {
            edges.push((i, j));
        }
    }

    edges.sort_by(|a, b| {
        let dist_a = rooms[a.0].center().distance(&rooms[a.1].center());
        let dist_b = rooms[b.0].center().distance(&rooms[b.1].center());
        dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut mst_edges = Vec::new();
    for (a, b) in &edges {
        if connected[*a] != connected[*b] {
            mst_edges.push((*a, *b));
            if connected[*a] {
                connected[*b] = true;
            } else {
                connected[*a] = true;
            }
            connected_count += 1;
            if connected_count == rooms.len() {
                break;
            }
        }
    }

    if mst_edges.len() < rooms.len() - 1 {
        for i in 1..rooms.len() {
            mst_edges.push((i - 1, i));
        }
    }

    let extra_loops = (rooms.len() as f32 * 0.3) as usize;
    let mut shuffled_edges: Vec<(usize, usize)> = edges
        .iter()
        .filter(|(a, b)| !mst_edges.contains(&(*a, *b)) && !mst_edges.contains(&(*b, *a)))
        .cloned()
        .collect();
    use rand::seq::SliceRandom;
    shuffled_edges.shuffle(rng);

    for i in 0..extra_loops.min(shuffled_edges.len()) {
        mst_edges.push(shuffled_edges[i]);
    }

    for (a, b) in mst_edges {
        let start = rooms[a].center();
        let end = rooms[b].center();

        let corridor = if rng.gen_bool(0.5) {
            carve_h_then_v(tiles, start.x, start.y, end.x, end.y, map_width)
        } else {
            carve_v_then_h(tiles, start.x, start.y, end.x, end.y, map_width)
        };

        corridors.push(corridor);
    }

    corridors
}

fn carve_h_then_v(
    tiles: &mut [Tile],
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    map_width: i32,
) -> Vec<(i32, i32)> {
    let mut corridor = Vec::new();

    let start_x = x1.min(x2);
    let end_x = x1.max(x2);
    for x in start_x..=end_x {
        let idx = (y1 * map_width + x) as usize;
        if idx < tiles.len() {
            tiles[idx] = Tile::floor();
            corridor.push((x, y1));
        }
    }

    let start_y = y1.min(y2);
    let end_y = y1.max(y2);
    for y in start_y..=end_y {
        let idx = (y * map_width + x2) as usize;
        if idx < tiles.len() {
            tiles[idx] = Tile::floor();
            corridor.push((x2, y));
        }
    }

    corridor
}

fn carve_v_then_h(
    tiles: &mut [Tile],
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    map_width: i32,
) -> Vec<(i32, i32)> {
    let mut corridor = Vec::new();

    let start_y = y1.min(y2);
    let end_y = y1.max(y2);
    for y in start_y..=end_y {
        let idx = (y * map_width + x1) as usize;
        if idx < tiles.len() {
            tiles[idx] = Tile::floor();
            corridor.push((x1, y));
        }
    }

    let start_x = x1.min(x2);
    let end_x = x1.max(x2);
    for x in start_x..=end_x {
        let idx = (y2 * map_width + x) as usize;
        if idx < tiles.len() {
            tiles[idx] = Tile::floor();
            corridor.push((x, y2));
        }
    }

    corridor
}
