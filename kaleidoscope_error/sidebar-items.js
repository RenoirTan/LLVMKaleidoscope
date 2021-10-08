initSidebarItems({"struct":[["Error","A struct representing an error. As you can see from the type signature, you must provide a type (preferably an enum) that implements the 4 traits listed under the documentation for [`ErrorKind`]. This `EK` is used to classify the type of error that has occurred, and will be shown in a formatted string created by calling [`format!`] and related macros."]],"trait":[["ErrorKind","The traits every ErrorKind enum must satisfy. If your enum implements all of the following traits, then [`ErrorKind`] gets automatically implemented."]],"type":[["Result","A special [`std::result::Result`] type for Kaleidoscope. Instead of an error type parameter, you are instead asked for an ErrorKind enum type which implements the traits specified by [`ErrorKind`]. This error kind enum is used by [`Error`] to classify the error that has occurred."]]});