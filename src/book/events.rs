pub fn text() -> String {
r#####"
<h1 id="events">Events</h1>
<p>Events are created by passing <a href="https://docs.rs/seed/0.4.1/seed/dom_types/struct.Listener.html">Listener</a>s, or vecs of Listeners into <code>Node</code> macros. They’re created using the following functions exposed in the prelude: <code>simple_ev</code>, <code>input_ev</code>, <code>keyboard_ev</code>, <code>mouse_ev</code>, and <code>raw_ev</code>. The first is demonstrated in the example in the quickstart section, and all are demonstrated in the todomvc example.</p>
<p><code>simple_ev</code> takes two arguments: an event trigger, which can be a <code>Seed::Ev</code> (imported in the prelude), an <code>&amp;str</code>, or a <code>String</code>, (eg <code>Ev::Click</code>, “click”, “contextmenu” etc), and an instance of your <code>Msg</code> enum. (eg Msg::Increment). The other three event-creation-funcs take a trigger, and a <a href="https://doc.rust-lang.org/book/ch13-01-closures.html">closure</a> (An anonymous function, similar to an arrow func in JS) that returns a Msg enum.</p>
<p><code>simple_ev</code> does not pass any information about the event, only that it fired. Example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb1-1" title="1"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb1-2" title="2"><span class="kw">enum</span> Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb1-3" title="3">    ClickClick</a>
<a class="sourceLine" id="cb1-4" title="4"><span class="op">}</span></a>
<a class="sourceLine" id="cb1-5" title="5"><span class="co">// ...</span></a>
<a class="sourceLine" id="cb1-6" title="6">simple_ev(<span class="pp">Ev::</span>DblClick, <span class="pp">Msg::</span>ClickClick)`</a></code></pre></div>
<p><code>input_ev</code> passes the event target’s value field, eg what a user entered in an <code>input</code>, <code>textarea</code>, or <code>select</code>, field. Example:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb2-1" title="1"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb2-2" title="2"><span class="kw">enum</span> Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb2-3" title="3">    NewWords(<span class="dt">String</span>)</a>
<a class="sourceLine" id="cb2-4" title="4"><span class="op">}</span></a>
<a class="sourceLine" id="cb2-5" title="5"><span class="co">// ...</span></a>
<a class="sourceLine" id="cb2-6" title="6"><span class="co">// ...</span></a>
<a class="sourceLine" id="cb2-7" title="7">input_ev(<span class="pp">Ev::</span>Input, <span class="pp">Msg::</span>NewWords)</a></code></pre></div>
<p>Example <code>select</code> element:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb3-1" title="1"><span class="kw">enum</span> Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb3-2" title="2">    ChangeSelected(<span class="dt">String</span>)</a>
<a class="sourceLine" id="cb3-3" title="3"><span class="op">}</span></a>
<a class="sourceLine" id="cb3-4" title="4"><span class="co">// ... (in update)</span></a>
<a class="sourceLine" id="cb3-5" title="5">ChangeSelected(selected) =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb3-6" title="6">    <span class="pp">log!</span>(<span class="st">&quot;Value of the option selected: &quot;</span>, selected);</a>
<a class="sourceLine" id="cb3-7" title="7">    Render(Model<span class="op">{</span>selected, ..model<span class="op">}</span>)</a>
<a class="sourceLine" id="cb3-8" title="8"><span class="op">}</span></a>
<a class="sourceLine" id="cb3-9" title="9"></a>
<a class="sourceLine" id="cb3-10" title="10"><span class="co">// ...</span></a>
<a class="sourceLine" id="cb3-11" title="11"><span class="pp">select!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb3-12" title="12">    <span class="pp">attrs!</span><span class="op">{</span><span class="pp">At::</span>Value =&gt; <span class="st">&quot;0&quot;</span><span class="op">}</span>,</a>
<a class="sourceLine" id="cb3-13" title="13">    <span class="pp">option!</span><span class="op">[</span><span class="pp">attrs!</span><span class="op">{</span><span class="pp">At::</span>Value =&gt; <span class="st">&quot;0&quot;</span><span class="op">}</span>, <span class="st">&quot;Option A&quot;</span><span class="op">]</span>,</a>
<a class="sourceLine" id="cb3-14" title="14">    <span class="pp">option!</span><span class="op">[</span><span class="pp">attrs!</span><span class="op">{</span><span class="pp">At::</span>Value =&gt; <span class="st">&quot;1&quot;</span><span class="op">}</span>, <span class="st">&quot;Option B&quot;</span><span class="op">]</span>,</a>
<a class="sourceLine" id="cb3-15" title="15">    <span class="pp">option!</span><span class="op">[</span><span class="pp">attrs!</span><span class="op">{</span><span class="pp">At::</span>Value =&gt; <span class="st">&quot;2&quot;</span><span class="op">}</span>, <span class="st">&quot;Option C&quot;</span><span class="op">]</span>,</a>
<a class="sourceLine" id="cb3-16" title="16">    input_ev(<span class="pp">Ev::</span>Input, <span class="pp">Msg::</span>ChangeSelected)</a>
<a class="sourceLine" id="cb3-17" title="17"><span class="op">]</span></a></code></pre></div>
<p><code>keyboard_ev</code> returns a <a href="https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.KeyboardEvent.html">web_sys::KeyboardEvent</a>, which exposes several getter methods like <code>key_code</code> and <code>key</code>. <code>mouse_ev</code> works in a similar way. Example:</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb4-1" title="1"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb4-2" title="2"><span class="kw">enum</span> Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb4-3" title="3">    PutTheHammerDown(<span class="pp">web_sys::</span>KeyboardEvent)</a>
<a class="sourceLine" id="cb4-4" title="4"><span class="op">}</span></a>
<a class="sourceLine" id="cb4-5" title="5"><span class="co">// ...</span></a>
<a class="sourceLine" id="cb4-6" title="6">keyboard_ev(<span class="st">&quot;input&quot;</span>, <span class="pp">Msg::</span>PutTheHammerDown)</a></code></pre></div>
<p>Note that in the examples for input_ev and keyboard_ev, the syntax is simplified since we’re only passing the field text, and keyboard event respectively to the Msg. The input_ev example is Rust shorthand for <code>input_ev(&quot;input, |text| Msg::NewWords(text)</code>. If you were to pass something other than, or more than just the input text (Or KeyboardEvent for keyboard_ev, or Event for raw_ev described below), you can’t use this shorthand, and would have to do something like this intead, explicitly writing the closure:</p>
<div class="sourceCode" id="cb5"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb5-1" title="1"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb5-2" title="2"><span class="kw">enum</span> Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb5-3" title="3">    NewWords(<span class="dt">String</span>, <span class="dt">u32</span>)</a>
<a class="sourceLine" id="cb5-4" title="4"><span class="op">}</span></a>
<a class="sourceLine" id="cb5-5" title="5"><span class="co">// ...</span></a>
<a class="sourceLine" id="cb5-6" title="6">input_ev(<span class="st">&quot;input&quot;</span>, <span class="kw">move</span> |text| <span class="pp">Msg::</span>NewWords(text, <span class="dv">0</span>))</a></code></pre></div>
<p><code>raw_ev</code> returns a <a href="https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Event.html">web_sys::Event</a>. It lets you access any part of any type of event, albeit with more verbose syntax. If you wish to do something like <code>prevent_default()</code>, or anything not listed above, you may need to take this approach. Note that for many common operations, like taking the value of an input element after an <code>input</code> or <code>change</code> event, you have to deal with casting from a generic event or target to the specific one. Seed provides convenience functions to handle this. They wrap wasm-bindgen’s <code>.dyn_ref()</code>, from its <a href="https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html">JsCast</a> trait.</p>
<p>Example syntax showing how you might use raw_ev; processing an input and handling a keyboard event, while using prevent_default:</p>
<div class="sourceCode" id="cb6"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb6-1" title="1"><span class="co">// (in update func)</span></a>
<a class="sourceLine" id="cb6-2" title="2"><span class="pp">Msg::</span>KeyPress(event) =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb6-3" title="3">    event.prevent_default();</a>
<a class="sourceLine" id="cb6-4" title="4">    <span class="kw">let</span> code = <span class="pp">seed::</span>to_kbevent(&amp;ev).key_code();</a>
<a class="sourceLine" id="cb6-5" title="5">    <span class="co">// ..</span></a>
<a class="sourceLine" id="cb6-6" title="6">    <span class="kw">let</span> target = event.target().unwrap();</a>
<a class="sourceLine" id="cb6-7" title="7">    <span class="kw">let</span> text = <span class="pp">seed::</span>to_input(&amp;target).value();</a>
<a class="sourceLine" id="cb6-8" title="8">    </a>
<a class="sourceLine" id="cb6-9" title="9">    <span class="co">// ...</span></a>
<a class="sourceLine" id="cb6-10" title="10">    <span class="co">// In view</span></a>
<a class="sourceLine" id="cb6-11" title="11">    raw_ev(<span class="pp">Ev::</span>Input, <span class="pp">Msg::</span>KeyPress),</a>
<a class="sourceLine" id="cb6-12" title="12"><span class="op">}</span></a></code></pre></div>
<p>Seed also provides <code>to_textarea</code> and <code>to_select</code> functions, which you’d use as <code>to_input</code>. It provides <code>to_html_el</code>, which is useful for changing settings like <code>focus</code>, and <code>to_mouse_event</code>, which you’d use like <code>to_kbevent</code>.</p>
<p>This extra step is caused by a conflict between Rust’s type system, and the way DOM events are handled. For example, you may wish to pull text from an input field by reading the event target’s <code>value</code> field. However, not all targets contain value; it may have to be represented as an <code>HtmlInputElement</code>. (See <a href="https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.EventTarget.html">the web-sys ref</a>, and <a href="https://developer.mozilla.org/en-US/docs/Web/API/EventTarget">Mdn ref</a>; there’s no value field)) Another example: If we wish to read the key_code of an event, we must first cast it as a <code>KeyboardEvent</code>; pure Events (<code>web_sys</code> and DOM) do not contain this field.</p>
<p>It’s likely you’ll be able to do most of what you wish with the simpler event funcs. If there’s a type of event or use you think would benefit from a similar func, submit an issue or PR. In the descriptions above for all event-creation funcs, we assumed minimal code in the closure, and more code in the update func’s match arms. For example, to process a keyboard event, these two approaches are equivalent:</p>
<div class="sourceCode" id="cb7"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb7-1" title="1"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb7-2" title="2"><span class="kw">enum</span> Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb7-3" title="3">    KeyDown(<span class="pp">web_sys::</span>KeyboardEvent)</a>
<a class="sourceLine" id="cb7-4" title="4"><span class="op">}</span></a>
<a class="sourceLine" id="cb7-5" title="5"></a>
<a class="sourceLine" id="cb7-6" title="6"><span class="co">// ... (in update)</span></a>
<a class="sourceLine" id="cb7-7" title="7">KeyDown(event) =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb7-8" title="8">    <span class="kw">let</span> code = event.key_code()</a>
<a class="sourceLine" id="cb7-9" title="9">    <span class="co">// ...</span></a>
<a class="sourceLine" id="cb7-10" title="10"><span class="op">}</span></a>
<a class="sourceLine" id="cb7-11" title="11"></a>
<a class="sourceLine" id="cb7-12" title="12"><span class="co">// ... In view</span></a>
<a class="sourceLine" id="cb7-13" title="13">keyboard_ev(<span class="st">&quot;keydown&quot;</span>, <span class="pp">Msg::</span>KeyDown)</a></code></pre></div>
<p>and</p>
<div class="sourceCode" id="cb8"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb8-1" title="1"><span class="kw">enum</span> Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb8-2" title="2">    KeyDown(<span class="dt">u32</span>)</a>
<a class="sourceLine" id="cb8-3" title="3"><span class="op">}</span></a>
<a class="sourceLine" id="cb8-4" title="4"></a>
<a class="sourceLine" id="cb8-5" title="5"><span class="co">// ... (in update)</span></a>
<a class="sourceLine" id="cb8-6" title="6">KeyDown(code) =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb8-7" title="7">    <span class="co">// ...</span></a>
<a class="sourceLine" id="cb8-8" title="8"><span class="op">}</span></a>
<a class="sourceLine" id="cb8-9" title="9"></a>
<a class="sourceLine" id="cb8-10" title="10"><span class="co">// ... In view</span></a>
<a class="sourceLine" id="cb8-11" title="11">keyboard_ev(<span class="st">&quot;keydown&quot;</span>, |ev| KeyDown(ev.key_code()))</a></code></pre></div>
<p>You can pass more than one variable to the <code>Msg</code> enum via the closure, as long as it’s set up appropriate in <code>Msg</code>’s definition. Note that if you pass a value to the enum other than what’s between ||, you may receive an error about lifetimes. This is corrected by making the closure a move type. Eg:</p>
<div class="sourceCode" id="cb9"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb9-1" title="1">keyboard_ev(<span class="pp">Ev::</span>KeyDown, <span class="kw">move</span> |ev| <span class="pp">Msg::</span>EditKeyDown(id, ev.key_code()))</a></code></pre></div>
<p>Where <code>id</code> is a value defined earlier.</p>
<p>Event syntax may be improved later with the addition of a single macro that infers what the type of event is based on the trigger, and avoids the use of manually creating a <code>Vec</code> to store the <code>Listener</code>s. For examples of all of the above (except raw_ev), check out the <a href="https://github.com/David-OConnor/seed/tree/master/examples/todomvc">todomvc example</a>.</p>
<p>The <a href="https://github.com/David-OConnor/seed/tree/master/examples/todomvc">todomvc example</a> has a number of event-handling examples, including use of raw_ev, where it handles text input triggered by a key press, and uses prevent_default().</p>
<h2 id="window-events">Window events</h2>
<p>We handle events triggered by the overall window specially, since it doesn’t fit directly into our virtual DOM. We pass to <code>Seed::App::build::window_events()</code> a function that accepts a ref to <code>Model</code>, and returns a <code>Vec&lt;devents::Listener&gt;</code>. We use it to control which listeners are attached to the window based on the model. Excerpt from the <a href="https://github.com/David-OConnor/seed/blob/master/examples/window_events/src/lib.rs">window_events</a> example:</p>
<div class="sourceCode" id="cb10"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb10-1" title="1"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb10-2" title="2"><span class="kw">enum</span> Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb10-3" title="3">    ToggleWatching,</a>
<a class="sourceLine" id="cb10-4" title="4">    UpdateCoords(<span class="pp">web_sys::</span>MouseEvent),</a>
<a class="sourceLine" id="cb10-5" title="5">    KeyPressed(<span class="pp">web_sys::</span>KeyboardEvent),</a>
<a class="sourceLine" id="cb10-6" title="6"><span class="op">}</span></a>
<a class="sourceLine" id="cb10-7" title="7"></a>
<a class="sourceLine" id="cb10-8" title="8"><span class="kw">fn</span> update(msg: Msg, model: &amp;<span class="kw">mut</span> Model, _: &amp;<span class="kw">mut</span> Orders&lt;Msg&gt;) <span class="op">{</span></a>
<a class="sourceLine" id="cb10-9" title="9">    <span class="kw">match</span> msg <span class="op">{</span></a>
<a class="sourceLine" id="cb10-10" title="10">        <span class="pp">Msg::</span>ToggleWatching =&gt; model.watching = !model.watching,</a>
<a class="sourceLine" id="cb10-11" title="11">        <span class="pp">Msg::</span>UpdateCoords(ev) =&gt; model.coords = (ev.screen_x(), ev.screen_y()),</a>
<a class="sourceLine" id="cb10-12" title="12">        <span class="pp">Msg::</span>KeyPressed(ev) =&gt; model.last_keycode = ev.key_code(),</a>
<a class="sourceLine" id="cb10-13" title="13">    <span class="op">}</span></a>
<a class="sourceLine" id="cb10-14" title="14"><span class="op">}</span></a>
<a class="sourceLine" id="cb10-15" title="15"></a>
<a class="sourceLine" id="cb10-16" title="16"><span class="co">// ...</span></a>
<a class="sourceLine" id="cb10-17" title="17"></a>
<a class="sourceLine" id="cb10-18" title="18"><span class="kw">fn</span> window_events(model: &amp;Model) -&gt; <span class="dt">Vec</span>&lt;<span class="pp">seed::events::</span>Listener&lt;Msg&gt;&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb10-19" title="19">    <span class="kw">let</span> <span class="kw">mut</span> result = <span class="dt">Vec</span>::new();</a>
<a class="sourceLine" id="cb10-20" title="20">    <span class="kw">if</span> model.watching <span class="op">{</span></a>
<a class="sourceLine" id="cb10-21" title="21">        result.push(mouse_ev(<span class="st">&quot;mousemove&quot;</span>, |ev| <span class="pp">Msg::</span>UpdateCoords(ev)));</a>
<a class="sourceLine" id="cb10-22" title="22">        result.push(keyboard_ev(<span class="st">&quot;keydown&quot;</span>, |ev| <span class="pp">Msg::</span>KeyPressed(ev)));</a>
<a class="sourceLine" id="cb10-23" title="23">    <span class="op">}</span></a>
<a class="sourceLine" id="cb10-24" title="24">    result</a>
<a class="sourceLine" id="cb10-25" title="25"><span class="op">}</span></a>
<a class="sourceLine" id="cb10-26" title="26"></a>
<a class="sourceLine" id="cb10-27" title="27"></a>
<a class="sourceLine" id="cb10-28" title="28"><span class="at">#[</span>wasm_bindgen<span class="at">]</span></a>
<a class="sourceLine" id="cb10-29" title="29"><span class="kw">pub</span> <span class="kw">fn</span> render() <span class="op">{</span></a>
<a class="sourceLine" id="cb10-30" title="30">    <span class="pp">seed::App::</span>build(<span class="pp">Init::</span>new(<span class="pp">Model::</span><span class="kw">default</span>()), update, view)</a>
<a class="sourceLine" id="cb10-31" title="31">        .window_events(window_events)</a>
<a class="sourceLine" id="cb10-32" title="32">        .finish()</a>
<a class="sourceLine" id="cb10-33" title="33">        .run();</a>
<a class="sourceLine" id="cb10-34" title="34"><span class="op">}</span></a></code></pre></div>
<p>If <code>model.watching</code> is <code>true</code>, the window listens for keyboard and mouse events, then updates the model accordingly. If not, it doesn’t listen.</p>
"#####.into()
}