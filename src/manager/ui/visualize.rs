use std::thread;

use eframe::{
    egui,
    egui::{Frame, Response, Sense, Ui},
    emath::remap,
    epaint::Vec2,
    epaint::{self, Color32, Stroke},
};
use egui_plot::{
    self, Bar, BarChart, Line, Plot, PlotPoint, PlotPoints, PlotResponse, PlotUi,
    Points,
};

use crate::{
    adapter::{
        context::{Context, State},
        nodes::{Node, Nodes, Nodetype},
    },
    core::nn::nn::NN,
};

use super::gradient::{Color, Gradient};

//use std::f64::consts::TAU;

fn create_node(pos: PlotPoint, node: &Node) -> Points {
    let label = match node {
        Node {
            nodetype: Nodetype::Input,
            level,
            bias,
            value,
        } => format!("Type:Input\nValue:{}", value),
        Node {
            nodetype: Nodetype::Middle,
            level,
            bias,
            value,
        } => format!(
            "Type:Middle\nLevel:{}\nBias:{}\nValue:{}",
            level, bias, value
        ),
        Node {
            nodetype: Nodetype::Output,
            level,
            bias,
            value,
        } => format!("Type:Output\nBias:{}\nValue:{}", bias, value),
    };

    let radius = 10.0;

    let grad = Gradient::default(10.0, -10.0);
    let Color { r, g, b } = grad.get_color(node.bias as f32);

    let points = Points::new(vec![[pos.x, pos.y]])
        .filled(true)
        .shape(egui_plot::MarkerShape::Circle)
        .radius(radius)
        .color(Color32::from_rgb(r, g, b))
        .name(label);

    points
}

fn get_pos(start: usize, end: usize, distance: f64, idx: usize) {}

fn visualize(plot_ui: &mut PlotUi, nn: Nodes) {
    // let width = nn.layers.len();
    // let height = nn.layers.iter().max().unwrap();

    // let hot;
    // let cold;
    // let hi;
    // let low;

    for (level, connections) in nn.connections.iter().enumerate() {
        for (node_idx, node_connections) in connections.iter().enumerate() {
            for con in node_connections.iter() {
                let len_src = nn.layers[level];
                let len_dst = nn.layers[level + 1];

                let pos_src_x = con.src_level as f64 * 10.0;
                let pos_src_y = con.src as f64 * 2.0 - (1.0 * (len_src - 1) as f64);
                let pos_dst_x = con.dst_level as f64 * 10.0;
                let pos_dst_y = con.dst as f64 * 2.0 - (1.0 * (len_dst - 1) as f64);

                let grad = Gradient::default(10.0, -10.0);
                let Color { r, g, b } = grad.get_color(con.weight as f32);

                plot_ui.line(
                    Line::new(PlotPoints::from_explicit_callback(
                        move |x| {
                            (pos_dst_y - pos_src_y) * (x - pos_src_x)
                                / (pos_dst_x - pos_src_x)
                                + pos_src_y
                        },
                        (pos_src_x + 0.2)..(pos_dst_x - 0.2),
                        256,
                    ))
                    .color(Color32::from_rgb(r, g, b))
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
        let count = nodes.len();
        for (nidx, node) in nodes.iter().enumerate() {
            let pos_y = nidx as f64 * 2.0 - (2.0 / 2.0 * (count - 1) as f64);
            plot_ui.points(create_node(PlotPoint::new(pos_x, pos_y), node))
        }
    }
}

pub fn draw_node(ui: &mut Ui, context: &mut Context) -> Option<Response> {
    match context.state {
        State::Empty => return None,
        _ => (),
    }

    let plot = Plot::new("network").height(600.0).data_aspect(1.0);

    let nodes = context.nodes.as_ref().unwrap().clone();

    let PlotResponse {
        response,
        inner:
            (
                screen_pos,
                pointer_coordinate,
                pointer_coordinate_drag_delta,
                bounds,
                hovered,
                lll,
            ),
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

    Some(response)
}

fn visualize_costs(plot_ui: &mut PlotUi, costs: Vec<f64>) {
    // let width = nn.layers.len();
    // let height = nn.layers.iter().max().unwrap();

    // let hot;
    // let cold;
    // let hi;
    // let low;

    let mut chart = BarChart::new(
        costs
            .iter()
            .enumerate()
            .map(|(cycle, cost)| {
                Bar::new(cycle as f64, cost.log10())
                    .width(1.0)
                    .name(format!("cost: {}\ncycle: {}", cost, cycle))
            })
            .collect(),
    );
    plot_ui.bar_chart(chart);
}

pub fn draw_cost(ui: &mut Ui, context: &mut Context) -> Option<Response> {
    match context.state {
        State::Empty => return None,
        _ => (),
    }

    let plot = Plot::new("costs").height(300.0).clamp_grid(true);
    //.data_aspect(100.0);

    let costs = context.costs.clone();

    let PlotResponse {
        response,
        inner:
            (
                screen_pos,
                pointer_coordinate,
                pointer_coordinate_drag_delta,
                bounds,
                hovered,
                lll,
            ),
        ..
    } = plot.show(ui, |plot_ui| {
        (
            plot_ui.screen_from_plot(PlotPoint::new(0.0, 0.0)),
            plot_ui.pointer_coordinate(),
            plot_ui.pointer_coordinate_drag_delta(),
            plot_ui.plot_bounds(),
            plot_ui.response().hovered(),
            visualize_costs(plot_ui, costs),
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

    Some(response)
}
