(function() {var implementors = {};
implementors["ndarray"] = [{"text":"impl&lt;'a, A, S, S2, D, E&gt; BitAndAssign&lt;&amp;'a ArrayBase&lt;S2, E&gt;&gt; for ArrayBase&lt;S, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Clone + BitAndAssign&lt;A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: DataMut&lt;Elem = A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S2: Data&lt;Elem = A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Dimension,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: Dimension,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A, S, D&gt; BitAndAssign&lt;A&gt; for ArrayBase&lt;S, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: ScalarOperand + BitAndAssign&lt;A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: DataMut&lt;Elem = A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Dimension,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()