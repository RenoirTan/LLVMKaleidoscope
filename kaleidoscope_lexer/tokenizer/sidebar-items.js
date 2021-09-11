initSidebarItems({"mod":[["filestream","A special structure which iterates over the characters in a file. It has special functions to ensure smooth traversal through a file or any sequence that can be iterated over."],["lexerser","A module for serialising a tokeniser. This only exists so I can convert a Kaleidoscope programme into a JSON file."],["lexertuple","Some tuples storing a [`FileStream`] and a [`Tokenizer`]."],["tokenizer","A struct that reads a file and creates tokens from them."],["tokiter","A module defining a [`TokenIterator`] that can continually read a file stream and spit out tokens in a for loop."]],"struct":[["FileStream","A file stream which returns a unicode codepoint one at a time. This is in contrast to a normal [`std::fs::File`] which can only read bytes to an array."],["LexerSerializer","Serialises a [`TokenIterator`] into a list of tokens."],["LexerTupleMut","A tuple storing mutable references to a [`FileStream`] and a [`Tokenizer`]. Both of these objects can be used to create a stream of tokens."],["LexerTupleRef","A tuple storing immutable references to a [`FileStream`] and a [`Tokenizer`]. This is basically useless because you need to be able to mutate both the stream and the tokenizer to create tokens. In order to use that functionality, please see [`LexerTupleMut`]."],["TokenIterator","A structure that takes a [`FileStream`] and reads the characters to produce one token for each iteration in a for loop."],["Tokenizer","The tokeniser which iterates over the characters in a file stream and yields a stream of tokens."]]});