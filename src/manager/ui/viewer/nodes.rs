use eframe::{
    egui::{Response, Ui},
    epaint::Color32,
};
use egui_plot::{self, AxisHints, Line, Plot, PlotPoint, PlotPoints, PlotResponse, PlotUi, Points};

use crate::adapter::{
    context::{Context, State},
    nodes::{Node, Nodes, Nodetype},
};

use super::super::gradient::{Color, Gradient};

pub struct NodesView {
    pub is_open: bool,
}

impl NodesView {
    pub fn view(&mut self, ctx: &eframe::egui::Context, ui: &mut Ui, context: &mut Context) {
        eframe::egui::Window::new("NodesViewer")
            .open(&mut self.is_open)
            .resizable(true)
            .show(ctx, |ui| draw_node(ui, context));
    }
}

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

fn visualize(plot_ui: &mut PlotUi, nn: Nodes) {
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
                            (pos_dst_y - pos_src_y) * (x - pos_src_x) / (pos_dst_x - pos_src_x)
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

    let x_axes = vec![AxisHints::default().label("Level")];
    let y_axes = vec![AxisHints::default().label("Nodes")];

    let plot = Plot::new("network")
        .height(600.0)
        .data_aspect(1.0)
        .custom_x_axes(x_axes)
        .custom_y_axes(y_axes);

    let nodes = context.nodes.as_ref().unwrap().clone();

    let PlotResponse {
        response,
        inner: ((),),
        ..
    } = plot.show(ui, |plot_ui| (visualize(plot_ui, nodes),));

    Some(response)
}
