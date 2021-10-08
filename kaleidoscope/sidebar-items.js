initSidebarItems({"enum":[["TokenKind","The type of token a token is."]],"macro":[["function_name","Get the name of the function (no paths) of the current function/method."],["function_path","Get the full qualified name of the current function/method."],["hash_map","Create a hash map"],["impl_display","Implement a default version of [`std::fmt::Display`] for a type if that type already implements [`std::fmt::Debug`]."],["untrimmed_function_path","See [`function_path`]."]],"mod":[["nodes","A module defining all the nodes in a Kaleidoscope Abstract Syntax Tree."]],"struct":[["CodeGen","A structure representing an LLVM IR generator."],["Driver","The driver that brings input from a file stream to the parser."],["Error","A struct representing an error. As you can see from the type signature, you must provide a type (preferably an enum) that implements the 4 traits listed under the documentation for [`ErrorKind`]. This `EK` is used to classify the type of error that has occurred, and will be shown in a formatted string created by calling [`format!`] and related macros."],["Interpreter","A full interpreter that can parse a programme by itself."],["NodeId","An ID for each node type."],["Parser","The parser struct that converts a Kaleidoscope program into an Abstract Syntax Tree."],["To64LLVMWord","An empty struct to convert integer types into 64-bit word arrays."],["Token","A token in a Kaleidoscope file."],["Tokenizer","The tokeniser which iterates over the characters in a file stream and yields a stream of tokens."]],"trait":[["ErrorKind","The traits every ErrorKind enum must satisfy. If your enum implements all of the following traits, then [`ErrorKind`] gets automatically implemented."],["IRRepresentableExpression","A node that implements this trait can be converted into LLVM IR."],["IRRepresentableNode",""],["Node","The trait that all node types must implement."]],"type":[["ParseResult","The return type of most parser functions in [`Parser`]."],["Result","A special [`std::result::Result`] type for Kaleidoscope. Instead of an error type parameter, you are instead asked for an ErrorKind enum type which implements the traits specified by [`ErrorKind`]. This error kind enum is used by [`Error`] to classify the error that has occurred."]]});