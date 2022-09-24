| Macros                     | Define                                                         | Usage                | note   | Example    |
|----------------------------|----------------------------------------------------------------|----------------------|--------|------------|
| Declarative Macro          | #[macro_export]/macro_rules! macro_name{}                      | macro_name!()        |        | println!   |
| -------------------------- | -------------------------------------------------------------- | -------------------- | ------ | ---------- |
| Function Macro             | #[proc_macros]/pub fn macro_name                               | macro_name!()        |        |            |
| -------------------------- | -------------------------------------------------------------- | -------------------- | ------ | ---------- |
| Derive Macro               | #[proc_macros_derive(DeriveMacroName)]/pub  fn other_fn_name   | DeriveMacroName!()   |        |            |
| -------------------------- | -------------------------------------------------------------- | -------------------- | ------ | ---------- |
| Attritubte Macro           |                                                                |                      |        |            |


| 种类                    | Define                                                       | Usage              | note | Example  |
|-----------------------|--------------------------------------------------------------|--------------------|------|----------|
| 声明宏-Declarative Macro | #[macro_export]/macro_rules! macro_name{}                    | macro_name!()      |      | println! |
| 函数宏-Function Macro    | #[proc_macros]/pub fn macro_name                             | macro_name!()      |      |          |
| 派生宏-Derive Macro      | #[proc_macros_derive(DeriveMacroName)]/pub  fn other_fn_name | DeriveMacroName!() |      |          |
| 属性宏-Attritubte Macro  |                                                              |                    |      |          |
