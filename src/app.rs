use std::{fmt::Debug, f64::consts::E, ops::RangeInclusive};
use eframe::epaint::RectShape;
use egui::{Vec2, FontId, TextStyle, Ui, Context, RichText, Color32, Style, Rect, Shape, Sense, plot::{Plot, Points, PlotPoints, PlotPoint, Line, PlotBounds, GridMark, GridInput}, accesskit::Point, Pos2};


#[derive(PartialEq, Clone, Copy, Debug)]
enum Choice { A = 0, B = 1, C = 2, D = 3, E = 4, NONE = 5}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Page { Response = 0, Results = 1, Settings = 2}
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:

    fontSizes: (f32, f32, f32, f32, f32), // Heading, Body, Monospace, Button, Small

    #[serde(skip)]
    currentPage: Page,
    
    #[serde(skip)]
    currentHighlight: Choice,
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    questions: Vec<Vec<(String, Choice, Choice)>>,
    
    #[serde(skip)]
    responses: Vec<(Choice, Choice)>,

    #[serde(skip)]
    tally: (i8, i8, i8, i8),

    #[serde(skip)]
    intensity: (i8, i8, i8, i8)
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            fontSizes: (30.0, 18.0, 14.0, 14.0, 10.0),
            currentPage: Page::Response,
            currentHighlight: Choice::NONE,
            questions: vec![
                vec![("enthusiastic".to_string(), Choice::B, Choice::B),
                    ("daring".to_string(), Choice::A, Choice::A),
                    ("diplomatic".to_string(), Choice::D, Choice::D),
                    ("satisfied".to_string(), Choice::C, Choice::C)],

                vec![("cautious".to_string(), Choice::D, Choice::D),
                    ("determined".to_string(), Choice::A, Choice::A),
                    ("convincing".to_string(), Choice::B, Choice::B),
                    ("good-natured".to_string(), Choice::C, Choice::E)],
                    
                vec![("friendly".to_string(), Choice::B, Choice::E),
                    ("accurate".to_string(), Choice::D, Choice::D),
                    ("outspoken".to_string(), Choice::A, Choice::A),
                    ("calm".to_string(), Choice::E, Choice::C)],
                
                vec![("talkative".to_string(), Choice::B, Choice::B),
                    ("controlled".to_string(), Choice::D, Choice::D),
                    ("conventional".to_string(), Choice::C, Choice::C),
                    ("decisive".to_string(), Choice::A, Choice::A)],
                    
                vec![("adventurous".to_string(), Choice::A, Choice::A),
                    ("insightful".to_string(), Choice::D, Choice::D),
                    ("outgoing".to_string(), Choice::B, Choice::B),
                    ("moderate".to_string(), Choice::C, Choice::C)],
                    
                vec![("gentle".to_string(), Choice::C, Choice::C),
                    ("persuasive".to_string(), Choice::B, Choice::E),
                    ("humble".to_string(), Choice::E, Choice::D),
                    ("original".to_string(), Choice::E, Choice::A)],
                
                vec![("expressive".to_string(), Choice::B, Choice::B),
                    ("conscientious".to_string(), Choice::D, Choice::D),
                    ("dominant".to_string(), Choice::A, Choice::A),
                    ("responsive".to_string(), Choice::E, Choice::C)], 
                    
                vec![("poised".to_string(), Choice::B, Choice::B),
                    ("observant".to_string(), Choice::D, Choice::E),
                    ("modest".to_string(), Choice::C, Choice::C),
                    ("impatient".to_string(), Choice::A, Choice::A)],
                    
                vec![("tactful".to_string(), Choice::D, Choice::D),
                    ("agreeable".to_string(), Choice::C, Choice::C),
                    ("magnetic".to_string(), Choice::B, Choice::B),
                    ("insistent".to_string(), Choice::A, Choice::A)],
                    
                vec![("brave".to_string(), Choice::A, Choice::A),
                    ("inspiring".to_string(), Choice::B, Choice::B),
                    ("submissive".to_string(), Choice::C, Choice::C),
                    ("timid".to_string(), Choice::E, Choice::D)],
                    
                vec![("reserved".to_string(), Choice::D, Choice::D),
                    ("obliging".to_string(), Choice::C, Choice::C),
                    ("strong-willed".to_string(), Choice::A, Choice::A),
                    ("cheerful".to_string(), Choice::B, Choice::B)],
                    
                vec![("stimulating".to_string(), Choice::B, Choice::B),
                    ("kind".to_string(), Choice::C, Choice::C),
                    ("perceptive".to_string(), Choice::D, Choice::D),
                    ("independent".to_string(), Choice::A, Choice::A)],
                    
                vec![("competitive".to_string(), Choice::A, Choice::A),
                    ("considerate".to_string(), Choice::C, Choice::C),
                    ("joyful".to_string(), Choice::B, Choice::B),
                    ("private".to_string(), Choice::D, Choice::D)],
                    
                vec![("fussy".to_string(), Choice::D, Choice::D),
                    ("obedient".to_string(), Choice::C, Choice::C),
                    ("firm".to_string(), Choice::A, Choice::A),
                    ("playful".to_string(), Choice::B, Choice::B)],
                    
                vec![("attractive".to_string(), Choice::B, Choice::B),
                    ("introspective".to_string(), Choice::D, Choice::E),
                    ("stubborn".to_string(), Choice::A, Choice::A),
                    ("predictable".to_string(), Choice::C, Choice::C)],
                    
                vec![("logical".to_string(), Choice::D, Choice::D),
                    ("bold".to_string(), Choice::A, Choice::A),
                    ("loyal".to_string(), Choice::C, Choice::C),
                    ("charming".to_string(), Choice::B, Choice::B)],
                    
                vec![("sociable".to_string(), Choice::B, Choice::B),
                    ("patient".to_string(), Choice::C, Choice::C),
                    ("self-reliant".to_string(), Choice::A, Choice::A),
                    ("soft-spoken".to_string(), Choice::D, Choice::D)],
                    
                vec![("willing".to_string(), Choice::C, Choice::C),
                    ("eager".to_string(), Choice::A, Choice::E),
                    ("thorough".to_string(), Choice::D, Choice::D),
                    ("high-spririted".to_string(), Choice::B, Choice::B)],
                    
                vec![("aggressive".to_string(), Choice::A, Choice::A),
                    ("extroverted".to_string(), Choice::B, Choice::B),
                    ("amiable".to_string(), Choice::C, Choice::C),
                    ("fearful".to_string(), Choice::E, Choice::D)],
                    
                vec![("confident".to_string(), Choice::B, Choice::B),
                    ("sympathetic".to_string(), Choice::C, Choice::C),
                    ("impartial".to_string(), Choice::E, Choice::D),
                    ("assertive".to_string(), Choice::A, Choice::A)],
                    
                vec![("well-disciplined".to_string(), Choice::D, Choice::D),
                    ("generous".to_string(), Choice::C, Choice::C),
                    ("animated".to_string(), Choice::B, Choice::B),
                    ("persistent".to_string(), Choice::A, Choice::A)],
                    
                vec![("impulsive".to_string(), Choice::B, Choice::B),
                    ("introverted".to_string(), Choice::D, Choice::D),
                    ("forceful".to_string(), Choice::A, Choice::A),
                    ("easygoing".to_string(), Choice::C, Choice::C)],
                    
                vec![("good mixer".to_string(), Choice::B, Choice::B),
                    ("refined".to_string(), Choice::D, Choice::D),
                    ("vigorous".to_string(), Choice::A, Choice::A),
                    ("lenient".to_string(), Choice::C, Choice::C)],
                    
                vec![("captivating".to_string(), Choice::B, Choice::B),
                    ("contented".to_string(), Choice::C, Choice::C),
                    ("demanding".to_string(), Choice::A, Choice::A),
                    ("compliant".to_string(), Choice::D, Choice::D)],
                    
                vec![("argumentative".to_string(), Choice::A, Choice::A),
                    ("systematic".to_string(), Choice::D, Choice::D),
                    ("cooperative".to_string(), Choice::C, Choice::C),
                    ("light-hearted".to_string(), Choice::B, Choice::B)],
                    
                vec![("jovial".to_string(), Choice::B, Choice::B),
                    ("precise".to_string(), Choice::D, Choice::D),
                    ("direct".to_string(), Choice::A, Choice::A),
                    ("even-tempered".to_string(), Choice::C, Choice::C)],
                    
                vec![("restless".to_string(), Choice::A, Choice::A),
                    ("neighborly".to_string(), Choice::C, Choice::C),
                    ("appealing".to_string(), Choice::B, Choice::B),
                    ("careful".to_string(), Choice::D, Choice::D)],
                    
                vec![("respectful".to_string(), Choice::D, Choice::D),
                    ("pioneering".to_string(), Choice::A, Choice::A),
                    ("optimistic".to_string(), Choice::B, Choice::B),
                    ("helpful".to_string(), Choice::C, Choice::C)]],

            responses: vec![(Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE)],

            tally: (0, 0, 0, 0),
            intensity: (0, 0, 0, 0)
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { fontSizes, currentPage, currentHighlight, questions , responses , tally, intensity} = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     // The top panel is often a good place for a menu bar:
        //     egui::menu::bar(ui, |ui| {
        //         ui.menu_button("File", |ui| {
        //             if ui.button("Quit").clicked() {
        //                 _frame.close();
        //             }
        //         });
        //     });
        // });

        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (TextStyle::Heading, FontId::new(fontSizes.0, egui::FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(fontSizes.1, egui::FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(fontSizes.2, egui::FontFamily::Proportional)),
            (TextStyle::Button, FontId::new(fontSizes.3, egui::FontFamily::Proportional)),
            (TextStyle::Small, FontId::new(fontSizes.4, egui::FontFamily::Proportional))

        ].into();
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("DiSC Program");
            ui.separator();

            ui.horizontal(|ui| {
                if ui.selectable_label(*currentPage == Page::Response, "Response Page").clicked() { *currentPage = Page::Response; }
                if ui.selectable_label(*currentPage == Page::Results, "Results Page").clicked() { *currentPage = Page::Results; process(responses, tally, intensity); }
                if ui.selectable_label(*currentPage == Page::Settings, "Settings Page").clicked() { *currentPage = Page::Settings; }
            });

            ui.collapsing(RichText::new("Instructions").strong().color(Color32::from_rgb(137, 207, 240)), |ui| {

                match *currentPage {
                    Page::Response => show_response_instructions(ui),
                    Page::Results => show_results_instructions(ui),
                    _ => ()
                }
            });

            ui.separator();
            ui.add_space(20.0);

            ui.set_min_width(ui.available_width());

            match currentPage {
                Page::Response => show_response_page(currentPage, questions, responses, tally, intensity, ctx, ui),
                Page::Results => show_results_page(currentHighlight, intensity, ui),
                Page::Settings => show_settings_page(fontSizes, ui)
            }
        });

        // if false {
        //     egui::Window::new("Window").show(ctx, |ui| {
        //         ui.label("Windows can be moved by dragging them.");
        //         ui.label("They are automatically sized based on contents.");
        //         ui.label("You can turn on resizing and scrolling if you like.");
        //         ui.label("You would normally choose either panels OR windows.");
        //     });
        // }
    }
}

fn show_response_page(currentPage: &mut Page, questions: &mut Vec<Vec<(String, Choice, Choice)>>, responses: &mut Vec<(Choice, Choice)>, tally: &mut (i8, i8, i8, i8), intensity: &mut (i8, i8, i8, i8), ctx: &Context, ui: &mut Ui) {

    egui::ScrollArea::vertical().show(ui, |ui| {

        egui::Grid::new("Response Page ".to_owned())
        .striped(true)
        .spacing(Vec2 {x: 10.0, y: 0.0})
        .min_row_height(4.0)
        .show(ui, |ui|{

            for i in 0..questions.len() {

                if i == 0 {

                    ui.label("");
                    ui.label(RichText::new("MOST").color(Color32::from_rgb(137, 207, 240)));
                    ui.label(RichText::new("LEAST").color(Color32::from_rgb(137, 207, 240)));
                    ui.end_row();
                }
                
                ui.label(RichText::new((i + 1).to_string().to_owned()).strong().color(Color32::from_rgb(137, 207, 240)));

                for j in 0..questions[i].len() {

                    ui.end_row();
                    ui.add_space(10.0);
                    ui.label(questions[i][j].0.to_owned());
                    ui.add_space(10.0);
                    ui.radio_value(&mut responses[i].0, questions[i][j].1, "");
                    ui.radio_value(&mut responses[i].1, questions[i][j].2, "");
                    ui.end_row();
                }

                ui.label("");
                ui.end_row();
            }
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
            ui.add_space(50.0);
            if ui.button("Next").clicked() { *currentPage = Page::Results; }
        });

        ui.add_space(20.0);
    });
}

fn show_response_instructions(ui: &mut Ui) {

    ui.add_space(10.0);

    egui::ScrollArea::vertical().show(ui, |ui| {

        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("1)").strong().color(Color32::from_rgb(137, 207, 240)));
            ui.label(RichText::new("Responding").color(Color32::from_rgb(137, 207, 240)));
        });
        ui.add_space(15.0);
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("A.").strong());
            ui.label(RichText::new("Study the first group of four words on the left while thinking about yourself in your selected setting or focus."));

        });
        ui.add_space(15.0);
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("B.").strong());
            ui.add_space(5.0);
            ui.label(RichText::new("Select"));
            ui.label(RichText::new("only one word").strong());
            ui.label(RichText::new("that"));
            ui.label(RichText::new("MOST").strong().color(Color32::from_rgb(137, 207, 240)));
            ui.label(RichText::new("describes you. Select the"));
            ui.label(RichText::new("first").strong());
            ui.label(RichText::new("bubble after the word in the"));
            ui.label(RichText::new("MOST").strong().color(Color32::from_rgb(137, 207, 240)));
            ui.label(RichText::new("column."));

        });
        ui.add_space(15.0);
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("C.").strong());
            ui.add_space(5.0);
            ui.label(RichText::new("Select"));
            ui.label(RichText::new("only one word").strong());
            ui.label(RichText::new("that"));
            ui.label(RichText::new("LEAST").strong().color(Color32::from_rgb(137, 207, 240)));
            ui.label(RichText::new("describes you. Select the"));
            ui.label(RichText::new("second").strong());
            ui.label(RichText::new("bubble after the word in the"));
            ui.label(RichText::new("LEAST").strong().color(Color32::from_rgb(137, 207, 240)));
            ui.label(RichText::new("column."));

        });
        ui.add_space(15.0);
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("D.").strong());
            ui.label(RichText::new("Use the same procedure to respond to the remaining groups of descriptive words. Feel free to look up the definitions if you are unsure what a word means."));

        });
        ui.add_space(15.0);
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("REMEMBER:").strong());
            ui.label(RichText::new("Select only"));
            ui.label(RichText::new("one").strong());
            ui.label(RichText::new("MOST").strong().color(Color32::from_rgb(137, 207, 240)));
            ui.label(RichText::new("and"));
            ui.label(RichText::new("one").strong());
            ui.label(RichText::new("LEAST").strong().color(Color32::from_rgb(137, 207, 240)));
            ui.label(RichText::new("choice for each group."));

        });
        ui.add_space(20.0);

        ui.group(|ui| {
            
            ui.label(RichText::new("EXAMPLE 1").strong());
            ui.add_space(5.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("The individual responding tends to be"));
                ui.label(RichText::new("MOST").strong().color(Color32::from_rgb(137, 207, 240)));
                ui.label(RichText::new("enthusiastic").italics());
                ui.label(RichText::new("and"));
                ui.label(RichText::new("LEAST").strong().color(Color32::from_rgb(137, 207, 240)));
                ui.label(RichText::new("satisfied").italics());
                ui.label(RichText::new("in his or her selected setting."));
            });
            ui.add_space(10.0);

            egui::Grid::new("Example 1".to_owned())
                .striped(true)
                .spacing(Vec2 {x: 10.0, y: 0.0})
                .min_row_height(4.0)
                .show(ui, |ui|{

                    ui.label("");
                    ui.label(RichText::new("MOST").strong().color(Color32::from_rgb(137, 207, 240)));
                    ui.label(RichText::new("LEAST").strong().color(Color32::from_rgb(137, 207, 240)));
                    ui.end_row();

                    ui.label(RichText::new("1".to_owned()).strong().color(Color32::from_rgb(137, 207, 240)));

                    ui.end_row();
                    ui.add_space(10.0);
                    ui.label("enthusiastic".to_owned());
                    ui.add_space(10.0);
                    ui.radio(true, "");
                    ui.radio(false, "");
                    ui.end_row();

                    ui.end_row();
                    ui.add_space(10.0);
                    ui.label("daring".to_owned());
                    ui.add_space(10.0);
                    ui.radio(false, "");
                    ui.radio(false, "");
                    ui.end_row();

                    ui.end_row();
                    ui.add_space(10.0);
                    ui.label("diplomatic".to_owned());
                    ui.add_space(10.0);
                    ui.radio(false, "");
                    ui.radio(false, "");
                    ui.end_row();

                    ui.end_row();
                    ui.add_space(10.0);
                    ui.label("satisfied".to_owned());
                    ui.add_space(10.0);
                    ui.radio(false, "");
                    ui.radio(true, "");
                    ui.end_row();
                });
        });
    });

    ui.add_space(10.0);

}

fn show_results_page(currentHighlight: &mut Choice, intensity: &mut (i8, i8, i8, i8), ui: &mut Ui) {

    egui::ScrollArea::vertical().show(ui, |ui| {      

        let x_fmt = |x, _range: &RangeInclusive<f64>| {

            let mut str = "".to_owned();

            if x == 1.0 {
                str = "D".to_owned();
            } else if x == 2.0 {
                str = "i".to_owned();
            } else if x == 3.0 {
                str = "S".to_owned();
            } else if x == 4.0 {
                str = "C".to_owned();
            } else {
                return String::new();
            }
            
            return str.to_owned();
        };

        let y_fmt = |y, _range: &RangeInclusive<f64>| {

            if y >= 1.0 && y <= 28.0 {
                return format!("{}", y);
            } else {
                return String::new();
            }
        };

        let y_spacer = |m: GridInput| {

            return vec![

                GridMark {value: 1.0, step_size: 4.0},
                GridMark {value: 5.0, step_size: 4.0},
                GridMark {value: 9.0, step_size: 4.0},
                GridMark {value: 13.0, step_size: 4.0},
                GridMark {value: 17.0, step_size: 4.0},
                GridMark {value: 21.0, step_size: 4.0},
                GridMark {value: 25.0, step_size: 4.0},
                GridMark {value: 28.0, step_size: 1.0},
            ]
        };

        Plot::new("Graph")
            .data_aspect(6.5 / 28.0)
            .view_aspect(0.75)
            .show_x(false)
            .show_y(false)
            .allow_drag(false)
            .allow_scroll(false)
            .allow_zoom(false)
            .allow_boxed_zoom(false)
            .x_axis_formatter(x_fmt)
            .y_axis_formatter(y_fmt)
            .y_grid_spacer(y_spacer)
            .show(ui, |plot_ui| {

                let series: PlotPoints = PlotPoints::Owned(vec![PlotPoint::new(1, intensity.0), PlotPoint::new(2, intensity.1), PlotPoint::new(3, intensity.2), PlotPoint::new(4, intensity.3)]);
                let line: egui::plot::Line = Line::new(series);                
                plot_ui.line(line);

                plot_ui.set_plot_bounds(PlotBounds::from_min_max([0.0, 0.0], [4.5, 28.0]));
            });

        ui.set_min_width(ui.available_width());
        egui::Grid::new("Numbers")
            .num_columns(5)
            .striped(true)
            .min_row_height(30.0)
            .min_col_width(ui.available_width() / 5.0)
            .show(ui, |ui| {

                ui.label("");
                if ui.selectable_label(*currentHighlight == Choice::A, RichText::new("D").strong().color(Color32::from_rgb(137, 207, 240))).clicked() { if *currentHighlight == Choice::A { *currentHighlight = Choice::NONE; } else { *currentHighlight = Choice::A; }};
                if ui.selectable_label(*currentHighlight == Choice::B, RichText::new("i").strong().color(Color32::from_rgb(137, 207, 240))).clicked() { if *currentHighlight == Choice::B { *currentHighlight = Choice::NONE; } else { *currentHighlight = Choice::B; }};
                if ui.selectable_label(*currentHighlight == Choice::C, RichText::new("S").strong().color(Color32::from_rgb(137, 207, 240))).clicked() { if *currentHighlight == Choice::C { *currentHighlight = Choice::NONE; } else { *currentHighlight = Choice::C; }};
                if ui.selectable_label(*currentHighlight == Choice::D, RichText::new("C").strong().color(Color32::from_rgb(137, 207, 240))).clicked() { if *currentHighlight == Choice::D { *currentHighlight = Choice::NONE; } else { *currentHighlight = Choice::D; }};

                ui.end_row();

                ui.label(RichText::new("Intensity"));
                ui.label(&intensity.0.to_string());
                ui.label(&intensity.1.to_string());
                ui.label(&intensity.2.to_string());
                ui.label(&intensity.3.to_string());

                ui.end_row();

                ui.label(RichText::new("Segment"));
                ui.label(&intensity_to_segment(intensity.0).to_string());
                ui.label(&intensity_to_segment(intensity.1).to_string());
                ui.label(&intensity_to_segment(intensity.2).to_string());
                ui.label(&intensity_to_segment(intensity.3).to_string());

                ui.end_row();
            });

        ui.set_min_width(ui.available_width());
        match currentHighlight {

            Choice::A => show_d_highlights(ui),
            Choice::B => show_i_highlights(ui),
            Choice::C => show_s_highlights(ui),
            Choice::D => show_c_highlights(ui),
            _ => ()

        }
    });
}

fn show_results_instructions(ui: &mut Ui) {}

fn show_settings_page(fontSizes: &mut (f32, f32, f32, f32, f32), ui: &mut Ui) {

    ui.add(egui::Slider::new(&mut fontSizes.0, 8.0..=32.0).text("Heading"));
    ui.add(egui::Slider::new(&mut fontSizes.1, 8.0..=32.0).text("Body"));
    //ui.add(egui::Slider::new(&mut fontSizes.2, 8.0..=32.0).text("Monospace"));
    ui.add(egui::Slider::new(&mut fontSizes.3, 8.0..=32.0).text("Button"));
    ui.add(egui::Slider::new(&mut fontSizes.4, 8.0..=32.0).text("Small"));
}

// Helper Functions

fn process(responses: &mut Vec<(Choice, Choice)>, tally: &mut (i8, i8, i8, i8), intensity: &mut (i8, i8, i8, i8)) {

    tally.0 = 0;
    tally.1 = 0;
    tally.2 = 0;
    tally.3 = 0;

    for response in responses {

        match response.0 {

            Choice::A => tally.0 += 1,
            Choice::B => tally.1 += 1,
            Choice::C => tally.2 += 1,
            Choice::D => tally.3 += 1,
            _ => ()
        }

        match response.1 {

            Choice::A => tally.0 -= 1,
            Choice::B => tally.1 -= 1,
            Choice::C => tally.2 -= 1,
            Choice::D => tally.3 -= 1,
            _ => ()
        }
    }

    intensity.0 = (27.38232853 / (1.0 + 0.297148753 * E.powf(-0.1801194362 * tally.0 as f64))).clamp(0.0, 28.0) as i8; // Logistic Regression
    intensity.1 = (28.13823356 / (1.0 + 1.242064677 * E.powf(-0.2464025952 * tally.1 as f64))).clamp(0.0, 28.0) as i8; // Logistic Regression
    intensity.2 = (29.51533099 / (1.0 + 2.209999802 * E.powf(-0.1941614665  * tally.2 as f64))).clamp(0.0, 28.0) as i8; // Logistic Regression
    intensity.3 = (27.31404101 / (1.0 + 0.5608447664 * E.powf(-0.2479183241  * tally.3 as f64))).clamp(0.0, 28.0) as i8; // Logistic Regression
}

fn intensity_to_segment(val: i8) -> i8 {

    return ((val - 1) / 4) + 1;
}

fn show_d_highlights(ui: &mut Ui) {

    egui::ScrollArea::horizontal().show(ui, |ui| {
        egui::Grid::new("Dominance")
            .num_columns(5)
            .min_col_width(ui.available_width() / 10.0)
            .show(ui, |ui| {

                ui.small(RichText::new("DOMINANCE").strong());
                ui.small(RichText::new("This person's tendencies include").color(Color32::from_rgb(137, 207, 240)));
                ui.small(RichText::new("This person desires an environment that includes").color(Color32::from_rgb(137, 207, 240)));
                ui.small(RichText::new("This person needs others who").color(Color32::from_rgb(137, 207, 240)));
                ui.small(RichText::new("To be more effective, this person needs").color(Color32::from_rgb(137, 207, 240)));
                ui.end_row();

                ui.small("Emphasis is on shaping the environment by overcoming opposition to accomplish results.");
                ui.small("• getting immediate results");
                ui.small("• power and authority");
                ui.small("• weigh pros and cons");
                ui.small("• to receive difficult assignments");
                ui.end_row();

                ui.small("");
                ui.small("• causing action");
                ui.small("• prestige and challenge");
                ui.small("• calculate risks");
                ui.small("• to understand that they need people");
                ui.end_row();

                ui.small("");
                ui.small("• accepting challanges");
                ui.small("• opportunities for individual accomplishments");
                ui.small("• use caution");
                ui.small("• to base techniques on practical experience");
                ui.end_row();

                ui.small("");
                ui.small("• making quick decisions");
                ui.small("• a wipe scrope of operations");
                ui.small("• create a predictable environment");
                ui.small("• to receive an occasional shock");
                ui.end_row();

                ui.small("");
                ui.small("• questioning the status quo");
                ui.small("• direct answers");
                ui.small("• research facts");
                ui.small("• to identify with a group");
                ui.end_row();

                ui.small("");
                ui.small("• taking authority");
                ui.small("• opportunities for advancement");
                ui.small("• deliberate before deciding");
                ui.small("• to verbalize reasons for conclusions");
                ui.end_row();

                ui.small("");
                ui.small("• managing trouble");
                ui.small("• freedom from controls and supervision");
                ui.small("• recognize the needs of others");
                ui.small("• to be aware of existing sanctions");
                ui.end_row();

                ui.small("");
                ui.small("• solving problems");
                ui.small("• many new and varied activities");
                ui.small("");
                ui.small("• to pace self and to relax more");

            });
    });
}

fn show_i_highlights(ui: &mut Ui) {}
fn show_s_highlights(ui: &mut Ui) {}
fn show_c_highlights(ui: &mut Ui) {}