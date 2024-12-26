//类属性宏
#[route(GET, "/")]
fn index() {}

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream{}

//类函数宏
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream{}
