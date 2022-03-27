use std::process::Command;

fn main() {
    let mut config = prost_build::Config::new();
    config.bytes(&["."]);
    config.type_attribute(".", "#[derive(PartialOrd)]"); // 给所有的类型加入PartialOrd派生宏
    config
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
    Command::new("cargo")
        .args(&["fmt", "--", "src/*.rs"])
        .status()
        .expect("cargo fmt failed");

    println!("cargo:return-if-changed=build.rs");
    println!("cargo:return-if-changed=abi.proto");
}
