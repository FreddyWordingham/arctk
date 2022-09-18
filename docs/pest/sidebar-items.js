window.SIDEBAR_ITEMS = {"enum":[["Atomicity","The current atomicity of a `ParserState`."],["Lookahead","The current lookahead status of a `ParserState`."],["MatchDir","Match direction for the stack. Used in `PEEK[a..b]`/`stack_match_peek_slice`."],["Token","A token generated by a `Parser`."]],"fn":[["set_call_limit","Sets the maximum call limit for the parser state to prevent stack overflows or excessive execution times in some grammars. If set, the calls are tracked as a running total over all non-terminal rules that can nest closures (which are passed to transform the parser state)."],["state","Creates a `ParserState` from a `&str`, supplying it to a closure `f`."]],"macro":[["fails_with","Testing tool that compares produced errors."],["parses_to","Testing tool that compares produced tokens."]],"mod":[["error","Types for different kinds of parsing failures."],["iterators","Types and iterators for parser output."],["prec_climber","Constructs useful in infix operator parsing with the precedence climbing method."]],"struct":[["Lines","Line iterator for Spans, created by `Span::lines()`."],["LinesSpan","Line iterator for Spans, created by `Span::lines_span()`."],["ParserState","The complete state of a `Parser`."],["Position","A cursor position in a `&str` which provides useful methods to manually parse that string."],["Span","A span over a `&str`. It is created from either two `Position`s or from a `Pair`."]],"trait":[["Parser","A trait with a single method that parses strings."],["RuleType","A trait which parser rules must implement."]],"type":[["ParseResult","Type alias to simplify specifying the return value of chained closures."]]};