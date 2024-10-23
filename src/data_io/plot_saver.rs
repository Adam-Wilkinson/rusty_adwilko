extern crate pyo3;
extern crate numpy;

use std::cmp::Ordering;
use std::ops::Range;

use crate::data_io::Savable;
use crate::domain::{CalculationResults, OneDimensionalDomain, TwoDimensionalDomain};
use num::complex::Complex64;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyList, PyTuple};
use numpy::{PyArray, PyArray2, PyArrayMethods};

pub struct Plot<'a> {
    pub extra_funcs : &'a [&'a str],
    pub scale : Scale,
    pub show : bool,
    pub style : PlotStyleDef<'a>,
    pub color_theme : ColorTheme<'a>,
}

#[derive(Debug, PartialEq)]
pub enum Scale {
    Linear,
    Logarithmic { clipping : Range<f64>},
}

pub struct ColorTheme<'a> {
    text_color : &'a str,
    axes_label_color : &'a str,
    tick_color : &'a str,
}

pub const LIGHT : ColorTheme = ColorTheme {
    text_color : "black",
    axes_label_color : "black",
    tick_color : "black",
};

pub const DARK : ColorTheme = ColorTheme {
    text_color : "white",
    axes_label_color : "white",
    tick_color : "white",
};

pub struct PlotStyleDef<'a> {
    font_family : &'a str,
    label_size : usize,
    font_size : usize,
    legend_fontsize : usize,
    tick_labelsize : usize,
}

pub const PAPER_STYLE : PlotStyleDef = PlotStyleDef {
    font_family : "serif",
    label_size : 12,
    font_size : 12,
    legend_fontsize : 8,
    tick_labelsize : 10,
};

pub const PRESENTATION_STYLE : PlotStyleDef = PlotStyleDef {
    font_family : "serif",
    label_size : 20,
    font_size : 20,
    legend_fontsize : 16,
    tick_labelsize : 14,
};

impl<'a> Default for Plot<'a> {
    fn default() -> Self {
        Plot { extra_funcs : &[], scale : Scale::Linear, show : false, style : PAPER_STYLE, color_theme : LIGHT }
    }
}

impl<'a> Savable<Plot<'a>, PyErr> for Vec<Complex64> {
    fn save(&self, file_path : &str, context : &Plot<'a>) -> Result<(), PyErr> {
        Python::with_gil(|py| {
            let plt = configured_plt(py, &context)?;
            let (fig , ax_numpy) : (Bound<PyAny>, Bound<PyAny>) = plt.getattr("subplots")?.call1((2,))?.downcast_into::<PyTuple>()?.extract()?;
            let ax = ax_numpy.getattr("tolist")?.call0()?.downcast_into::<PyList>()?;
            let (real_axes, imag_axes) = (ax.get_item(0)?, ax.get_item(1)?);
            
            real_axes.getattr("set_title")?.call1(("Real Part",))?;
            imag_axes.getattr("set_title")?.call1(("Imaginary Part",))?;

            real_axes.getattr("plot")?.call1((PyList::new_bound(py, self.iter().map(|x| x.re).collect::<Vec<f64>>()),))?;
            imag_axes.getattr("plot")?.call1((PyList::new_bound(py, self.iter().map(|x| x.im).collect::<Vec<f64>>()),))?;

            fig.getattr("savefig")?.call1((file_path.to_owned() + ".pdf",))?;
            if context.show {
                plt.getattr("show")?.call0()?;
            }

            Ok::<(), PyErr>(())
        })
    }
}

impl<'a> Savable<Plot<'a>, PyErr> for Vec<f64> {
    fn save(&self, file_path : &str, context : &Plot) -> Result<(), PyErr> {
        Python::with_gil(|py| {
            let plt = configured_plt(py, &context)?;
            plt.getattr("plot")?.call1((PyList::new_bound(py, self),))?;

            call_extra_funcs(py, "matplotlib.pyplot", context.extra_funcs)?;

            plt.getattr("savefig")?.call1((file_path.to_owned() + ".pdf",))?;
            if context.show {
                plt.getattr("show")?.call0()?;
            }

            Ok::<(), PyErr>(())
        })
    }
}

impl<'a, const N : usize> Savable<Plot<'a>, PyErr> for CalculationResults<'a, OneDimensionalDomain, Complex64, N> {
    fn save(&self, file_path : &str, context : &Plot<'a>) -> Result<(), PyErr> {
        Python::with_gil(|py| {
            let plt = configured_plt(py, &context)?;
            let (fig, ax_numpy) : (Bound<PyAny>, Bound<PyAny>) = plt.getattr("subplots")?.call1((2,))?.extract()?;
            let ax: Bound<PyList> = ax_numpy.getattr("tolist")?.call0()?.downcast_into()?;
            let (real_axes, imag_axes) = (ax.get_item(0)?, ax.get_item(1)?);

            real_axes.getattr("set_title")?.call1(("Real Part",))?;
            imag_axes.getattr("set_title")?.call1(("Imaginary Part",))?;

            let x_axis = PyList::new_bound(py, &self.domain_data.values);

            let (min, max) = match &context.scale {
                Scale::Linear => (f64::NEG_INFINITY, f64::INFINITY),
                Scale::Logarithmic { clipping } => (clipping.start, clipping.end)
            };

            if let Scale::Logarithmic { clipping : _ } = &context.scale {
                [&real_axes, &imag_axes].iter().try_for_each(|ax| {
                    ax.getattr("set_yscale")?.call1(("log",))?;
                    ax.getattr("set_ylabel")?.call1(("log",))?;
                    ax.getattr("set_xscale")?.call1(("log",))?;
                    ax.getattr("set_xlabel")?.call1(("log",))?;
                    Ok::<(), pyo3::PyErr>(())
                })?;
            }

            self.results.iter()
                .map(|y_axis| {
                    real_axes.getattr("plot")?.call1((&x_axis, PyList::new_bound(py, y_axis.iter().map(|x| x.re.clamp(min, max)).collect::<Vec<f64>>()),))?;
                    imag_axes.getattr("plot")?.call1((&x_axis, PyList::new_bound(py, y_axis.iter().map(|x| x.im.clamp(min, max)).collect::<Vec<f64>>()),))
                }).collect::<Result<Vec<_>, _>>()?;

            if !self.result_names.iter().all(String::is_empty) {
                    plt.getattr("legend")?.call1((PyList::new_bound(py, &self.result_names),))?;
            }

            call_extra_funcs(py, "matplotlib.pyplot", context.extra_funcs)?;

            fig.getattr("savefig")?.call1((file_path.to_owned() + ".pdf",))?;
            if context.show {
                plt.getattr("show")?.call0()?;
            }

            Ok::<(), PyErr>(())
        })
    }
}

impl<'a, const N : usize> Savable<Plot<'a>, PyErr> for CalculationResults<'a, OneDimensionalDomain, f64, N> {
    fn save(&self, file_path : &str, context : &Plot<'a>) -> Result<(), PyErr> {
        Python::with_gil(|py| {
            let plt = configured_plt(py, &context)?;

            let x_axis = PyList::new_bound(py, &self.domain_data.values);

            let (min, max) = match &context.scale {
                Scale::Linear => (f64::NEG_INFINITY, f64::INFINITY),
                Scale::Logarithmic { clipping } => (clipping.start, clipping.end)
            };

            self.results.iter()
                .map(|result| plt.getattr("plot")?.call1((&x_axis, PyList::new_bound(py, result.iter().map(|val| val.clamp(min, max))))))
                .collect::<Result<Vec<_>, _>>()?;

            if !self.result_names.iter().all(String::is_empty) {
                plt.getattr("legend")?.call1((PyList::new_bound(py, &self.result_names),))?;
            }

            call_extra_funcs(py, "matplotlib.pyplot", context.extra_funcs)?;

            let savefig_kwargs = PyDict::new_bound(py);
            savefig_kwargs.set_item("bbox_inches", "tight")?;
            savefig_kwargs.set_item("facecolor", "none")?;

            if let Scale::Logarithmic { clipping: _ } = &context.scale {
                plt.getattr("yscale")?.call1(("log",))?;
                plt.getattr("ylabel")?.call1(("log",))?;
                plt.getattr("xscale")?.call1(("log",))?;
                plt.getattr("xlabel")?.call1(("log",))?;
            }

            plt.getattr("savefig")?.call((file_path.to_owned() + ".pdf",), Some(&savefig_kwargs))?;
            if context.show {
                plt.getattr("show")?.call0()?;
            }

            Ok::<(), PyErr>(())
        })
    }
}

impl<'a, const N : usize> Savable<Plot<'a>, PyErr> for CalculationResults<'a, TwoDimensionalDomain, f64, N> {
    fn save(&self, file_path : &str, context : &Plot<'a>) -> Result<(), PyErr> {
        Python::with_gil(|py| {
            let np = py.import_bound("numpy")?;
            let plt = configured_plt(py, &context)?;
            let (nrows, ncols) = graph_layout_from_index(N);
            let (fig, ax) : (Bound<PyAny>, Bound<PyAny>) = plt.getattr("subplots")?.call1((nrows, ncols,))?.extract()?;
            let flattened_ax_binding = np
                .getattr("array")?.call1((ax,))?
                .getattr("flatten")?.call0()?
                .downcast_into::<PyArray<PyObject, numpy::ndarray::Ix1>>()?
                .readonly();
                
            let flattened_ax = flattened_ax_binding.as_slice()?;
            
            let (context_min, context_max) = match &context.scale {
                Scale::Linear => (f64::NEG_INFINITY, f64::INFINITY),
                Scale::Logarithmic { clipping } => (clipping.start, clipping.end)
            };

            let plot_kwargs = PyDict::new_bound(py);
            plot_kwargs.set_item("origin", "lower")?;
            plot_kwargs.set_item("extent", [self.domain_data.x_limits.0, self.domain_data.x_limits.1, self.domain_data.y_limits.0, self.domain_data.y_limits.1])?;
            plot_kwargs.set_item("cmap", "seismic")?;

            self.results.iter()
                .zip(flattened_ax)
                .map(|(result, current_ax)| {
                    let my_plot_kwargs = plot_kwargs.copy()?;
                    let minimum = (*result.iter().min_by(|x, y| if x.is_nan() { Ordering::Greater } else { x.total_cmp(y) }).unwrap()).max(context_min);
                    let maximum = (*result.iter().min_by(|x, y| if x.is_nan() { Ordering::Greater } else if y.is_nan() { Ordering::Less } else { y.total_cmp(x) }).unwrap()).min(context_max);

                    if minimum < 0.0 {
                        my_plot_kwargs.set_item("cmap", "seismic")?;
                        my_plot_kwargs.set_item("vmin", -f64::max(-minimum, maximum))?;
                        my_plot_kwargs.set_item("vmax", f64::max(-minimum, maximum))?;
                        if let Scale::Logarithmic { clipping : _ } = context.scale {
                            my_plot_kwargs.set_item("norm", "symlog")?;
                        }
                    }
                    else {
                        my_plot_kwargs.set_item("cmap", "magma")?;
                        my_plot_kwargs.set_item("vmin", f64::max(minimum, 1e-25))?;
                        my_plot_kwargs.set_item("vmax", maximum)?;
                        if let Scale::Logarithmic { clipping : _ } = context.scale {
                            my_plot_kwargs.set_item("norm", "log")?;
                        }
                    }

                    let clamped_result = result.map(|value| value.clamp(minimum, maximum));

                    let image = current_ax.getattr(py, "imshow")?.call_bound(py, (PyArray2::from_array_bound(py, &clamped_result),), Some(&my_plot_kwargs))?;
                    fig.getattr("colorbar")?.call((image,), Some(&[("ax", current_ax)].into_py_dict_bound(py)))
                }).collect::<Result<Vec<_>, _>>()?;

            self.result_names.iter()
                .zip(flattened_ax)
                .filter(|(s, _)| !s.is_empty())
                .map(|(title, current_ax)| {
                    current_ax.getattr(py, "set_title")?.call1(py, (title,))
                }).collect::<Result<Vec<_>,  _>>()?;

            call_extra_funcs(py, "matplotlib.pyplot", context.extra_funcs)?;

            let savefig_kwargs = PyDict::new_bound(py);
            savefig_kwargs.set_item("bbox_inches", "tight")?;
            savefig_kwargs.set_item("facecolor", "none")?;

            fig.getattr("savefig")?.call((file_path.to_owned() + ".pdf",), Some(&savefig_kwargs))?;
            if context.show {
                plt.getattr("show")?.call0()?;
            }

            Ok::<(), PyErr>(())
        })
    }
}

fn graph_layout_from_index(index : usize) -> (usize, usize) {
    match index {
        1 => (1,1), 2 => (1,2), 3 => (1,3), 4 => (2,2), 5 => (2,3), 6 => (2,3), _ => (3,3)
    }
}

fn call_extra_funcs<'a>(py : Python<'a>, module_name : &str, extra_funcs : &[&str]) -> Result<Vec<Bound<'a, PyAny>>, PyErr> {
    let module_local = module_name.replace(".", "");
    let locals = [(module_local.clone(), &py.import_bound(module_name)?)].into_py_dict_bound(py);
    extra_funcs.into_iter().map(|extra_func| {
        py.eval_bound(&((&module_local).clone() + "." + extra_func), None, Some(&locals))
    }).collect::<Result<Vec<_>, _>>()
}

fn configured_plt<'a>(py : Python<'a>, plot : &Plot) -> Result<Bound<'a, PyModule>, PyErr> {
    let mpl = py.import_bound("matplotlib")?;
    let style_params = PyDict::new_bound(py);
    let plt = py.import_bound("matplotlib.pyplot")?;
    plt.getattr("style")?.getattr("use")?.call1(("seaborn-v0_8",))?;
    style_params.set_item("text.usetex", true)?;
    style_params.set_item("font.family", plot.style.font_family)?;
    style_params.set_item("axes.labelsize", plot.style.label_size)?;
    style_params.set_item("font.size", plot.style.font_size)?;
    style_params.set_item("legend.fontsize", plot.style.legend_fontsize)?;
    style_params.set_item("xtick.labelsize", plot.style.tick_labelsize)?;
    style_params.set_item("ytick.labelsize", plot.style.tick_labelsize)?;
    style_params.set_item("text.color", plot.color_theme.text_color)?;
    style_params.set_item("axes.labelcolor", plot.color_theme.axes_label_color)?;
    style_params.set_item("xtick.color", plot.color_theme.tick_color)?;
    style_params.set_item("ytick.color", plot.color_theme.tick_color)?;
    style_params.set_item("figure.figsize", figure_size(800.0))?;
    mpl.getattr("rcParams")?.getattr("update")?.call1((style_params,))?;
    return Ok(plt)
}

fn figure_size(width : f64) -> (f64, f64) {
    let inches_per_pt = 1.0 / 72.27;
    let golden_ratio = (5.0_f64.sqrt() - 1.0) / 2.0;
    let fig_width_in = width * inches_per_pt;
    let fig_height_in = fig_width_in * golden_ratio;
    return (fig_width_in, fig_height_in)
}