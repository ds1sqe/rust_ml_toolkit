use eframe::egui::{Response, Ui};
use egui_plot::{self, Bar, BarChart, Plot, PlotPoint, PlotResponse, PlotUi};

use crate::{
    adapter::context::{Context, State},
    core::nn::cost::CostInfo,
};

pub struct CostsView {
    pub is_open: bool,
}

impl CostsView {
    pub fn view(&mut self, ctx: &eframe::egui::Context, ui: &mut Ui, context: &mut Context) {
        eframe::egui::Window::new("CostsViewer")
            .open(&mut self.is_open)
            .resizable(true)
            .show(ctx, |ui| draw_cost(ui, context));
    }
}

fn visualize_costs(plot_ui: &mut PlotUi, costs: Vec<CostInfo>) {
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
            .map(|(cycle, cost_info)| {
                Bar::new(cycle as f64, cost_info.avg.log10())
                    .width(1.0)
                    .name(format!("cost: {}\ncycle: {}", cost_info.avg, cycle))
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

    let costs = context.cost_info.clone();

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
