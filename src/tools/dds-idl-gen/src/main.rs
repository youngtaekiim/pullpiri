fn main() {
    println!("Hello, world!");
    let idl_src = r#"module currentLightState {
        @final 
        struct DataType {
          bool on;
        };
      };"#;
    let result = dust_dds_gen::compile_idl(idl_src).unwrap();
    println!("{}", result);
}
