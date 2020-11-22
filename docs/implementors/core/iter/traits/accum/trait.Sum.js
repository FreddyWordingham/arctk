(function() {var implementors = {};
implementors["nalgebra"] = [{"text":"impl&lt;N, R:&nbsp;DimName, C:&nbsp;DimName&gt; Sum&lt;Matrix&lt;N, R, C, &lt;DefaultAllocator as Allocator&lt;N, R, C&gt;&gt;::Buffer&gt;&gt; for MatrixMN&lt;N, R, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar + ClosedAdd + Zero,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, R, C&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, C:&nbsp;Dim&gt; Sum&lt;Matrix&lt;N, Dynamic, C, &lt;DefaultAllocator as Allocator&lt;N, Dynamic, C&gt;&gt;::Buffer&gt;&gt; for MatrixMN&lt;N, Dynamic, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar + ClosedAdd + Zero,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, Dynamic, C&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, N, R:&nbsp;DimName, C:&nbsp;DimName&gt; Sum&lt;&amp;'a Matrix&lt;N, R, C, &lt;DefaultAllocator as Allocator&lt;N, R, C&gt;&gt;::Buffer&gt;&gt; for MatrixMN&lt;N, R, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar + ClosedAdd + Zero,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, R, C&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, N, C:&nbsp;Dim&gt; Sum&lt;&amp;'a Matrix&lt;N, Dynamic, C, &lt;DefaultAllocator as Allocator&lt;N, Dynamic, C&gt;&gt;::Buffer&gt;&gt; for MatrixMN&lt;N, Dynamic, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar + ClosedAdd + Zero,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, Dynamic, C&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["noisy_float"] = [{"text":"impl&lt;F:&nbsp;Float, C:&nbsp;FloatChecker&lt;F&gt;&gt; Sum&lt;NoisyFloat&lt;F, C&gt;&gt; for NoisyFloat&lt;F, C&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, F:&nbsp;Float, C:&nbsp;FloatChecker&lt;F&gt;&gt; Sum&lt;&amp;'a NoisyFloat&lt;F, C&gt;&gt; for NoisyFloat&lt;F, C&gt;","synthetic":false,"types":[]}];
implementors["num_complex"] = [{"text":"impl&lt;T:&nbsp;Num + Clone&gt; Sum&lt;Complex&lt;T&gt;&gt; for Complex&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;'a + Num + Clone&gt; Sum&lt;&amp;'a Complex&lt;T&gt;&gt; for Complex&lt;T&gt;","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T:&nbsp;Integer + Clone&gt; Sum&lt;Ratio&lt;T&gt;&gt; for Ratio&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;Integer + Clone&gt; Sum&lt;&amp;'a Ratio&lt;T&gt;&gt; for Ratio&lt;T&gt;","synthetic":false,"types":[]}];
implementors["rgb"] = [{"text":"impl&lt;T&gt; Sum&lt;RGB&lt;T&gt;&gt; for RGB&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Default + Add&lt;Output = T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Sum&lt;Gray&lt;T&gt;&gt; for Gray&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Default + Add&lt;Output = T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, A&gt; Sum&lt;RGBA&lt;T, A&gt;&gt; for RGBA&lt;T, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Default + Add&lt;Output = T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Default + Add&lt;Output = A&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, A&gt; Sum&lt;GrayAlpha&lt;T, A&gt;&gt; for GrayAlpha&lt;T, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Default + Add&lt;Output = T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Default + Add&lt;Output = A&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["wide"] = [{"text":"impl Sum&lt;f32x4&gt; for f32x4","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sum&lt;&amp;'a f32x4&gt; for f32x4","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()