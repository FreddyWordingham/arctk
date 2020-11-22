(function() {var implementors = {};
implementors["arrayvec"] = [{"text":"impl&lt;A&gt; AsRef&lt;str&gt; for ArrayString&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Array&lt;Item = u8&gt; + Copy,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; AsRef&lt;[&lt;A as Array&gt;::Item]&gt; for ArrayVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["core_graphics"] = [{"text":"impl AsRef&lt;CGColorSpaceRef&gt; for CGColorSpace","synthetic":false,"types":[]},{"text":"impl AsRef&lt;CGContextRef&gt; for CGContext","synthetic":false,"types":[]},{"text":"impl AsRef&lt;CGDataProviderRef&gt; for CGDataProvider","synthetic":false,"types":[]},{"text":"impl AsRef&lt;CGDisplayModeRef&gt; for CGDisplayMode","synthetic":false,"types":[]},{"text":"impl AsRef&lt;CGEventRef&gt; for CGEvent","synthetic":false,"types":[]},{"text":"impl AsRef&lt;CGEventSourceRef&gt; for CGEventSource","synthetic":false,"types":[]},{"text":"impl AsRef&lt;CGFontRef&gt; for CGFont","synthetic":false,"types":[]},{"text":"impl AsRef&lt;CGGradientRef&gt; for CGGradient","synthetic":false,"types":[]},{"text":"impl AsRef&lt;CGImageRef&gt; for CGImage","synthetic":false,"types":[]},{"text":"impl AsRef&lt;CGPathRef&gt; for CGPath","synthetic":false,"types":[]}];
implementors["crossbeam_epoch"] = [{"text":"impl&lt;T:&nbsp;?Sized + Pointable&gt; AsRef&lt;T&gt; for Owned&lt;T&gt;","synthetic":false,"types":[]}];
implementors["either"] = [{"text":"impl&lt;L, R, Target&gt; AsRef&lt;Target&gt; for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: AsRef&lt;Target&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: AsRef&lt;Target&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;L, R&gt; AsRef&lt;str&gt; for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: AsRef&lt;str&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: AsRef&lt;str&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;L, R, Target&gt; AsRef&lt;[Target]&gt; for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: AsRef&lt;[Target]&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: AsRef&lt;[Target]&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["generic_array"] = [{"text":"impl&lt;T, N&gt; AsRef&lt;[T]&gt; for GenericArray&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: ArrayLength&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 1]&gt; for GenericArray&lt;T, U1&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 2]&gt; for GenericArray&lt;T, U2&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 3]&gt; for GenericArray&lt;T, U3&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 4]&gt; for GenericArray&lt;T, U4&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 5]&gt; for GenericArray&lt;T, U5&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 6]&gt; for GenericArray&lt;T, U6&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 7]&gt; for GenericArray&lt;T, U7&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 8]&gt; for GenericArray&lt;T, U8&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 9]&gt; for GenericArray&lt;T, U9&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 10]&gt; for GenericArray&lt;T, U10&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 11]&gt; for GenericArray&lt;T, U11&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 12]&gt; for GenericArray&lt;T, U12&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 13]&gt; for GenericArray&lt;T, U13&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 14]&gt; for GenericArray&lt;T, U14&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 15]&gt; for GenericArray&lt;T, U15&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 16]&gt; for GenericArray&lt;T, U16&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 17]&gt; for GenericArray&lt;T, U17&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 18]&gt; for GenericArray&lt;T, U18&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 19]&gt; for GenericArray&lt;T, U19&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 20]&gt; for GenericArray&lt;T, U20&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 21]&gt; for GenericArray&lt;T, U21&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 22]&gt; for GenericArray&lt;T, U22&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 23]&gt; for GenericArray&lt;T, U23&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 24]&gt; for GenericArray&lt;T, U24&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 25]&gt; for GenericArray&lt;T, U25&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 26]&gt; for GenericArray&lt;T, U26&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 27]&gt; for GenericArray&lt;T, U27&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 28]&gt; for GenericArray&lt;T, U28&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 29]&gt; for GenericArray&lt;T, U29&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 30]&gt; for GenericArray&lt;T, U30&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 31]&gt; for GenericArray&lt;T, U31&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 32]&gt; for GenericArray&lt;T, U32&gt;","synthetic":false,"types":[]}];
implementors["nalgebra"] = [{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 1]&gt; for Matrix&lt;N, U1, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 2]&gt; for Matrix&lt;N, U1, U2, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U2&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 3]&gt; for Matrix&lt;N, U1, U3, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U3&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 4]&gt; for Matrix&lt;N, U1, U4, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U4&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 5]&gt; for Matrix&lt;N, U1, U5, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U5&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 6]&gt; for Matrix&lt;N, U1, U6, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U6&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 7]&gt; for Matrix&lt;N, U1, U7, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U7&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 8]&gt; for Matrix&lt;N, U1, U8, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U8&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 9]&gt; for Matrix&lt;N, U1, U9, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U9&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 10]&gt; for Matrix&lt;N, U1, U10, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U10&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 11]&gt; for Matrix&lt;N, U1, U11, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U11&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 12]&gt; for Matrix&lt;N, U1, U12, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U12&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 13]&gt; for Matrix&lt;N, U1, U13, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U13&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 14]&gt; for Matrix&lt;N, U1, U14, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U14&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 15]&gt; for Matrix&lt;N, U1, U15, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U15&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 16]&gt; for Matrix&lt;N, U1, U16, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U16&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 2]&gt; for Matrix&lt;N, U2, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U2, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 3]&gt; for Matrix&lt;N, U3, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U3, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 4]&gt; for Matrix&lt;N, U4, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U4, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 5]&gt; for Matrix&lt;N, U5, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U5, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 6]&gt; for Matrix&lt;N, U6, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U6, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 7]&gt; for Matrix&lt;N, U7, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U7, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 8]&gt; for Matrix&lt;N, U8, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U8, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 9]&gt; for Matrix&lt;N, U9, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U9, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 10]&gt; for Matrix&lt;N, U10, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U10, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 11]&gt; for Matrix&lt;N, U11, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U11, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 12]&gt; for Matrix&lt;N, U12, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U12, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 13]&gt; for Matrix&lt;N, U13, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U13, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 14]&gt; for Matrix&lt;N, U14, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U14, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 15]&gt; for Matrix&lt;N, U15, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U15, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; AsRef&lt;[N; 16]&gt; for Matrix&lt;N, U16, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U16, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 2]; 2]&gt; for Matrix&lt;N, U2, U2, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U2, U2&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 2]; 3]&gt; for Matrix&lt;N, U2, U3, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U2, U3&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 2]; 4]&gt; for Matrix&lt;N, U2, U4, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U2, U4&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 2]; 5]&gt; for Matrix&lt;N, U2, U5, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U2, U5&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 2]; 6]&gt; for Matrix&lt;N, U2, U6, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U2, U6&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 3]; 2]&gt; for Matrix&lt;N, U3, U2, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U3, U2&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 3]; 3]&gt; for Matrix&lt;N, U3, U3, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U3, U3&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 3]; 4]&gt; for Matrix&lt;N, U3, U4, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U3, U4&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 3]; 5]&gt; for Matrix&lt;N, U3, U5, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U3, U5&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 3]; 6]&gt; for Matrix&lt;N, U3, U6, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U3, U6&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 4]; 2]&gt; for Matrix&lt;N, U4, U2, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U4, U2&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 4]; 3]&gt; for Matrix&lt;N, U4, U3, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U4, U3&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 4]; 4]&gt; for Matrix&lt;N, U4, U4, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U4, U4&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 4]; 5]&gt; for Matrix&lt;N, U4, U5, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U4, U5&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 4]; 6]&gt; for Matrix&lt;N, U4, U6, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U4, U6&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 5]; 2]&gt; for Matrix&lt;N, U5, U2, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U5, U2&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 5]; 3]&gt; for Matrix&lt;N, U5, U3, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U5, U3&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 5]; 4]&gt; for Matrix&lt;N, U5, U4, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U5, U4&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 5]; 5]&gt; for Matrix&lt;N, U5, U5, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U5, U5&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 5]; 6]&gt; for Matrix&lt;N, U5, U6, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U5, U6&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 6]; 2]&gt; for Matrix&lt;N, U6, U2, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U6, U2&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 6]; 3]&gt; for Matrix&lt;N, U6, U3, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U6, U3&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 6]; 4]&gt; for Matrix&lt;N, U6, U4, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U6, U4&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 6]; 5]&gt; for Matrix&lt;N, U6, U5, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U6, U5&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; AsRef&lt;[[N; 6]; 6]&gt; for Matrix&lt;N, U6, U6, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U6, U6&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;T&gt; for Unit&lt;T&gt;","synthetic":false,"types":[]}];
implementors["ndarray"] = [{"text":"impl&lt;T, D&gt; AsRef&lt;[SliceOrIndex]&gt; for SliceInfo&lt;T, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: AsRef&lt;[SliceOrIndex]&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Dimension,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, D&gt; AsRef&lt;SliceInfo&lt;[SliceOrIndex], D&gt;&gt; for SliceInfo&lt;T, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: AsRef&lt;[SliceOrIndex]&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Dimension,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["noisy_float"] = [{"text":"impl&lt;F:&nbsp;Float, C:&nbsp;FloatChecker&lt;F&gt;&gt; AsRef&lt;F&gt; for NoisyFloat&lt;F, C&gt;","synthetic":false,"types":[]}];
implementors["palette"] = [{"text":"impl&lt;C, T, P:&nbsp;?Sized&gt; AsRef&lt;P&gt; for PreAlpha&lt;C, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Pixel&lt;T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Float,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: RawPixel&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;C, T, P:&nbsp;?Sized&gt; AsRef&lt;P&gt; for Alpha&lt;C, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Pixel&lt;T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: RawPixel&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S, T, P:&nbsp;?Sized&gt; AsRef&lt;P&gt; for Hsl&lt;S, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Component + Float,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: RgbSpace,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: RawPixel&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S, T, P:&nbsp;?Sized&gt; AsRef&lt;P&gt; for Hsv&lt;S, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Component + Float,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: RgbSpace,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: RawPixel&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S, T, P:&nbsp;?Sized&gt; AsRef&lt;P&gt; for Hwb&lt;S, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Component + Float,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: RgbSpace,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: RawPixel&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Wp, T, P:&nbsp;?Sized&gt; AsRef&lt;P&gt; for Lab&lt;Wp, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Component + Float,<br>&nbsp;&nbsp;&nbsp;&nbsp;Wp: WhitePoint,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: RawPixel&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Wp, T, P:&nbsp;?Sized&gt; AsRef&lt;P&gt; for Lch&lt;Wp, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Component + Float,<br>&nbsp;&nbsp;&nbsp;&nbsp;Wp: WhitePoint,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: RawPixel&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S, T, P:&nbsp;?Sized&gt; AsRef&lt;P&gt; for Luma&lt;S, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Component,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: LumaStandard,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: RawPixel&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S, T, P:&nbsp;?Sized&gt; AsRef&lt;P&gt; for Rgb&lt;S, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Component,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: RgbStandard,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: RawPixel&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Wp, T, P:&nbsp;?Sized&gt; AsRef&lt;P&gt; for Xyz&lt;Wp, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Component + Float,<br>&nbsp;&nbsp;&nbsp;&nbsp;Wp: WhitePoint,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: RawPixel&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Wp, T, P:&nbsp;?Sized&gt; AsRef&lt;P&gt; for Yxy&lt;Wp, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Component + Float,<br>&nbsp;&nbsp;&nbsp;&nbsp;Wp: WhitePoint,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: RawPixel&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["regex_syntax"] = [{"text":"impl AsRef&lt;[u8]&gt; for Literal","synthetic":false,"types":[]}];
implementors["rgb"] = [{"text":"impl&lt;T&gt; AsRef&lt;T&gt; for Gray&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T]&gt; for RGB&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T]&gt; for RGBA&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;T&gt; for GrayAlpha&lt;T&gt;","synthetic":false,"types":[]}];
implementors["smallvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; AsRef&lt;[&lt;A as Array&gt;::Item]&gt; for SmallVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["specs"] = [{"text":"impl AsRef&lt;dyn Error + 'static&gt; for BoxedErr","synthetic":false,"types":[]}];
implementors["wide"] = [{"text":"impl AsRef&lt;[f32; 4]&gt; for f32x4","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()