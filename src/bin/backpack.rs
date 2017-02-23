extern crate hashcode2017 as code;

use std::{env, mem};
use code::*;

fn main() {
    let path = env::args().skip(1).next().unwrap();
    let input = Input::from_file(&path);

    let mut closest_connections = Vec::with_capacity(input.endpoints.len());
    for endp in &input.endpoints {
        let closest_opt = endp.connections.iter().min_by(|ca, cb| {
            ca.latency.cmp(&cb.latency)
        });
        closest_connections.push(closest_opt);
    }

    let mut request_packs: Vec<Vec<Item>> = (0..input.cache_count)
        .map(|_| Vec::new()).collect();
    for (rid, request) in input.requests.iter().enumerate() {
        if let Some(connection) = closest_connections[request.endpoint_id as usize] {
            let center_latency = input.endpoints[request.endpoint_id as usize].data_center_latency;
            let latency_gain = center_latency - connection.latency;
            let video_size = input.video_sizes[request.video_id as usize];
            request_packs[connection.cache_id as usize].push(Item {
                request_id: rid as u32,
                latency_gain: latency_gain,
                video_size: video_size,
                ratio: video_size as f32 / latency_gain as f32,
            });
        }
    }

    let mut caches = Vec::with_capacity(input.cache_count as usize);
    let mut id = 0;
    for pack in &mut request_packs {
        let result = solve_backpack(pack, &input);
        let mut video_ids = Vec::new();
        for (selected, item) in result.best_items.into_iter().zip(pack.iter()) {
            if selected {
                video_ids.push(input.requests[item.request_id as usize].video_id);
            }
        }
        caches.push(Cache {
            id: id,
            video_ids: video_ids,
        });
        id += 1;
    }

    Output { caches: caches }.to_file("results.out");
}

#[derive(Debug)]
struct Result {
    best_items: Vec<bool>, // FIXME
    latency_gain: u32,
    video_size: u32
}

struct Item {
    request_id: u32,
    latency_gain: u16,
    video_size: u16,
    ratio: f32,
}

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Infeasible,
    Feasible(u32),
}

impl Cell {
    fn add(self, a: u32) -> Cell {
        match self {
            Cell::Infeasible => Cell::Infeasible,
            Cell::Feasible(b) => Cell::Feasible(a + b)
        }
    }

    fn min(self, other: Cell) -> Cell {
        use std::cmp;
        match (self, other) {
            (Cell::Infeasible, _) => other,
            (_, Cell::Infeasible) => self,
            (Cell::Feasible(a), Cell::Feasible(b)) =>
                Cell::Feasible(cmp::min(a, b))
        }
    }
}

// TODO: approximate

fn solve_backpack(items: &mut [Item], input: &Input) -> Result {
    items.sort_by(|a, b| a.ratio.partial_cmp(&b.ratio).unwrap());
    let v_max = items.len() *
        items.iter().map(|i| i.latency_gain).max().unwrap_or(0) as usize;

    match items.split_first() {
        None => Result {
            best_items: Vec::new(),
            latency_gain: 0,
            video_size: 0,
        },
        Some((i, is)) => {
            let mut rows = Vec::with_capacity(items.len());
            let mut prev_row = first_row(i, v_max);
            for i in is {
                let next_row = next_row(i, &prev_row, v_max);
                rows.push(mem::replace(&mut prev_row, next_row));
            }
            rows.push(prev_row);
            extract_result(rows, items, input)
        }
    }
}

fn first_row(item: &Item, v_max: usize) -> Vec<Cell> {
    let mut row = Vec::with_capacity(v_max);

    row.push(Cell::Feasible(0));
    for v in 1..(v_max + 1) {
        row.push(if v == item.latency_gain as usize {
            Cell::Feasible(item.video_size as u32)
        } else {
            Cell::Infeasible
        });
    }

    row
}

fn next_row(item: &Item, prev_row: &[Cell], v_max: usize) -> Vec<Cell> {
    let u = item.latency_gain as usize;
    let mut row = Vec::with_capacity(v_max);

    for v in 0..(v_max + 1) {
        row.push(if u <= v {
            let with = prev_row[v - u].add(item.video_size as u32);
            prev_row[v].min(with)
        } else {
            prev_row[v]
        });
    }

    row
}

fn extract_result(mut rows: Vec<Vec<Cell>>, items: &[Item], input: &Input) -> Result {
    let last_row = rows.pop().unwrap();
    let mut optimum_v = 0;
    let mut optimum_w = 0;
    for (v, cell) in last_row.into_iter().enumerate() {
        if let Cell::Feasible(w) = cell {
            if w <= input.cache_capacity {
                optimum_v = v as u32;
                optimum_w = w;
            }
        }
    }

    Result {
        best_items: traceback(optimum_v, Cell::Feasible(optimum_w), rows, items),
        latency_gain: optimum_v,
        video_size: optimum_w,
    }
}

fn traceback(mut v: u32, mut w: Cell, rows: Vec<Vec<Cell>>, items: &[Item])
             -> Vec<bool>
{
    let mut item_stack = Vec::with_capacity(items.len());
    for (row, item) in rows.into_iter().zip(items.iter()).rev() {
        let cell = row[v as usize];
        if w == cell {
            item_stack.push(false);
        } else {
            item_stack.push(true);
            v -= item.latency_gain as u32;
            w = row[v as usize];
        }
    }

    item_stack.reverse();
    item_stack
}
