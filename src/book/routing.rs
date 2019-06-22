pub fn text() -> String {
r#"
<h1 id="routing">Routing</h1>
<p>Seed includes flexible routing, inspired by <a href="https://github.com/reasonml/reason-react/blob/master/docs/router.md">React-Reason</a>: You can trigger state changes that update the address bar, and can be nagivated to/from using forward and back buttons. This works for landing-page routing as well, provided your server is configured to support. See the <a href="https://github.com/David-OConnor/seed/tree/master/examples/homepage">homepage</a> and <a href="https://github.com/David-OConnor/seed/tree/master/examples/todomvc">todomvc</a> examples.</p>
<p>Let's say our site the following pages: a guide, which can have subpages, and a changelog, accessible by <code>http://seed-rs.org/changelog</code>, <code>http://seed-rs.org/guide</code>, and <code>http://seed-rs.org/guide/3</code> (where 3 is the page we want) respectively. We describe the page by a <code>page</code> field in our model, which is an integer: 0 for guide, 1 for changelog, and an additional number for the guide page. An enum would be cleaner, but we don't wish to complicate this example.</p>
<h2 id="the-basics">The basics</h2>
<p>To set up the initial routing, pass a <code>routes</code> function describing how to handle routing, to <a href="https://docs.rs/seed/0.3.7/seed/struct.App.html#method.build">App::build</a>'s <code>routes</code> method.</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb1-1"><a href="#cb1-1"></a><span class="kw">fn</span> routes(url: &amp;<span class="pp">seed::</span>Url) -&gt; Msg <span class="op">{</span></span>
<span id="cb1-2"><a href="#cb1-2"></a>    <span class="kw">if</span> url.path.is_empty() <span class="op">{</span></span>
<span id="cb1-3"><a href="#cb1-3"></a>        <span class="kw">return</span> <span class="pp">Msg::</span>ChangePage(<span class="dv">0</span>)</span>
<span id="cb1-4"><a href="#cb1-4"></a>    <span class="op">}</span></span>
<span id="cb1-5"><a href="#cb1-5"></a></span>
<span id="cb1-6"><a href="#cb1-6"></a>    <span class="kw">match</span> url.path<span class="op">[</span><span class="dv">0</span><span class="op">]</span>.as_ref() <span class="op">{</span></span>
<span id="cb1-7"><a href="#cb1-7"></a>        <span class="st">&quot;guide&quot;</span> =&gt; <span class="op">{</span></span>
<span id="cb1-8"><a href="#cb1-8"></a>            <span class="co">// Determine if we&#39;re at the main guide page, or a subpage</span></span>
<span id="cb1-9"><a href="#cb1-9"></a>            <span class="kw">match</span> url.path.get(<span class="dv">1</span>).as_ref() <span class="op">{</span></span>
<span id="cb1-10"><a href="#cb1-10"></a>                <span class="cn">Some</span>(page) =&gt; <span class="pp">Msg::</span>ChangeGuidePage(page.<span class="pp">parse::</span>&lt;<span class="dt">usize</span>&gt;().unwrap()),</span>
<span id="cb1-11"><a href="#cb1-11"></a>                <span class="cn">None</span> =&gt; <span class="pp">Msg::</span>ChangePage(<span class="dv">0</span>)</span>
<span id="cb1-12"><a href="#cb1-12"></a>            <span class="op">}</span></span>
<span id="cb1-13"><a href="#cb1-13"></a>        <span class="op">}</span>,</span>
<span id="cb1-14"><a href="#cb1-14"></a>        <span class="st">&quot;changelog&quot;</span> =&gt; <span class="pp">Msg::</span>ChangePage(<span class="dv">1</span>),</span>
<span id="cb1-15"><a href="#cb1-15"></a>        _ =&gt; <span class="pp">Msg::</span>ChangePage(<span class="dv">0</span>),</span>
<span id="cb1-16"><a href="#cb1-16"></a>    <span class="op">}</span></span>
<span id="cb1-17"><a href="#cb1-17"></a><span class="op">}</span></span>
<span id="cb1-18"><a href="#cb1-18"></a></span>
<span id="cb1-19"><a href="#cb1-19"></a><span class="at">#[</span>wasm_bindgen<span class="at">]</span></span>
<span id="cb1-20"><a href="#cb1-20"></a><span class="kw">pub</span> <span class="kw">fn</span> render() <span class="op">{</span></span>
<span id="cb1-21"><a href="#cb1-21"></a>    <span class="pp">seed::App::</span>build(<span class="pp">Model::</span><span class="kw">default</span>(), update, view)</span>
<span id="cb1-22"><a href="#cb1-22"></a>        .routes(routes)</span>
<span id="cb1-23"><a href="#cb1-23"></a>        .finish()</span>
<span id="cb1-24"><a href="#cb1-24"></a>        .run();</span>
<span id="cb1-25"><a href="#cb1-25"></a><span class="op">}</span></span></code></pre></div>
<p>The simplest way to trigger routing is to set up an element with an <code>At::Href</code> attribute, who's value contains a leading <code>/</code>, and corresponds to one of the routes defined in your <code>routes</code> function. Clicking this will trigger routing, as defined in <code>routes</code>:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb2-1"><a href="#cb2-1"></a><span class="pp">a!</span><span class="op">[</span><span class="st">&quot;Guide&quot;</span>, <span class="pp">attrs!</span><span class="op">{</span><span class="pp">At::</span>Href =&gt; <span class="st">&quot;/guide&quot;</span><span class="op">}</span> <span class="op">]</span></span>
<span id="cb2-2"><a href="#cb2-2"></a><span class="pp">a!</span><span class="op">[</span><span class="st">&quot;Guide page 1&quot;</span>, <span class="pp">attrs!</span><span class="op">{</span><span class="pp">At::</span>Href =&gt; <span class="st">&quot;/guide/1&quot;</span><span class="op">}</span> <span class="op">]</span></span></code></pre></div>
<p>The tag containing <code>Href</code> doesn't need to be an <code>a!</code> tag; any will work:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb3-1"><a href="#cb3-1"></a><span class="pp">button!</span><span class="op">[</span><span class="st">&quot;Changelog&quot;</span>, <span class="pp">attrs!</span><span class="op">{</span><span class="pp">At::</span>Href =&gt; <span class="st">&quot;/changelog&quot;</span><span class="op">}</span> <span class="op">]</span></span></code></pre></div>
<h2 id="more-detail-and-routing-using-events">More detail, and routing using events</h2>
<p>Your <code>routes</code> function outputs the message that handles the routing, and accepts a ref to a <a href="https://docs.rs/seed/0.3.7/seed/routing/struct.Url.html">Url struct</a> describing the route, which routes has the following fields:</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb4-1"><a href="#cb4-1"></a><span class="kw">pub</span> <span class="kw">struct</span> Url <span class="op">{</span></span>
<span id="cb4-2"><a href="#cb4-2"></a>    <span class="kw">pub</span> path: <span class="dt">Vec</span>&lt;<span class="dt">String</span>&gt;,</span>
<span id="cb4-3"><a href="#cb4-3"></a>    <span class="kw">pub</span> hash: <span class="dt">Option</span>&lt;<span class="dt">String</span>&gt;,</span>
<span id="cb4-4"><a href="#cb4-4"></a>    <span class="kw">pub</span> search: <span class="dt">Option</span>&lt;<span class="dt">String</span>&gt;,</span>
<span id="cb4-5"><a href="#cb4-5"></a>    <span class="kw">pub</span> title: <span class="dt">Option</span>&lt;<span class="dt">String</span>&gt;,</span>
<span id="cb4-6"><a href="#cb4-6"></a><span class="op">}</span></span></code></pre></div>
<p><code>path</code> contains the path heirarchy from top to bottom. For example, the <code>changelog</code> page above's path is <code>vec![String::from("changelog")]</code>, representing <code>/changelog/</code>, and guide page 3's is <code>vec![String::from("guide"), 3.to_string()]</code>, representing <code>/guide/3/</code>. It's likely all you'll need. The other three properties aren't as common; <code>hash</code> describes text after a <code>#</code>; <code>search</code> describes text after a <code>?</code>, but before <code>#</code>, and title is a descriptive title, unimplemented in current web browsers, but may see use in the future.</p>
<p>To trigger routing from events, instead of using <code>At::Href</code>, include logic like this in the <code>update</code> function:</p>
<div class="sourceCode" id="cb5"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb5-1"><a href="#cb5-1"></a><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></span>
<span id="cb5-2"><a href="#cb5-2"></a><span class="kw">enum</span> Msg <span class="op">{</span></span>
<span id="cb5-3"><a href="#cb5-3"></a>    RoutePage(<span class="dt">u32</span>),</span>
<span id="cb5-4"><a href="#cb5-4"></a>    RouteGuidePage(<span class="dt">u32</span>),</span>
<span id="cb5-5"><a href="#cb5-5"></a>    ChangePage(<span class="dt">u32</span>),</span>
<span id="cb5-6"><a href="#cb5-6"></a>    ChangeGuidePage(<span class="dt">u32</span>),</span>
<span id="cb5-7"><a href="#cb5-7"></a><span class="op">}</span></span>
<span id="cb5-8"><a href="#cb5-8"></a></span>
<span id="cb5-9"><a href="#cb5-9"></a><span class="kw">fn</span> set_guide_page(guide_page: Page, model: &amp;<span class="kw">mut</span> Model) <span class="op">{</span></span>
<span id="cb5-10"><a href="#cb5-10"></a>    model.page = <span class="pp">Page::</span>Guide;</span>
<span id="cb5-11"><a href="#cb5-11"></a>    model.guide_page = guide_page;</span>
<span id="cb5-12"><a href="#cb5-12"></a><span class="op">}</span></span>
<span id="cb5-13"><a href="#cb5-13"></a></span>
<span id="cb5-14"><a href="#cb5-14"></a><span class="kw">fn</span> update(msg: Msg, model: &amp;<span class="kw">mut</span> Model, orders: &amp;<span class="kw">mut</span> Orders&lt;Msg&gt;) <span class="op">{</span></span>
<span id="cb5-15"><a href="#cb5-15"></a>    <span class="kw">match</span> msg <span class="op">{</span></span>
<span id="cb5-16"><a href="#cb5-16"></a>        <span class="pp">Msg::</span>RoutePage(page) =&gt; <span class="op">{</span></span>
<span id="cb5-17"><a href="#cb5-17"></a>            <span class="pp">seed::</span>push_route(<span class="pp">vec!</span><span class="op">[</span>page<span class="op">]</span>);</span>
<span id="cb5-18"><a href="#cb5-18"></a>            orders.skip().send_msg(<span class="pp">Msg::</span>ChangePage(page))</span>
<span id="cb5-19"><a href="#cb5-19"></a>        <span class="op">}</span>,</span>
<span id="cb5-20"><a href="#cb5-20"></a>        <span class="pp">Msg::</span>RouteGuidePage(guide_page) =&gt; <span class="op">{</span></span>
<span id="cb5-21"><a href="#cb5-21"></a>            <span class="pp">seed::</span>push_route(<span class="pp">vec!</span><span class="op">[</span><span class="st">&quot;guide&quot;</span>, guide_page<span class="op">]</span>);</span>
<span id="cb5-22"><a href="#cb5-22"></a>            orders.skip().send_msg(<span class="pp">Msg::</span>ChangeGuidePage(guide_page))</span>
<span id="cb5-23"><a href="#cb5-23"></a>        <span class="op">}</span>,</span>
<span id="cb5-24"><a href="#cb5-24"></a>        <span class="co">// This is separate, because nagivating the route triggers state updates, which would</span></span>
<span id="cb5-25"><a href="#cb5-25"></a>        <span class="co">// trigger an additional push state.</span></span>
<span id="cb5-26"><a href="#cb5-26"></a>        <span class="pp">Msg::</span>ChangePage(page) =&gt; model.page = page</span>
<span id="cb5-27"><a href="#cb5-27"></a>        <span class="pp">Msg::</span>ChangeGuidePage(guide_page) =&gt; Render(Model <span class="op">{</span>guide_page, page: <span class="pp">Page::</span>Guide, ..model<span class="op">}</span>),</span>
<span id="cb5-28"><a href="#cb5-28"></a>        <span class="pp">Msg::</span>ChangeGuidePage(guide_page) =&gt; <span class="op">{</span></span>
<span id="cb5-29"><a href="#cb5-29"></a>            model.guide_page = page;</span>
<span id="cb5-30"><a href="#cb5-30"></a>            model.page = <span class="pp">Page::</span>Guide;</span>
<span id="cb5-31"><a href="#cb5-31"></a>        <span class="op">}</span></span>
<span id="cb5-32"><a href="#cb5-32"></a>    <span class="op">}</span></span>
<span id="cb5-33"><a href="#cb5-33"></a><span class="op">}</span></span></code></pre></div>
<p>Notice how the <code>Route</code> messages above call <a href="https://docs.rs/seed/0.3.7/seed/routing/fn.push_route.html">seed::push_route</a>, and the <code>Change</code> messages are called in the <code>routes</code> function, and are recursively called in the update function. <code>push_route</code> accepts a single parameter: a <code>Url</code> struct, which you can create with a struct literal, or <a href="https://docs.rs/seed/0.3.7/seed/routing/struct.Url.html#method.new">seed::Url::new</a>. Alternatively, you can pass a <code>Vec&lt;String&gt;</code> / <code>Vec&lt;&amp;str&gt;</code>, representing the path.</p>
<div class="sourceCode" id="cb6"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb6-1"><a href="#cb6-1"></a><span class="pp">seed::</span>push_route(</span>
<span id="cb6-2"><a href="#cb6-2"></a>    <span class="pp">seed::Url::</span>new(<span class="pp">vec!</span><span class="op">[</span><span class="st">&quot;myurl&quot;</span><span class="op">]</span>)</span>
<span id="cb6-3"><a href="#cb6-3"></a>        .hash(<span class="st">&quot;textafterhash&quot;</span>)</span>
<span id="cb6-4"><a href="#cb6-4"></a>        .search(<span class="st">&quot;textafterquestionmark&quot;</span>)</span>
<span id="cb6-5"><a href="#cb6-5"></a>)</span></code></pre></div>
<p>When a page is loaded or browser naviation occurs (eg back button), Seed uses the <code>routes</code> func you provided to determine which message to call.</p>
<p>Notice how we keep ChangePage and RoutePage separate in our example. Do not call <code>push_route</code> from one of these messages, or you'll end up with recusions/unwanted behavior: <code>ChangePage</code> in our example performs the action associated with routing, while <code>RoutePage</code> updates our route history, then recursively calls <code>ChangePage</code>. If you were to attempt this in the same message, each browser navigation event would add a redundant route history entry, interfering with navigation. `</p>
<p>We call routing messages from in-app navigation events, like this:</p>
<div class="sourceCode" id="cb7"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb7-1"><a href="#cb7-1"></a><span class="pp">h2!</span><span class="op">[</span> simple_ev(<span class="pp">Ev::</span>Click, <span class="pp">Msg::</span>RoutePage(<span class="dv">0</span>)), <span class="st">&quot;Guide&quot;</span> <span class="op">]</span></span></code></pre></div>
<p>Or programatically using lifecycle hooks:</p>
<div class="sourceCode" id="cb8"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb8-1"><a href="#cb8-1"></a>    did_mount(<span class="kw">move</span> |_| <span class="op">{</span></span>
<span id="cb8-2"><a href="#cb8-2"></a>        <span class="kw">if</span> model.logged_in <span class="op">{</span></span>
<span id="cb8-3"><a href="#cb8-3"></a>            state.update(<span class="pp">Msg::</span>RoutePage(<span class="dv">0</span>))</span>
<span id="cb8-4"><a href="#cb8-4"></a>        <span class="op">}</span></span>
<span id="cb8-5"><a href="#cb8-5"></a>    <span class="op">}</span>)</span></code></pre></div>
<p>To make landing-page routing work, configure your server so that all relevant paths towards the root or html file, instead of returning an error. The <code>serve.py</code> script included in the quickstart repo and examples is set up for this. Once this is configured, intial routing on page load will work as expected: The page will initialize with the default state, then immediately update based on the message returned by the <code>routes</code> function.</p>
"#.into()
}