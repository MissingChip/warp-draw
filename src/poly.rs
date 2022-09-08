
use futures_util::{stream::SplitSink, SinkExt};
use warp::ws::{Message, WebSocket};

use crate::poly::CommandData::*;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32,
}

pub enum CommandData {
    Points(Vec<Point>),
    Move((Point, f32)),
    RGBA([u8;4]),
    Weight(f32),
    NA,
}

pub struct Command {
    op: u32,
    id: u32,
    data: CommandData,
}

impl Command {
    fn poly<T: IntoIterator<Item = Point>>(id: u32, points: T) -> Command
    {
        let op = 0;
        let mut pointsx = Vec::new();
        for point in points.into_iter() {
            pointsx.push(point);
        }
        Command{op, id, data: CommandData::Points(pointsx)}
    }
    fn poly_from<T: IntoIterator<Item = [f32; 2]>>(id: u32, points: T) -> Command
    {
        let mut pointsx = Vec::new();
        for pointarr in points.into_iter() {
            let point = Point{x: pointarr[0], y: pointarr[1]};
            pointsx.push(point);
        }
        Self::poly(id, pointsx)
    }
    fn draw(id: u32) -> Command
    {
        let op = 1;
        Command{op, id, data: CommandData::Move((Point{x: 0.0, y: 0.0}, 0.0))}
    }
    fn draw_at(id: u32, point: Point) -> Command
    {
        let op = 1;
        Command{op, id, data: CommandData::Move((point, 0.0))}
    }
    fn draw_at_rot(id: u32, point: Point, rot: f32) -> Command
    {
        let op = 1;
        Command{op, id, data: CommandData::Move((point, rot))}
    }
    fn fill(id: u32, r: u8, g: u8, b: u8, a: u8) -> Command
    {
        let op = 2;
        Command{op, id, data: CommandData::RGBA([r, g, b, a])}
    }
    fn stroke(id: u32, r: u8, g: u8, b: u8, a: u8) -> Command
    {
        let op = 3;
        Command{op, id, data: CommandData::RGBA([r, g, b, a])}
    }
    fn weight(id: u32, weight: f32) -> Command
    {
        let op = 4;
        Command{op, id, data: CommandData::Weight(weight)}
    }
    fn clear() -> Command
    {
        let op = 8;
        let id = 0;
        Command{op, id, data: CommandData::NA}
    }
    fn stub(&self, data_size: usize) -> Vec<u8>{
        let mut bytes = Vec::<u8>::with_capacity(4*2 + data_size);
        self.write_stub(&mut bytes);
        bytes
    }
    fn write_stub(&self, bytes: &mut Vec<u8>) {
        Self::write_u32(bytes, self.op);
        Self::write_u32(bytes, self.id);
    }
    fn write_u32(bytes: &mut Vec<u8>, x: u32) {
        let be_bytes = x.to_be_bytes();
        bytes.extend(&be_bytes);
    }
    fn write_f32(bytes: &mut Vec<u8>, x: f32) {
        let be_bytes = x.to_be_bytes();
        bytes.extend(&be_bytes);
    }
    fn write_point(bytes: &mut Vec<u8>, point: &Point) {
        Self::write_f32(bytes, point.x);
        Self::write_f32(bytes, point.y);
    }
    fn write_points(bytes: &mut Vec<u8>, points: &Vec<Point>) {
        for point in points {
            Self::write_point(bytes, &point);
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes;
        match &self.data {
            Points(points) => {
                bytes = self.stub(4*2*points.len());
                Self::write_points(&mut bytes, points);
            }
            Move(translation) => {
                bytes = self.stub(4*3);
                Self::write_point(&mut bytes, &translation.0);
                Self::write_f32(&mut bytes, translation.1);
            }
            RGBA(rgba) => {
                bytes = self.stub(4);
                bytes.extend(rgba);
            }
            Weight(weight) => {
                bytes = self.stub(4);
                Self::write_f32(&mut bytes, *weight);
            }
            NA => {
                bytes = self.stub(0);
            }
        }
        bytes
    }
}

pub async fn send_test_poly(tx: &mut SplitSink<WebSocket, Message>) {
    let points = [
        [-0.5f32, -0.5],
        [0.0, 0.5],
        [0.5, -0.5],
    ];
    let cmd = Command::poly_from(0, points);
    tx.send(Message::binary(cmd.as_bytes())).await.unwrap();
    tx.send(Message::binary(Command::fill(0, 20, 20, 30, 255).as_bytes())).await.unwrap();
    tx.send(Message::binary(Command::stroke(0, 255, 0, 0, 255).as_bytes())).await.unwrap();
    tx.send(Message::binary(Command::weight(0, 10.0).as_bytes())).await.unwrap();
    let cmd = Command::draw_at(0, Point{x: 0.4, y: 0.1});
    tx.send(Message::binary(cmd.as_bytes())).await.unwrap();
    tx.send(Message::binary(Command::clear().as_bytes())).await.unwrap();
    let cmd = Command::draw_at_rot(0, Point{x: 0.4, y: 0.4}, 0.15);
    tx.send(Message::binary(cmd.as_bytes())).await.unwrap();
    let points = [
        [-0.5f32, 0.5],
        [0.0, -0.5],
        [0.5, 0.5],
    ];
    let cmd = Command::poly_from(1, points);
    tx.send(Message::binary(cmd.as_bytes())).await.unwrap();
    tx.send(Message::binary(Command::fill(1, 0, 0, 0, 128).as_bytes())).await.unwrap();
    let cmd = Command::draw(1);
    tx.send(Message::binary(cmd.as_bytes())).await.unwrap();
}