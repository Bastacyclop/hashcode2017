use std::{io, fs};

struct Header {
    video_count: u16,
    endpoint_count: u16,
    request_count: u32,
    cache_count: u16,
    cache_capacity: u32
}

impl Header {
    fn from_line(line: &str) -> Self {
        let mut numbers = line.split_whitespace();
        Header {
            video_count: numbers.next().unwrap().parse().unwrap(),
            endpoint_count: numbers.next().unwrap().parse().unwrap(),
            request_count: numbers.next().unwrap().parse().unwrap(),
            cache_count: numbers.next().unwrap().parse().unwrap(),
            cache_capacity: numbers.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Input {
    pub cache_count: u16,
    pub cache_capacity: u32,
    pub video_sizes: Vec<u16>,
    pub endpoints: Vec<Endpoint>,
    pub requests: Vec<Request>,
}

impl Input {
    pub fn from_file(path: &str) -> Self {
        use std::io::BufRead;
        let file = fs::File::open(path).unwrap();
        let mut line = String::new();
        let mut reader = io::BufReader::new(file);

        reader.read_line(&mut line).unwrap();
        let header = Header::from_line(&line);
        line.clear();

        let mut video_sizes = Vec::with_capacity(header.video_count as usize);
        reader.read_line(&mut line).unwrap();
        {
            let mut split = line.split_whitespace();
            for _ in 0..header.video_count {
                video_sizes.push(split.next().unwrap().parse::<u16>().unwrap());
            }
        }
        line.clear();

        let mut endpoints = Vec::with_capacity(header.endpoint_count as usize);
        for _ in 0..header.endpoint_count {
            reader.read_line(&mut line).unwrap();
            let data_center_latency: u16;
            let cache_count: usize;
            {
                let mut split = line.split_whitespace();
                data_center_latency = split.next().unwrap().parse().unwrap();
                cache_count = split.next().unwrap().parse().unwrap();
            }
            line.clear();

            let mut connections = Vec::with_capacity(cache_count);
            for _ in 0..cache_count {
                reader.read_line(&mut line).unwrap();
                connections.push(Connection::from_line(&line));
                line.clear();
            }

            endpoints.push(Endpoint {
                data_center_latency: data_center_latency,
                connections: connections
            });
        }

        let mut requests = Vec::with_capacity(header.request_count as usize);
        for _ in 0..header.request_count {
            reader.read_line(&mut line).unwrap();
            requests.push(Request::from_line(&line));
            line.clear();
        }

        Input {
            cache_count: header.cache_count,
            cache_capacity: header.cache_capacity,
            video_sizes: video_sizes,
            endpoints: endpoints,
            requests: requests,
        }
    }
}

#[derive(Debug)]
pub struct Endpoint {
    pub data_center_latency: u16,
    pub connections: Vec<Connection>,
}

#[derive(Debug)]
pub struct Connection {
    pub cache_id: u16,
    pub latency: u16,
}

impl Connection {
    fn from_line(line: &str) -> Self {
        let mut numbers = line.split_whitespace();
        Connection {
            cache_id: numbers.next().unwrap().parse().unwrap(),
            latency: numbers.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub video_id: u16,
    pub endpoint_id: u16,
    pub request_count: u16,
}

impl Request {
    fn from_line(line: &str) -> Self {
        let mut split = line.split_whitespace();
        Request {
            video_id: split.next().unwrap().parse().unwrap(),
            endpoint_id: split.next().unwrap().parse().unwrap(),
            request_count: split.next().unwrap().parse().unwrap(),
        }
    }
}

