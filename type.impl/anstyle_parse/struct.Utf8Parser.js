(function() {var type_impls = {
"anstyle_parse":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-Utf8Parser\" class=\"impl\"><a class=\"src rightside\" href=\"src/anstyle_parse/lib.rs.html#342\">source</a><a href=\"#impl-Clone-for-Utf8Parser\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"anstyle_parse/struct.Utf8Parser.html\" title=\"struct anstyle_parse::Utf8Parser\">Utf8Parser</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/anstyle_parse/lib.rs.html#342\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"anstyle_parse/struct.Utf8Parser.html\" title=\"struct anstyle_parse::Utf8Parser\">Utf8Parser</a></h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.2/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/core/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","anstyle_parse::DefaultCharAccumulator"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Utf8Parser\" class=\"impl\"><a class=\"src rightside\" href=\"src/anstyle_parse/lib.rs.html#342\">source</a><a href=\"#impl-Debug-for-Utf8Parser\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"anstyle_parse/struct.Utf8Parser.html\" title=\"struct anstyle_parse::Utf8Parser\">Utf8Parser</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/anstyle_parse/lib.rs.html#342\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","anstyle_parse::DefaultCharAccumulator"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-CharAccumulator-for-Utf8Parser\" class=\"impl\"><a class=\"src rightside\" href=\"src/anstyle_parse/lib.rs.html#348-355\">source</a><a href=\"#impl-CharAccumulator-for-Utf8Parser\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"anstyle_parse/trait.CharAccumulator.html\" title=\"trait anstyle_parse::CharAccumulator\">CharAccumulator</a> for <a class=\"struct\" href=\"anstyle_parse/struct.Utf8Parser.html\" title=\"struct anstyle_parse::Utf8Parser\">Utf8Parser</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.add\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/anstyle_parse/lib.rs.html#349-354\">source</a><a href=\"#method.add\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"anstyle_parse/trait.CharAccumulator.html#tymethod.add\" class=\"fn\">add</a>(&amp;mut self, byte: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/core/primitive.u8.html\">u8</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.2/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/core/primitive.char.html\">char</a>&gt;</h4></section></summary><div class='docblock'>Build a <code>char</code> out of bytes <a href=\"anstyle_parse/trait.CharAccumulator.html#tymethod.add\">Read more</a></div></details></div></details>","CharAccumulator","anstyle_parse::DefaultCharAccumulator"],["<section id=\"impl-StructuralPartialEq-for-Utf8Parser\" class=\"impl\"><a class=\"src rightside\" href=\"src/anstyle_parse/lib.rs.html#342\">source</a><a href=\"#impl-StructuralPartialEq-for-Utf8Parser\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for <a class=\"struct\" href=\"anstyle_parse/struct.Utf8Parser.html\" title=\"struct anstyle_parse::Utf8Parser\">Utf8Parser</a></h3></section>","StructuralPartialEq","anstyle_parse::DefaultCharAccumulator"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-Utf8Parser\" class=\"impl\"><a class=\"src rightside\" href=\"src/anstyle_parse/lib.rs.html#342\">source</a><a href=\"#impl-Default-for-Utf8Parser\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"anstyle_parse/struct.Utf8Parser.html\" title=\"struct anstyle_parse::Utf8Parser\">Utf8Parser</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/anstyle_parse/lib.rs.html#342\">source</a><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; <a class=\"struct\" href=\"anstyle_parse/struct.Utf8Parser.html\" title=\"struct anstyle_parse::Utf8Parser\">Utf8Parser</a></h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"https://doc.rust-lang.org/1.77.2/core/default/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","anstyle_parse::DefaultCharAccumulator"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-Utf8Parser\" class=\"impl\"><a class=\"src rightside\" href=\"src/anstyle_parse/lib.rs.html#342\">source</a><a href=\"#impl-PartialEq-for-Utf8Parser\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for <a class=\"struct\" href=\"anstyle_parse/struct.Utf8Parser.html\" title=\"struct anstyle_parse::Utf8Parser\">Utf8Parser</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/anstyle_parse/lib.rs.html#342\">source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"struct\" href=\"anstyle_parse/struct.Utf8Parser.html\" title=\"struct anstyle_parse::Utf8Parser\">Utf8Parser</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/core/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>self</code> and <code>other</code> values to be equal, and is used\nby <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.2/src/core/cmp.rs.html#242\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/core/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/core/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","anstyle_parse::DefaultCharAccumulator"],["<section id=\"impl-Eq-for-Utf8Parser\" class=\"impl\"><a class=\"src rightside\" href=\"src/anstyle_parse/lib.rs.html#342\">source</a><a href=\"#impl-Eq-for-Utf8Parser\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for <a class=\"struct\" href=\"anstyle_parse/struct.Utf8Parser.html\" title=\"struct anstyle_parse::Utf8Parser\">Utf8Parser</a></h3></section>","Eq","anstyle_parse::DefaultCharAccumulator"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()