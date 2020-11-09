(function() {var implementors = {};
implementors["nalgebra"] = [{"text":"impl&lt;N, R:&nbsp;Dim, C:&nbsp;Dim, S&gt; Neg for Matrix&lt;N, R, C, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar + ClosedNeg,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Storage&lt;N, R, C&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, R, C&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, N, R:&nbsp;Dim, C:&nbsp;Dim, S&gt; Neg for &amp;'a Matrix&lt;N, R, C, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar + ClosedNeg,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Storage&lt;N, R, C&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, R, C&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar + ClosedNeg, R:&nbsp;Dim, C:&nbsp;Dim&gt; Neg for Unit&lt;MatrixMN&lt;N, R, C&gt;&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, R, C&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar + ClosedNeg, D:&nbsp;DimName&gt; Neg for Point&lt;N, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, N:&nbsp;Scalar + ClosedNeg, D:&nbsp;DimName&gt; Neg for &amp;'a Point&lt;N, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;SimdRealField&gt; Neg for Quaternion&lt;N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N::Element: SimdRealField,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, N:&nbsp;SimdRealField&gt; Neg for &amp;'a Quaternion&lt;N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N::Element: SimdRealField,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["ndarray"] = [{"text":"impl&lt;A, S, D&gt; Neg for ArrayBase&lt;S, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Clone + Neg&lt;Output = A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: DataOwned&lt;Elem = A&gt; + DataMut,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Dimension,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, A, S, D&gt; Neg for &amp;'a ArrayBase&lt;S, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&amp;'a A: 'a + Neg&lt;Output = A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Data&lt;Elem = A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Dimension,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["noisy_float"] = [{"text":"impl&lt;F:&nbsp;Float, C:&nbsp;FloatChecker&lt;F&gt;&gt; Neg for NoisyFloat&lt;F, C&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, F:&nbsp;Float, C:&nbsp;FloatChecker&lt;F&gt;&gt; Neg for &amp;'a NoisyFloat&lt;F, C&gt;","synthetic":false,"types":[]}];
implementors["num_complex"] = [{"text":"impl&lt;T:&nbsp;Clone + Num + Neg&lt;Output = T&gt;&gt; Neg for Complex&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;Clone + Num + Neg&lt;Output = T&gt;&gt; Neg for &amp;'a Complex&lt;T&gt;","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T&gt; Neg for Ratio&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Clone + Integer + Neg&lt;Output = T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Neg for &amp;'a Ratio&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Clone + Integer + Neg&lt;Output = T&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["simba"] = [{"text":"impl Neg for AutoSimd&lt;[f32; 2]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[f32; 4]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[f32; 8]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[f32; 16]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[f64; 2]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[f64; 4]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[f64; 8]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[i128; 1]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[i128; 2]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[i128; 4]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[i16; 2]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[i16; 4]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[i16; 8]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[i16; 16]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[i16; 32]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[i32; 2]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[i32; 4]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[i32; 8]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[i32; 16]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[i64; 2]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[i64; 4]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[i64; 8]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[i8; 2]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[i8; 4]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[i8; 8]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[i8; 16]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[i8; 32]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[isize; 2]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[isize; 4]&gt;","synthetic":false,"types":[]},{"text":"impl Neg for AutoSimd&lt;[isize; 8]&gt;","synthetic":false,"types":[]}];
implementors["typenum"] = [{"text":"impl Neg for Z0","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;Unsigned + NonZero&gt; Neg for PInt&lt;U&gt;","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;Unsigned + NonZero&gt; Neg for NInt&lt;U&gt;","synthetic":false,"types":[]},{"text":"impl Neg for ATerm","synthetic":false,"types":[]},{"text":"impl&lt;V, A&gt; Neg for TArr&lt;V, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;V: Neg,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Neg,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()