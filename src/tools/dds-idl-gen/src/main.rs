fn main() {
    println!("Hello, world!\n");
    let idl_src = r#"module gearState {
        @final 
        struct DataType {
          bool on;
        };
      };"#;
    let result = dust_dds_gen::compile_idl(idl_src).unwrap();
    println!("{}\n", result);

    let idl_src = r#"module day {
        @final 
        struct DataType {
          string day;
        };
      };"#;
    let result = dust_dds_gen::compile_idl(idl_src).unwrap();
    println!("{}\n", result);

    let idl_src = r#"module currentLightState {
        @final 
        struct DataType {
          bool on;
        };
      };"#;
    let result = dust_dds_gen::compile_idl(idl_src).unwrap();
    println!("{}", result);
}
