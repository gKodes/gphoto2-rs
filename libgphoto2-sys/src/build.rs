use std::env;
use std::path::PathBuf;

fn main() {
  let libgphoto2_dir = env::var_os("LIBGPHOTO2_DIR").map(PathBuf::from);

  #[cfg(feature = "test")]
  let libgphoto2_dir = libgphoto2_dir.or_else(|| Some(gphoto2_test::libgphoto2_dir().to_owned()));

  if let Some(libgphoto2_dir) = libgphoto2_dir {
    let paths = env::join_paths(
      std::iter::once(libgphoto2_dir.join("lib/pkgconfig").into_os_string())
        .chain(env::var_os("PKG_CONFIG_PATH")),
    )
    .unwrap();

    env::set_var("PKG_CONFIG_PATH", paths);
  }

  let lib = pkg_config::Config::new()
    .atleast_version("2.5.10")
    .probe("libgphoto2")
    .expect("Could not find libgphoto2");

  for link_path in lib.link_paths {
    println!("cargo:rustc-link-search=native={}", link_path.to_str().unwrap());
  }

  let bindings = bindgen::Builder::default()
    .clang_args(lib.include_paths.iter().map(|path| format!("-I{}", path.to_str().unwrap())))
    .header("src/wrapper.h")
    .generate_comments(true)
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
    .bitfield_enum("CameraFilePermissions")
    .bitfield_enum("CameraFileStatus")
    .bitfield_enum("Camera(File|Folder)?Operation")
    .bitfield_enum("Camera(File|Storage)InfoFields")
    .generate()
    .expect("Unable to generate bindings");

  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
}
