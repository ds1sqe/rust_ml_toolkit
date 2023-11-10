use eframe::{
    egui,
    egui::{Frame, Response, Sense, Ui},
    emath::remap,
    epaint::Vec2,
    epaint::{self, Color32, Stroke},
};
use egui_plot::{self, Line, Plot, PlotPoint, PlotPoints, PlotResponse, PlotUi, Points};

use crate::{adapter::nodes::Nodes, core::nn::nn::NN};

//use std::f64::consts::TAU;

fn circle(pos: PlotPoint) -> Points {
    let radius = 10.0;
    let points = Points::new(vec![[pos.x, pos.y]])
        .filled(true)
        .shape(egui_plot::MarkerShape::Circle)
        .radius(radius)
        .color(Color32::from_rgb(100, 200, 100))
        .name("circle");

    points
}

fn get_pos(start: usize, end: usize, distance: f64, idx: usize) {}

fn visualize(plot_ui: &mut PlotUi, nn: Nodes) {
    let width = nn.layers.len();
    let height = nn.layers.iter().max().unwrap();

    // let hot;
    // let cold;
    // let hi;
    // let low;

    for (level, connections) in nn.connections.iter().enumerate() {
        for (node_idx, node_connections) in connections.iter().enumerate() {
            for con in node_connections.iter() {
                let pos_src_x = con.src_level as f64 * 10.0;
                let pos_src_y = con.src as f64 * 2.0;
                let pos_dst_x = con.dst_level as f64 * 10.0;
                let pos_dst_y = con.dst as f64 * 2.0;
                plot_ui.line(
                    Line::new(PlotPoints::from_explicit_callback(
                        move |x| {
                            (pos_dst_y - pos_src_y) * (x - pos_src_x) / (pos_dst_x - pos_src_x)
                                + pos_src_y
                        },
                        (pos_src_x + 0.1)..(pos_dst_x - 0.1),
                        256,
                    ))
                    .name(format!(
                        "{}:{}->{}:{}\nweight:{}",
                        con.src_level, con.src, con.dst_level, con.dst, con.weight
                    )),
                )
            }
        }
    }

    for (level, nodes) in nn.nodes.iter().enumerate() {
        let pos_x = level as f64 * 10.0;
        for (nidx, node) in nodes.iter().enumerate() {
            let pos_y = nidx as f64 * 2.0;
            plot_ui.points(circle(PlotPoint::new(pos_x, pos_y)))
        }
    }
}

pub fn draw(ui: &mut Ui) -> Response {
    let plot = Plot::new("network").height(600.0).data_aspect(1.0);

    let layers = [4, 8, 8, 8, 4];
    let orgin = NN::new(&layers);
    let nodes = Nodes::from(&orgin);

    let PlotResponse {
        response,
        inner: (screen_pos, pointer_coordinate, pointer_coordinate_drag_delta, bounds, hovered, lll),
        ..
    } = plot.show(ui, |plot_ui| {
        (
            plot_ui.screen_from_plot(PlotPoint::new(0.0, 0.0)),
            plot_ui.pointer_coordinate(),
            plot_ui.pointer_coordinate_drag_delta(),
            plot_ui.plot_bounds(),
            plot_ui.response().hovered(),
            visualize(plot_ui, nodes),
            //plot_ui.points(circle(PlotPoint::new(0.0, 0.0))),
        )
    });

    ui.label(format!(
        "plot bounds: min: {:.02?}, max: {:.02?}",
        bounds.min(),
        bounds.max()
    ));
    ui.label(format!(
        "origin in screen coordinates: x: {:.02}, y: {:.02}",
        screen_pos.x, screen_pos.y
    ));
    ui.label(format!("plot hovered: {hovered}"));
    let coordinate_text = if let Some(coordinate) = pointer_coordinate {
        format!("x: {:.02}, y: {:.02}", coordinate.x, coordinate.y)
    } else {
        "None".to_owned()
    };
    ui.label(format!("pointer coordinate: {coordinate_text}"));
    let coordinate_text = format!(
        "x: {:.02}, y: {:.02}",
        pointer_coordinate_drag_delta.x, pointer_coordinate_drag_delta.y
    );
    ui.label(format!("pointer coordinate drag delta: {coordinate_text}"));

    response
}
