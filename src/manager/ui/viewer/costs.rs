use eframe::egui::{Response, Ui};
use egui_plot::{self, AxisHints, Bar, BarChart, Plot, PlotResponse, PlotUi};

use crate::{
    adapter::context::{Context, State},
    core::nn::cost::CostInfo,
};

#[derive(PartialEq)]
pub enum CostsMenu {
    Chart,
    Detail,
}

pub struct CostsWindow {
    pub is_open: bool,
    pub menu: CostsMenu,
    pub focused_cycle: Option<usize>,
}

impl CostsWindow {
    pub fn new() -> Self {
        Self {
            is_open: false,
            menu: CostsMenu::Chart,
            focused_cycle: None,
        }
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn view(&mut self, ctx: &eframe::egui::Context, ui: &mut Ui, context: &mut Context) {
        eframe::egui::Window::new("CostsViewer")
            .open(&mut self.is_open)
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.menu, CostsMenu::Chart, "Chart");
                    ui.selectable_value(&mut self.menu, CostsMenu::Detail, "Detail");
                });
                ui.separator();

                match self.menu {
                    CostsMenu::Chart => {
                        draw_cost(ui, context, &mut self.focused_cycle);
                    }
                    CostsMenu::Detail => cost_detail(ui, context, &self.focused_cycle),
                }
            });
    }
}

fn visualize_costs(plot_ui: &mut PlotUi, costs: &Vec<CostInfo>) {
    let chart = BarChart::new(
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
pub fn draw_cost(
    ui: &mut Ui,
    context: &mut Context,
    focus: &mut Option<usize>,
) -> Option<Response> {
    match context.state {
        State::Empty => {
            ui.label("Is Blank");
            return None;
        }
        _ => (),
    }

    let x_axes = vec![AxisHints::default().label(format!(
        "Cycle / {}",
        context.session.as_ref().unwrap().option.cycle
    ))];
    let y_axes = vec![AxisHints::default().label("Log10 ( Cost )")];
    let plot = Plot::new("costs")
        .height(300.0)
        .clamp_grid(true)
        .custom_x_axes(x_axes)
        .custom_y_axes(y_axes);

    let costs = context.cost_info.clone();

    let PlotResponse {
        response,
        inner: (pointer_coordinate, hovered, clicked, ()),
        ..
    } = plot.show(ui, |plot_ui| {
        (
            plot_ui.pointer_coordinate(),
            plot_ui.response().hovered(),
            plot_ui.response().clicked(),
            visualize_costs(plot_ui, &costs),
        )
    });

    if focus.is_some() {
        // if it's in a bound of costs
        let focused = focus.unwrap();

        if 0 < focused && focused <= costs.len().saturating_sub(1) {
            let cost = &costs[focused];

            ui.separator();
            ui.heading("Focused");

            ui.label(format!("Cycle: {}", focused));
            let max = cost.max();
            ui.label(format!("Max index: {}\nMax cost: {}", max.idx, max.val));
            let min = cost.min();
            ui.label(format!("Min index: {}\nMin cost: {}", min.idx, min.val));
            let avg = cost.avg;
            ui.label(format!("AVG: {}", avg));

            if ui.button("Remove Focus").clicked() {
                *focus = None;
            }
            ui.separator();
        }
    } else {
        ui.label("Click plot to focus on");
    }

    if hovered && pointer_coordinate.is_some() {
        // currently hoverd cycle
        let cycle = pointer_coordinate.unwrap().x.ceil() as usize;

        // if it's in a bound of costs
        if 0 < cycle && cycle <= costs.len().saturating_sub(1) {
            let cost = &costs[cycle];

            ui.separator();
            ui.heading("Hoverd");

            ui.label(format!("Cycle: {}", cycle));
            let max = cost.max();
            ui.label(format!("Max index: {}\nMax cost: {}", max.idx, max.val));
            let min = cost.min();
            ui.label(format!("Min index: {}\nMin cost: {}", min.idx, min.val));
            let avg = cost.avg;
            ui.label(format!("AVG: {}", avg));

            ui.separator();

            if clicked {
                *focus = Some(cycle);
            }
        }
    }

    Some(response)
}

pub fn cost_detail(ui: &mut Ui, context: &mut Context, cycle: &Option<usize>) {
    match context.state {
        State::Empty => {
            ui.label("Is Blank");
            return;
        }
        _ => (),
    }

    if cycle.is_some() {
        let cycle = cycle.unwrap();
        let cost = &context.cost_info[cycle];

        let x_axes = vec![AxisHints::default().label("Index")];
        let y_axes = vec![AxisHints::default().label("Log10 ( Cost )")];
        let plot = Plot::new("costs")
            .height(300.0)
            .clamp_grid(true)
            .custom_x_axes(x_axes)
            .auto_bounds_x()
            .auto_bounds_y()
            .custom_y_axes(y_axes);

        plot.show(ui, |plot_ui| {
            let chart = BarChart::new(
                cost.costs
                    .iter()
                    .enumerate()
                    .map(|(idx, cost_info)| Bar::new(idx as f64, cost_info.val.log10()).width(1.0))
                    .collect(),
            );
            plot_ui.bar_chart(chart);
        });

        ui.separator();
        ui.label(format!("Cycle: {}", cycle));
        let max = cost.max();
        ui.label(format!("Max index: {}\nMax cost: {}", max.idx, max.val));
        let min = cost.min();
        ui.label(format!("Min index: {}\nMin cost: {}", min.idx, min.val));
        let avg = cost.avg;
        ui.label(format!("AVG: {}\n", avg));
        ui.separator();

        ui.label("{Index}:{Cost}");
        for cost_detail in cost.costs.iter() {
            ui.label(format!("{}:{}", cost_detail.idx, cost_detail.val));
        }
    } else {
        ui.heading("Please set focused cycle");
    }
}
