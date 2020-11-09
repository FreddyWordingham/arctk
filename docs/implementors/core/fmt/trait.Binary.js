(function() {var implementors = {};
implementors["ansi_rgb"] = [{"text":"impl&lt;T&gt; Binary for WithBackground&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Binary,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Binary for WithForeground&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Binary,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["console"] = [{"text":"impl&lt;D:&nbsp;Binary&gt; Binary for StyledObject&lt;D&gt;","synthetic":false,"types":[]}];
implementors["itertools"] = [{"text":"impl&lt;'a, I&gt; Binary for Format&lt;'a, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Binary,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["nalgebra"] = [{"text":"impl&lt;N, R:&nbsp;Dim, C:&nbsp;Dim, S&gt; Binary for Matrix&lt;N, R, C, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar + Binary,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Storage&lt;N, R, C&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;usize, R, C&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["ndarray"] = [{"text":"impl&lt;'a, A:&nbsp;Binary, S, D:&nbsp;Dimension&gt; Binary for ArrayBase&lt;S, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Data&lt;Elem = A&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["num_complex"] = [{"text":"impl&lt;T&gt; Binary for Complex&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Binary + Num + PartialOrd + Clone,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T:&nbsp;Binary + Clone + Integer&gt; Binary for Ratio&lt;T&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()