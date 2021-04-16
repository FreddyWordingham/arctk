initSidebarItems({"struct":[["Batching","A “meta iterator adaptor”. Its closure receives a reference to the iterator and may pick off as many elements as it likes, to produce the next iterator element."],["CircularTupleWindows","An iterator over all windows,wrapping back to the first elements when the window would otherwise exceed the length of the iterator, producing tuples of a specific size."],["ConsTuples","An iterator that maps an iterator of tuples like `((A, B), C)` to an iterator of `(A, B, C)`."],["ExactlyOneError","Iterator returned for the error case of `IterTools::exactly_one()` This iterator yields exactly the same elements as the input iterator."],["FilterMapOk","An iterator adapter to filter and apply a transformation on values within a nested `Result::Ok`."],["FilterOk","An iterator adapter to filter values within a nested `Result::Ok`."],["Format","Format all iterator elements lazily, separated by `sep`."],["FormatWith","Format all iterator elements lazily, separated by `sep`."],["Interleave","An iterator adaptor that alternates elements from two iterators until both run out."],["InterleaveShortest","An iterator adaptor that alternates elements from the two iterators until one of them runs out."],["IntersperseWith","An iterator adaptor to insert a particular value created by a function between each element of the adapted iterator."],["Iterate","An iterator that infinitely applies function to value and yields results."],["MergeBy","An iterator adaptor that merges the two base iterators in ascending order. If both base iterators are sorted (ascending), the result is sorted."],["MergeJoinBy","An iterator adaptor that merge-joins items from the two base iterators in ascending order."],["PadUsing","An iterator adaptor that pads a sequence to a minimum length by filling missing elements using a closure."],["PeekingTakeWhile","An iterator adaptor that takes items while a closure returns `true`."],["Positions","An iterator adapter to get the positions of each element that matches a predicate."],["ProcessResults","An iterator that produces only the `T` values as long as the inner iterator produces `Ok(T)`."],["Product","An iterator adaptor that iterates over the cartesian product of the element sets of two iterators `I` and `J`."],["PutBack","An iterator adaptor that allows putting back a single item to the front of the iterator."],["RepeatCall","See `repeat_call` for more information."],["RepeatN","An iterator that produces n repetitions of an element."],["Step","An iterator adaptor that steps a number elements in the base iterator for each iteration."],["TakeWhileRef","An iterator adaptor that borrows from a `Clone`-able iterator to only pick off elements while the predicate returns `true`."],["TupleBuffer","An iterator over a incomplete tuple."],["TupleCombinations","An iterator to iterate through all combinations in a `Clone`-able iterator that produces tuples of a specific size."],["TupleWindows","An iterator over all contiguous windows that produces tuples of a specific size."],["Tuples","An iterator that groups the items in tuples of a specific size."],["Unfold","See `unfold` for more information."],["Update","An iterator adapter to apply a mutating function to each element before yielding it."],["WhileSome","An iterator adaptor that filters `Option<A>` iterator elements and produces `A`. Stops on the first `None` encountered."],["WithPosition","An iterator adaptor that wraps each element in an `Position`."],["Zip","See `multizip` for more information."],["ZipEq","An iterator which iterates two other iterators simultaneously"],["ZipLongest","An iterator which iterates two other iterators simultaneously"]],"type":[["Coalesce","An iterator adaptor that may join together adjacent elements."],["Dedup","An iterator adaptor that removes repeated duplicates."],["DedupBy","An iterator adaptor that removes repeated duplicates, determining equality using a comparison function."],["DedupByWithCount","An iterator adaptor that removes repeated duplicates, while keeping a count of how many repeated elements were present. This will determine equality using a comparison function."],["DedupWithCount","An iterator adaptor that removes repeated duplicates, while keeping a count of how many repeated elements were present."],["Intersperse","An iterator adaptor to insert a particular value between each element of the adapted iterator."],["MapInto","An iterator adapter to apply `Into` conversion to each element."],["MapOk","An iterator adapter to apply a transformation within a nested `Result::Ok`."],["MapResults","See `MapOk`."],["Merge","An iterator adaptor that merges the two base iterators in ascending order. If both base iterators are sorted (ascending), the result is sorted."]]});