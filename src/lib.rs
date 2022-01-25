extern crate pyo3;
use pyo3::prelude::*;

#[cfg(feature = "blosc")]
use hdf5::filters::blosc_set_nthreads;
use hdf5::{File, H5Type, Result};
use ndarray;
use ndarray::{arr2, s};

#[derive(H5Type, Clone, PartialEq, Debug)] // register with HDF5
#[repr(u8)]
pub enum Color {
    R = 1,
    G = 2,
    B = 3,
}

#[derive(H5Type, Clone, PartialEq, Debug)] // register with HDF5
#[repr(C)]
pub struct Pixel {
    xy: (i64, i64),
    color: Color,
}

impl Pixel {
    pub fn new(x: i64, y: i64, color: Color) -> Self {
        Self { xy: (x, y), color }
    }
}

fn write_hdf5() -> Result<()> {
    use Color::*;
    let file = File::create("pixels.h5").expect("Can not create file"); // open for writing
    println!("Creating file: {:?}",&file.filename());
    let group = file.create_group("dir")?; // create a group
    #[cfg(feature = "blosc")]
    blosc_set_nthreads(2); // set number of blosc threads
    let builder = group.new_dataset_builder();
    #[cfg(feature = "blosc")]
    let builder = builder.blosc_zstd(9, true); // zstd + shuffle
    let ds = builder
        .with_data(&arr2(&[
            // write a 2-D array of data
            [Pixel::new(1, 2, R), Pixel::new(2, 3, B)],
            [Pixel::new(3, 4, G), Pixel::new(4, 5, R)],
            [Pixel::new(5, 6, B), Pixel::new(6, 7, G)],
        ]))
        // finalize and write the dataset
        .create("pixels")?;
    // create an attr with fixed shape but don't write the data
    let attr = ds.new_attr::<Color>().shape([3]).create("colors")?;
    // write the attr data
    attr.write(&[R, G, B])?;
    Ok(())
}

fn read_hdf5() -> Result<()> {

    use Color::*;
    let file = File::open("pixels.h5")?; // open for reading
    println!("Reading file: {:?}",&file.filename());
    let ds = file.dataset("dir/pixels")?; // open the dataset
    assert_eq!(
        // read a slice of the 2-D dataset and verify it
        ds.read_slice::<Pixel, _, _>(s![1.., ..])?,
        arr2(&[
            [Pixel::new(3, 4, G), Pixel::new(4, 5, R)],
            [Pixel::new(5, 6, B), Pixel::new(6, 7, G)],
        ])
    );
    let attr = ds.attr("colors")?; // open the attribute
    assert_eq!(attr.read_1d::<Color>()?.as_slice().unwrap(), &[R, G, B]);
    Ok(())
}


#[pyclass]
struct PyData {
}

#[pymethods]
impl PyData {
    #[new]
    fn init() -> Self {
        PyData { }
    }
    fn run<'py>(&self) {
        println!("Running a pure rust function in python");
        rust_function();
    }
}


fn rust_function() {
    println!("Welcome to pure Rust!");
    println!("Running HDF5-test");
    write_hdf5().expect("Failed writing HDF5 file!");
    read_hdf5().expect("Failed reading HDF5 file!");
    println!("done!.");
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn py03test(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyData>()?;
    Ok(())
}
