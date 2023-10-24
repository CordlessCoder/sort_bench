mod distributions;
mod harness;
mod sorts;
use distributions::*;
use harness::*;
use plotters::prelude::*;
use sorts::*;

struct Colors;

macro_rules! colors {
    ($($col:expr),*) => {
        [$({let [r, g, b] = color_hex::color_from_hex!($col); (r,g,b) }),*]
    };
}

impl Palette for Colors {
    /// 0..16 are the base16 colors
    /// 16..18 are the background, forground and cursor color
    #[rustfmt::skip]
    const COLORS: &'static [(u8, u8, u8)] = &colors!(
        "#1E1E2F", "#F38BA8", "#A6E3A1", "#F9E2AF", "#89B4FA", "#F5C2E7", "#94E2D5", "#BAC2DE",
        "#4e4e75", "#F38BA8", "#A6E3A1", "#F9E2AF", "#89B4FA", "#F5C2E7", "#94E2D5", "#A6ADC8",
        "#1E1E2F", "#CDD6F4", "#8CAAEE",
        "#5C5F77", "#D20F39", "#40A02B", "#DF8E1D", "#1E66F5", "#EA76CB", "#179299", "#ACB0BE",
        "#6C6F85", "#D20F39", "#40A02B", "#DF8E1D", "#1E66F5", "#EA76CB", "#179299", "#BCC0CC",
        "#EFF1F5", "#4C4F69", "#1E66F5"
    );
}

fn main() {
    let mut rng = rand::thread_rng();
    // let lengths = [100, 1_000, 10_000, 100_000, 1_000_000];
    let lengths = [100, 1_000, 10_000];
    let results = bench::<
        i32, // The type we'll be sorting
        // The input distributions
        (
            Uniform,
            Sorted,
            Reverse,
            AllEqual,
            Shuffled,
            ShuffledValues<16>,
        ),
        (BubbleSort, InsertionSort), // Sorting methods
    >(&mut rng, &lengths, 2);

    let dark = true;
    let get_color = |idx| Colors::pick(if dark { idx } else { idx + 18 });
    // let background = color(plotters::style::Palette)
    let font = |size| {
        FontFamily::SansSerif
            .into_font()
            .resize(size)
            .with_color(&get_color(17))
    };

    let [unstable, stable] = results;
    for (stable, data) in [("unstable", unstable), ("stable", stable)] {
        for (size, data) in data {
            let path = format!("images/{stable}_{size}.png");
            let root = BitMapBackend::new(&path, (1920, 1080)).into_drawing_area();

            root.fill(&get_color(16)).unwrap();

            // Boxplot::new_horizontal(key, quartiles)
            let distr_count = data.values().map(|d| d.len()).max().unwrap_or(0) as i32;
            let method_count = data.values().len() as i32;
            let max_runtime = data
                .values()
                .map(|v| {
                    v.iter()
                        .map(|x| x.1.time.as_nanos() as i32)
                        .max()
                        .unwrap_or(0)
                })
                .max()
                .unwrap_or(0);

            const MARGIN: u32 = 15;
            const BAR_STROKE: u32 = 2;
            let bar_colors: [_; 6] = std::array::from_fn(|idx| get_color(1 + idx));
            let (left, right) = root.split_horizontally((20).percent());
            let mut chart = ChartBuilder::on(&right)
                .caption(format!("Sorting {size} elements"), font(40.))
                .margin_top(MARGIN)
                .margin_right(MARGIN)
                .margin_bottom(MARGIN)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .build_cartesian_2d(
                    0..(max_runtime as f64 * 1.15) as i32,
                    (0..(method_count + 1) * (distr_count)).into_segmented(),
                )
                .unwrap();
            chart
                .configure_mesh()
                .disable_y_mesh()
                .disable_y_axis()
                .axis_style(get_color(17))
                .bold_line_style(get_color(7))
                .light_line_style(get_color(8))
                .label_style(font(22.))
                .x_label_formatter(&|&x| (x as f64 / size as f64).to_string())
                .x_desc("Cycles per element")
                .axis_desc_style(font(22.))
                .draw()
                .unwrap();
            let text = ChartBuilder::on(&left)
                .margin_left(MARGIN)
                .margin_top(MARGIN)
                .margin_bottom(MARGIN)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .build_cartesian_2d(
                    0..1,
                    (0..(method_count + 1) * (distr_count)).into_segmented(),
                )
                .unwrap();
            let font_color = get_color(17);
            data.iter()
                .enumerate()
                .map(|(i, sort)| (i as i32, sort))
                .zip(bar_colors.iter().cycle())
                .for_each(|((ytop, (sort, results)), color)| {
                    chart
                        .draw_series(
                            (ytop + 1..)
                                .step_by(method_count as usize + 1)
                                .zip(results.into_iter())
                                .flat_map(|(y, (dist, bench))| {
                                    text.plotting_area()
                                        .draw(&Text::new(
                                            dist.to_string(),
                                            (
                                                0,
                                                SegmentValue::Exact(
                                                    (y - 1) / (method_count + 1)
                                                        * (method_count + 1)
                                                        + method_count / 2
                                                        + method_count % 2
                                                        + 1,
                                                ),
                                            ),
                                            {
                                                let mut style = font_color
                                                    .into_text_style(text.plotting_area());
                                                style.font = FontDesc::new(
                                                    FontFamily::SansSerif,
                                                    26.,
                                                    FontStyle::Normal,
                                                );
                                                style
                                            },
                                        ))
                                        .unwrap();
                                    let coords = [
                                        (0, SegmentValue::Exact(y)),
                                        (bench.time.as_nanos() as i32, SegmentValue::Exact(y + 1)),
                                    ];
                                    [
                                        Rectangle::new(coords.clone(), color.filled()),
                                        Rectangle::new(
                                            {
                                                let mut c = coords;
                                                let offset = BAR_STROKE as i32 / 2;
                                                c[0].0 += offset;
                                                c[1].0 += offset;
                                                c
                                            },
                                            get_color(8).stroke_width(2),
                                        ),
                                    ]
                                }),
                        )
                        .unwrap()
                        .label(sort)
                        .legend(move |(x, y)| {
                            PathElement::new(vec![(x, y), (x + 20, y)], color.stroke_width(5))
                        });
                });

            text.plotting_area()
                .draw(&Text::new(
                    "Input distribution",
                    (0, SegmentValue::Exact((method_count + 1) * (distr_count))),
                    {
                        let mut style = font_color.into_text_style(text.plotting_area());
                        style.font = FontDesc::new(FontFamily::SansSerif, 26., FontStyle::Normal);
                        style
                    },
                ))
                .unwrap();
            chart
                .configure_series_labels()
                .border_style(&get_color(8))
                .background_style(&get_color(16).mix(0.8))
                .label_font(
                    FontFamily::SansSerif
                        .into_font()
                        .resize(30.)
                        .with_color(get_color(17)),
                )
                .draw()
                .unwrap();
            // chart
            //     .draw_series(LineSeries::new(
            //         (-314..314).map(|x| x as f64 / 100.0).map(|x| (x, x.sin())),
            //         &RED,
            //     ))
            //     .unwrap();
            root.present().unwrap();
        }
    }
}
