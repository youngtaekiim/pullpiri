fn main() {
    println!("Hello, world!");
    let idl_src = r#"module gearState {
        @final 
        struct DataType {
          string gear;
        };
      };"#;
    let result = dust_dds_gen::compile_idl(idl_src).unwrap();
    println!("{}", result);
}
